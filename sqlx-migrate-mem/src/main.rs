use anyhow::Result;
use sqlx::{pool::PoolOptions, Pool, Sqlite};

#[tokio::main]
async fn main() -> Result<()> {
    test_migrate_shared_conn().await?;
    test_migrate_conn().await?;
    Ok(())
}

async fn test_migrate_conn() -> Result<()> {
    let pool: Pool<Sqlite> = PoolOptions::new()
        .max_connections(100)
        .connect("sqlite::memory:")
        .await?;
    let mut conn = pool.acquire().await?;
    sqlx::migrate!("./migrations").run(&mut conn).await?;

    #[derive(PartialEq, Eq, Debug, sqlx::FromRow)]
    struct Record(String);

    let mut conn = pool.acquire().await?;
    let records = sqlx::query_as::<_, Record>("select * from foo")
        .fetch_all(&mut conn)
        .await?;

    assert_eq!(records.len(), 1);
    assert_eq!(records.first(), Some(&Record("collin".to_string())));
    Ok(())
}

async fn test_migrate_shared_conn() -> Result<()> {
    let pool: Pool<Sqlite> = PoolOptions::new()
        .max_connections(100)
        .connect("sqlite::memory:")
        .await?;
    let mut conn = pool.acquire().await?;
    sqlx::migrate!("./migrations").run(&mut conn).await?;

    #[derive(PartialEq, Eq, Debug, sqlx::FromRow)]
    struct Record(String);

    let mut conn = pool.acquire().await?;
    let records = sqlx::query_as::<_, Record>("select * from foo")
        .fetch_all(&mut conn)
        .await?;

    assert_eq!(records.len(), 1);
    assert_eq!(records.first(), Some(&Record("collin".to_string())));
    Ok(())
}
