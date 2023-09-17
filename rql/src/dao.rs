#![allow(dead_code, unused)]

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
    pub fn new<P: AsRef<str>>(path: P) -> Result<Self> {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()?;
        let dao = rt.block_on(Dao::new(path))?;
        let inner = BlockingInner { dao, rt };
        let inner = inner.into();
        Ok(Self { inner })
    }

    pub fn tables(&self) -> Result<Vec<String>> {
        self.inner.rt.block_on(self.inner.dao.tables())
    }

    async fn table_schema<P: AsRef<str>>(&self, table_name: P) -> Result<TableSchema> {
        self.inner
            .rt
            .block_on(self.inner.dao.table_schema(table_name))
    }
}

#[derive(Clone)]
struct Dao {
    pool: Pool<Sqlite>,
}

#[derive(sqlx::FromRow)]
struct TableSchema {
    cid: u32,
    name: String,
    typ: String,
    notnull: bool,
    dflt_value: String,
    pk: bool,
}

impl Dao {
    pub async fn new<P: AsRef<str>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let pool = SqlitePool::connect(path)
            .await
            .context(format!(r#"could not open "{path}""#))?;
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
        let res = sqlx::query_as::<_, TableSchema>("pragma table_info(?)")
            .bind(table_name.as_ref())
            .fetch_one(&mut *conn)
            .await?;
        todo!();
    }
}
