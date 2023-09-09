use anyhow::Result;
use sqlx::{Pool, Sqlite};

pub struct Dao {
    pool: Pool<Sqlite>,
}

impl Dao {
    pub async fn new() -> Result<Self> {
        let pool = Pool::connect("sqlite://:memory:").await?;
        Ok(Self { pool })
    }

    pub async fn get_users(&self) -> Result<Vec<User>> {
        let users = sqlx::query_as::<_, User>("SELECT * FROM users")
            .fetch_all(&self.pool)
            .await?;
        Ok(users)
    }
}

#[derive(sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub age: i32,
}
