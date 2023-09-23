use anyhow::{Context, Result};
use sqlx::{sqlite::SqliteRow, Column, Pool, Row, Sqlite, SqlitePool};
use std::{fmt::Display, ops::Deref, sync::Arc, time::Instant};
use tokio::runtime::Runtime;
use tracing::{info, warn};

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

    pub fn records(&self, schema: &TableSchema, req: GetRecords) -> Result<Records> {
        self.inner.rt.block_on(self.inner.dao.records(schema, req))
    }

    pub fn count<P: AsRef<str>>(&self, table_name: P) -> Result<u64> {
        self.inner.rt.block_on(self.inner.dao.count(table_name))
    }
}

#[derive(Clone)]
struct Dao {
    pool: Pool<Sqlite>,
}

pub struct TableSchema {
    pub name: String,
    pub cols: Vec<TableColumn>,
}

#[derive(sqlx::FromRow)]
#[allow(dead_code)]
pub struct TableColumn {
    cid: u32,
    pub name: String,
    #[sqlx(rename = "type")]
    typ: String,
    notnull: bool,
    dflt_value: String,
    pk: bool,
}

/// A row in the table
#[derive(Default)]
pub struct Record {
    pub fields: Vec<Field>,
}

pub struct Field {
    pub name: String,
    pub typ: FieldType,
    pub val: FieldValue,
}

#[derive(Debug, PartialEq)]
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

impl Display for FieldValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use FieldValue::*;
        match self {
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

#[derive(Debug)]
pub enum FieldType {
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
            "int" | "integer" | "bigint" | "uint64" => FieldType::Integer,
            "blob" => FieldType::Blob,
            "boolean" | "bool" => FieldType::Boolean,
            "datetime" => FieldType::DateTime,
            "date" => FieldType::Date,
            "time" => FieldType::Time,
            _ => panic!("unknown type: {}", value),
        }
    }
}

#[derive(Default)]
pub struct Records {
    records: Vec<Record>,
}

impl Deref for Records {
    type Target = Vec<Record>;
    fn deref(&self) -> &Self::Target {
        &self.records
    }
}

pub enum DbType<'a> {
    Path(&'a str),
    Memory,
}

pub struct GetRecords {
    pub table_name: String,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

impl GetRecords {
    pub fn new<S: Into<String>>(table_name: S) -> Self {
        GetRecords {
            table_name: table_name.into(),
            limit: None,
            offset: None,
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
        let cols = sqlx::query_as::<_, TableColumn>(&query)
            .fetch_all(&mut *conn)
            .await?;
        let schema = TableSchema { name, cols };
        Ok(schema)
    }

    async fn count<P: AsRef<str>>(&self, table_name: P) -> Result<u64> {
        #[derive(sqlx::FromRow)]
        struct Record {
            count: i64,
        }
        let mut conn = self.pool.acquire().await?;
        let query = format!("select count(*) as count from {}", table_name.as_ref());
        let record = sqlx::query_as::<_, Record>(&query)
            .fetch_one(&mut *conn)
            .await?;
        Ok(record.count as u64)
    }

    async fn records(&self, schema: &TableSchema, req: GetRecords) -> Result<Records> {
        let table_name = req.table_name.as_str();
        info!(table_name, "Fetching records");
        let mut conn = self.pool.acquire().await?;
        let limit = req.limit.map(|v| v.to_string()).unwrap_or_default();
        let offset = req.offset.map(|v| v.to_string()).unwrap_or_default();
        let query = format!("select rowid, * from {} {} {}", table_name, limit, offset);
        let rows = sqlx::query(&query).fetch_all(&mut *conn).await?;
        let mut records = Records::default();
        for row in rows {
            let mut record = Record::default();
            for (cidx, column) in row.columns().into_iter().enumerate() {
                if cidx == 0 {
                    // rowid
                    let name = String::from("rid");
                    let typ = FieldType::from("integer");
                    let val = row
                        .try_get::<Option<i64>, _>(0_usize)?
                        .expect("could not get rowid");
                    let val = FieldValue::RowID(val);
                    let field = Field { name, typ, val };
                    record.fields.push(field);
                } else {
                    let name = column.name().to_string();
                    let ord = column.ordinal();
                    let col = &schema.cols[ord - 1]; // account for rowid
                    let typ = FieldType::from(col.typ.as_ref());
                    let val = typ.decode(&row, ord)?;
                    let field = Field { name, typ, val };
                    record.fields.push(field);
                }
            }
            records.records.push(record);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
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
        assert_eq!(records[0].fields.len(), 2);
        assert_eq!(
            records[0].fields[0].val,
            FieldValue::Text(Some("collin".to_string()))
        );
        assert_eq!(records[0].fields[1].val, FieldValue::Integer(Some(46)),);
        Ok(())
    }
}
