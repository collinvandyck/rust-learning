#![allow(dead_code, unused)]

use anyhow::Result;
use sqlx::{Pool, Sqlite, SqlitePool};
use tokio::runtime::Runtime;

#[derive(Clone)]
struct Dao {
    pool: Pool<Sqlite>,
}

pub struct BlockingDao {
    dao: Dao,
    rt: Runtime,
}

impl BlockingDao {
    pub fn new<P: AsRef<str>>(path: P) -> Result<Self> {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()?;
        let dao = rt.block_on(Dao::new(path))?;
        Ok(Self { dao, rt })
    }

    pub fn tables(&mut self) -> Result<Vec<String>> {
        self.rt.block_on(self.dao.tables())
    }

    async fn table_schema<P: AsRef<str>>(&mut self, table_name: P) -> Result<TableSchema> {
        self.rt.block_on(self.dao.table_schema(table_name))
    }
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
        let pool = SqlitePool::connect(path.as_ref()).await?;
        Ok(Self { pool })
    }

    async fn tables(&mut self) -> Result<Vec<String>> {
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

    async fn table_schema<P: AsRef<str>>(&mut self, table_name: P) -> Result<TableSchema> {
        let mut conn = self.pool.acquire().await?;
        let res = sqlx::query_as::<_, TableSchema>("pragma table_info(?)")
            .bind(table_name.as_ref())
            .fetch_one(&mut *conn)
            .await?;
        todo!();
    }
}
