use anyhow::{Context, Result};
use sqlx::{Pool, Sqlite, SqlitePool};
use std::sync::Arc;
use tokio::runtime::Runtime;

#[derive(Clone)]
pub struct BlockingDao {
    inner: Arc<BlockingInner>,
}

struct BlockingInner {
    dao: Dao,
    rt: Runtime,
}

impl BlockingDao {
    pub fn new(db: DB) -> Result<Self> {
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
}

#[derive(Clone)]
struct Dao {
    pool: Pool<Sqlite>,
}

pub struct TableSchema {
    pub cols: Vec<Column>,
}

#[derive(sqlx::FromRow)]
pub struct Column {
    cid: u32,
    name: String,
    #[sqlx(rename = "type")]
    typ: String,
    notnull: bool,
    dflt_value: String,
    pk: bool,
}

/// A row in the table
pub struct Record {}

pub struct Field {
    name: String,
}

pub struct Records {}

pub enum DB<'a> {
    Path(&'a str),
    Memory,
}

impl<'a> DB<'a> {
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

impl<'a> From<&'a dyn AsRef<str>> for DB<'a> {
    fn from(value: &'a dyn AsRef<str>) -> Self {
        Self::Path(value.as_ref())
    }
}

impl Dao {
    pub async fn new(db: DB<'_>) -> Result<Self> {
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

    async fn table_schema<P: AsRef<str>>(&self, table_name: P) -> Result<TableSchema> {
        let mut conn = self.pool.acquire().await?;
        let query = format!("pragma table_info({})", table_name.as_ref());
        let cols = sqlx::query_as::<_, Column>(&query)
            .fetch_all(&mut *conn)
            .await?;
        let schema = TableSchema { cols };
        Ok(schema)
    }

    async fn records<P: AsRef<str>>(&self, table_name: P, schema: &TableSchema) -> Result<Records> {
        todo!()
    }
}

#[cfg(test)]
mod tests {}
