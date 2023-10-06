use anyhow::{Context, Result};
use sqlx::{sqlite::SqliteRow, Column, Pool, Row, Sqlite, SqlitePool};
use std::{fmt::Display, ops::Deref, sync::Arc, time::Instant};
use tokio::runtime::Runtime;
use tracing::{debug, info, warn};

#[derive(Clone)]
pub struct BlockingDao {
    inner: Arc<BlockingInner>,
}

struct BlockingInner {
    dao: Dao,
    rt: Runtime,
}

impl BlockingDao {
    pub fn new(db: DbType) -> Result<Self> {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()?;
        let dao = rt.block_on(Dao::new(db))?;
        let inner = BlockingInner { dao, rt };
        let inner = inner.into();
        Ok(Self { inner })
    }

    pub fn tables(&self) -> Result<Vec<String>> {
        self.inner.rt.block_on(self.inner.dao.tables())
    }

    pub fn table_schema<P: AsRef<str>>(&self, table_name: P) -> Result<TableSchema> {
        self.inner
            .rt
            .block_on(self.inner.dao.table_schema(table_name))
    }

    pub fn records(&self, schema: &TableSchema, req: GetRecords) -> Result<Vec<Record>> {
        self.inner.rt.block_on(self.inner.dao.records(schema, req))
    }

    pub fn count(&self, req: Count) -> Result<u64> {
        self.inner.rt.block_on(self.inner.dao.count(req))
    }

    pub fn max_lens(&self, schema: &TableSchema, req: MaxLens) -> Result<Vec<usize>> {
        self.inner.rt.block_on(self.inner.dao.max_lens(schema, req))
    }
}

#[derive(Clone)]
struct Dao {
    pool: Pool<Sqlite>,
}

#[derive(Debug, Clone)]
pub struct TableSchema {
    pub name: String,
    pub cols: Vec<TableColumn>,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum TableColumn {
    RowId,
    Spec(TableColumnSpec),
}

impl TableColumn {
    pub fn name<'a>(&'a self) -> &'a str {
        match self {
            TableColumn::RowId => "rowid",
            TableColumn::Spec(spec) => &spec.name,
        }
    }
    pub fn field_type(&self) -> FieldType {
        match self {
            TableColumn::RowId => FieldType::RowId,
            TableColumn::Spec(spec) => FieldType::from(spec.typ.as_ref()),
        }
    }
}

#[derive(sqlx::FromRow, Hash, PartialEq, Eq, Clone, Debug)]
#[allow(dead_code)]
pub struct TableColumnSpec {
    pub name: String,
    #[sqlx(rename = "type")]
    typ: String,
    cid: u32,
    notnull: bool,
    dflt_value: String,
    pk: bool,
}

