#![allow(dead_code)]

use anyhow::Result;
use mockall::automock;
use sqlx::{Pool, Sqlite};

pub struct DB {
    pool: Pool<Sqlite>,
}

#[automock]
impl DB {
    pub async fn new_dao() -> Result<Self> {
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

#[tokio::test]
async fn test_dao() -> Result<()> {
    let dao = mock_db().await;
    let users = dao.get_users().await?;
    assert_eq!(users.len(), 1);
    Ok(())
}

async fn mock_db() -> MockDB {
    let mut db = MockDB::new();
    db.expect_get_users().returning(|| {
        Ok(vec![User {
            id: 1,
            name: "test".to_string(),
            age: 20,
        }])
    });
    db
}