/// A row in the table
#[derive(Default, Debug, PartialEq, Clone)]
pub struct Record {
    pub fields: Vec<Field>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Field {
    pub name: String,
    pub typ: FieldType,
    pub val: FieldValue,
}

#[derive(Debug, PartialEq, Clone)]
pub enum FieldValue {
    RowID(i64),
    Null,
    Text(Option<String>),
    Real(Option<f64>),
    Blob(Option<Vec<u8>>),
    Integer(Option<i64>),
    Numeric(Option<f64>),
    Boolean(Option<bool>),
    Date(Option<Instant>),
    Time(Option<Instant>),
    DateTime(Option<Instant>),
}

impl FieldValue {
    pub fn len(&self) -> usize {
        use FieldValue::*;
        match self {
            RowID(val) => count_digits(*val),
            Text(Some(s)) => s.len(),
            Null => 4,
            _ => 10,
        }
    }
}

fn count_digits(v: i64) -> usize {
    let mut v_copy = v;
    let mut res = 0;
    while v_copy > 0 {
        v_copy = v_copy / 10;
        res += 1;
    }
    res
}

impl Display for FieldValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use FieldValue::*;
        match self {
            RowID(val) => write!(f, "{val}"),
            Text(Some(val)) => write!(f, "{val}"),
            Real(Some(val)) => write!(f, "{val}"),
            Blob(Some(val)) => write!(f, "bytes {{ len = {}}}", val.len()),
            Integer(Some(val)) => write!(f, "{val}"),
            Numeric(Some(val)) => write!(f, "{val}"),
            Boolean(Some(val)) => write!(f, "{val}"),
            Date(Some(val)) => write!(f, "{val:?}"),
            Time(Some(val)) => write!(f, "{val:?}"),
            DateTime(Some(val)) => write!(f, "{val:?}"),
            _ => Ok(()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FieldType {
    RowId,
    Null,
    Text,
    Real,
    Blob,
    Integer,
    Numeric,
    Boolean,
    Date,
    Time,
    DateTime,
}

impl FieldType {
    fn decode(&self, row: &SqliteRow, idx: usize) -> Result<FieldValue> {
        let val = match self {
            FieldType::Null => FieldValue::Null,
            FieldType::RowId => FieldValue::RowID(self.decode_i64(row, idx)?.unwrap()),
            FieldType::Text => FieldValue::Text(self.decode_string(row, idx)?),
            FieldType::Real => FieldValue::Real(self.decode_f64(row, idx)?),
            FieldType::Blob => FieldValue::Blob(self.decode_bytes(row, idx)?),
            FieldType::Integer => FieldValue::Integer(self.decode_i64(row, idx)?),
            FieldType::Numeric => FieldValue::Numeric(self.decode_f64(row, idx)?),
            FieldType::Boolean => FieldValue::Boolean(self.decode_bool(row, idx)?),
            FieldType::Date => FieldValue::Date(self.decode_instant(row, idx)?),
            FieldType::Time => FieldValue::Time(self.decode_instant(row, idx)?),
            FieldType::DateTime => FieldValue::DateTime(self.decode_instant(row, idx)?),
        };
        Ok(val)
    }

    fn decode_instant(&self, row: &SqliteRow, idx: usize) -> Result<Option<Instant>> {
        warn!("decode instant not implemented yet");
        Ok(None)
    }

    fn decode_bool(&self, row: &SqliteRow, idx: usize) -> Result<Option<bool>> {
        Ok(row.try_get::<Option<bool>, _>(idx)?)
    }

    fn decode_i64(&self, row: &SqliteRow, idx: usize) -> Result<Option<i64>> {
        Ok(row.try_get::<Option<i64>, _>(idx)?)
    }

    fn decode_bytes(&self, row: &SqliteRow, idx: usize) -> Result<Option<Vec<u8>>> {
        Ok(row.try_get::<Option<Vec<u8>>, _>(idx)?)
    }

    fn decode_f64(&self, row: &SqliteRow, idx: usize) -> Result<Option<f64>> {
        Ok(row.try_get::<Option<f64>, _>(idx)?)
    }

    fn decode_string(&self, row: &SqliteRow, idx: usize) -> Result<Option<String>> {
        Ok(row.try_get::<Option<String>, _>(idx)?)
    }
}

impl From<&str> for FieldType {
    fn from(value: &str) -> Self {
        let value = value.to_lowercase();
        let value = value.as_str();
        if value.starts_with("varchar") {
            return FieldType::Text;
        }
        match value {
            "string" | "text" | "timestamp" => FieldType::Text,
            "int" | "integer" | "bigint" | "uint64" | "numeric" => FieldType::Integer,
            "float" => FieldType::Real,
            "blob" => FieldType::Blob,
            "boolean" | "bool" => FieldType::Boolean,
            "datetime" => FieldType::DateTime,
            "date" => FieldType::Date,
            "time" => FieldType::Time,
            _ => panic!("unknown type: {}", value),
        }
    }
}

pub enum DbType<'a> {
    Path(&'a str),
    Memory,
}

#[derive(Default, Debug)]
pub struct GetRecords {
    pub table_name: String,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub query: Option<String>,
}

impl GetRecords {
    pub fn new<S: Into<String>>(table_name: S) -> Self {
        GetRecords {
            table_name: table_name.into(),
            ..Default::default()
        }
    }
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }
    pub fn search(mut self, query: &str) -> Self {
        self.query = Some(query.to_string());
        self
    }
    pub fn maybe_search(mut self, query: &Option<String>) -> Self {
        self.query = query.to_owned();
        self
    }
}

#[derive(Default, Debug)]
pub struct Count {
    pub table_name: String,
    pub query: Option<String>,
}

impl Count {
    pub fn new<S: Into<String>>(table_name: S) -> Self {
        Self {
            table_name: table_name.into(),
            ..Default::default()
        }
    }
    pub fn search(mut self, query: &str) -> Self {
        self.query = Some(query.to_string());
        self
    }
    pub fn maybe_search(mut self, query: &Option<String>) -> Self {
        self.query = query.to_owned();
        self
    }
}

#[derive(Default, Debug)]
pub struct MaxLens {
    pub table_name: String,
    pub query: Option<String>,
}

impl MaxLens {
    pub fn new<S: Into<String>>(table_name: S) -> Self {
        Self {
            table_name: table_name.into(),
            ..Default::default()
        }
    }
    pub fn search(mut self, query: &str) -> Self {
        self.query = Some(query.to_string());
        self
    }
    pub fn maybe_search(mut self, query: &Option<String>) -> Self {
        self.query = query.to_owned();
        self
    }
}

impl<'a> DbType<'a> {
    async fn connect(&'a self) -> Result<Pool<Sqlite>> {
        match self {
            Self::Path(path) => {
                let path = path.as_ref();
                SqlitePool::connect(path)
                    .await
                    .context(format!(r#"could not open "{path}""#))
            }
            Self::Memory => SqlitePool::connect(":memory:")
                .await
                .context("could not connect to memory db"),
        }
    }
}

impl Dao {
    pub async fn new(db: DbType<'_>) -> Result<Self> {
        let pool = db.connect().await?;
        Ok(Self { pool })
    }

    async fn tables(&self) -> Result<Vec<String>> {
        #[derive(sqlx::FromRow)]
        struct Record {
            name: String,
        }
        let mut conn = self.pool.acquire().await?;
        let res = sqlx::query_as::<_, Record>(
            "select name from sqlite_schema where type='table' order by name",
        )
        .fetch_all(&mut *conn)
        .await?
        .into_iter()
        .map(|s| s.name)
        .collect();
        Ok(res)
    }

    async fn table_schema<P: AsRef<str>>(&self, name: P) -> Result<TableSchema> {
        let name = name.as_ref().to_string();
        info!(name, "Getting table schema");
        let mut conn = self.pool.acquire().await?;
        let query = format!("pragma table_info({})", &name);
        let mut cols = sqlx::query_as::<_, TableColumnSpec>(&query)
            .fetch_all(&mut *conn)
            .await?
            .into_iter()
            .map(|c| TableColumn::Spec(c))
            .collect::<Vec<_>>();
        cols.insert(0, TableColumn::RowId);
        let schema = TableSchema { name, cols };
        Ok(schema)
    }

    async fn max_lens(&self, schema: &TableSchema, req: MaxLens) -> Result<Vec<usize>> {
        let mut conn = self.pool.acquire().await?;
        let query_parts = &schema
            .cols
            .iter()
            .map(|c| format!("max(length({}))", c.name()))
            .collect::<Vec<_>>()
            .join(",");
        let mut query = format!("select {} from {}", query_parts, schema.name);
        let where_clause = req
            .query
            .as_ref()
            .and_then(|q| build_where_clause(&schema, q));
        if let Some(wher) = where_clause {
            query.push_str(&format!(" WHERE {}", wher));
        }
        debug!("max lens query: {}", query);
        let row = sqlx::query(&query).fetch_one(&mut *conn).await?;
        let mut res = vec![];
        for (idx, col) in schema.cols.iter().enumerate() {
            let len = row.get::<i64, _>(idx);
            res.push(len.try_into().unwrap_or_default());
        }
        Ok(res)
    }

    async fn count(&self, req: Count) -> Result<u64> {
        #[derive(sqlx::FromRow)]
        struct Record {
            count: i64,
        }
        let schema = self.table_schema(&req.table_name).await?;
        let mut conn = self.pool.acquire().await?;
        let mut query = format!("select count(*) as count from {}", &req.table_name);
        let where_clause = req
            .query
            .as_ref()
            .and_then(|q| build_where_clause(&schema, q));
        if let Some(wher) = where_clause {
            query.push_str(&format!(" WHERE {}", wher));
        }

        debug!("count query: {}", query);
        let record = sqlx::query_as::<_, Record>(&query)
            .fetch_one(&mut *conn)
            .await?;
        Ok(record.count as u64)
    }

    async fn records(&self, schema: &TableSchema, req: GetRecords) -> Result<Vec<Record>> {
        let table_name = req.table_name.as_str();
        let schema = self.table_schema(table_name).await?;
        let mut conn = self.pool.acquire().await?;
        let limit = req.limit.map(|v| format!("limit {v}")).unwrap_or_default();
        let offset = req
            .offset
            .map(|v| format!("offset {v}"))
            .unwrap_or_default();
        let where_clause = req
            .query
            .as_ref()
            .and_then(|q| build_where_clause(&schema, q));

        let query = if let Some(wher) = where_clause {
            format!(
                "select rowid, * from {} WHERE {} {} {}",
                table_name, wher, limit, offset
            )
        } else {
            format!("select rowid, * from {} {} {}", table_name, limit, offset)
        };
        debug!(query, "records query: {}", query);

        let rows = sqlx::query(&query).fetch_all(&mut *conn).await?;
        let mut records = vec![];
        for row in rows {
            let mut record = Record::default();
            for column in row.columns() {
                let name = column.name().to_string();
                let ord = column.ordinal();
                let col = &schema.cols[ord];
                let typ = col.field_type();
                let val = typ.decode(&row, ord)?;
                let field = Field { name, typ, val };
                record.fields.push(field);
            }
            records.push(record);
        }
        Ok(records)
    }

    #[cfg(test)]
    async fn execute(&self, sql: &'static str) -> Result<()> {
        let mut conn = self.pool.acquire().await?;
        sqlx::query(sql).execute(&mut *conn).await?;
        Ok(())
    }
}

// returns a where clause against all string columns, OR'd,
// with the provided query. None is returned if no columns
// can be queried against.
//
// A better query would allow per-column constraints and be
// SQL type aware.
fn build_where_clause(schema: &TableSchema, query: &str) -> Option<String> {
    let numeric = query.parse::<f64>().is_ok();
    let constraints: Vec<String> = schema
        .cols
        .iter()
        .filter_map(|c| match c {
            TableColumn::RowId if numeric => Some(format!("rowid = {}", query)),
            TableColumn::Spec(spec)
                if spec.typ.to_lowercase() == "text" || spec.typ.to_lowercase() == "string" =>
            {
                Some(format!("{} LIKE '%{}%'", spec.name, query))
            }
            TableColumn::Spec(spec)
                if numeric
                    && (spec.typ.to_lowercase() == "numeric"
                        || spec.typ.to_lowercase() == "float"
                        || spec.typ.to_lowercase() == "double"
                        || spec.typ.to_lowercase() == "integer") =>
            {
                Some(format!("{} = {}", spec.name, query))
            }
            _ => return None,
        })
        .collect();

    if constraints.len() > 0 {
        Some(constraints.join(" OR "))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_test::traced_test;

    #[tokio::test]
    #[traced_test]
    async fn test_decode() -> Result<()> {
        let dao = Dao::new(DbType::Memory).await?;
        dao.execute("create table foo (name string, age integer)")
            .await?;
        let schema = dao.table_schema("foo").await?;
        let records = dao
            .records(&schema, GetRecords::new("foo").limit(100))
            .await?;
        assert_eq!(records.len(), 0);
        dao.execute("insert into foo (name, age) values ('collin', 46)")
            .await?;
        let records = dao
            .records(&schema, GetRecords::new("foo").limit(100))
            .await?;
        assert_eq!(records.len(), 1);
        assert_eq!(
            records[0]
                .fields
                .iter()
                .map(|f| f.val.clone())
                .collect::<Vec<_>>(),
            vec![
                FieldValue::RowID(1),
                FieldValue::Text(Some("collin".to_string())),
                FieldValue::Integer(Some(46)),
            ]
        );
        Ok(())
    }

    #[tokio::test]
    #[traced_test]
    async fn test_where_clause() -> Result<()> {
        let dao = Dao::new(DbType::Memory).await?;
        dao.execute("create table foo (name TEXT, age INTEGER)")
            .await?;
        let schema = dao.table_schema("foo").await?;

        let clause = build_where_clause(&schema, "collin");
        assert_eq!(Some("name LIKE '%collin%'"), clause.as_deref());

        let clause = build_where_clause(&schema, "1").unwrap();
        assert!(clause.contains("name LIKE '%1%'"));
        assert!(clause.contains("rowid = 1"));
        assert!(clause.contains("age = 1"));
        assert_eq!(3, clause.split(" OR ").collect::<Vec<_>>().len());

        let clause = build_where_clause(&schema, "1").unwrap();

        // no where clause
        let dao = Dao::new(DbType::Memory).await?;
        dao.execute("create table foo (buzz INTEGER)").await?;
        let schema = dao.table_schema("foo").await?;

        let clause = build_where_clause(&schema, "text search");
        assert!(clause.is_none());

        Ok(())
    }

    #[tokio::test]
    #[traced_test]
    async fn test_search() -> Result<()> {
        let dao = Dao::new(DbType::Memory).await?;
        dao.execute("create table foo (name TEXT, age INTEGER)")
            .await?;
        let schema = dao.table_schema("foo").await?;

        dao.execute("insert into foo (name, age) values ('collin', 46)")
            .await?;

        let matches = dao
            .records(&schema, GetRecords::new("foo").limit(100).search("1000"))
            .await?;
        assert_eq!(0, matches.len());
        let matches = dao
            .records(&schema, GetRecords::new("foo").limit(100).search("nomatch"))
            .await?;
        assert_eq!(0, matches.len());

        let matches = dao
            .records(&schema, GetRecords::new("foo").limit(100).search("collin"))
            .await?;
        assert_eq!(1, matches.len());
        assert_eq!(
            FieldValue::Text(Some("collin".to_string())),
            matches[0]
                .fields
                .iter()
                .find(|f| f.name == "name")
                .unwrap()
                .val
        );

        Ok(())
    }
}
