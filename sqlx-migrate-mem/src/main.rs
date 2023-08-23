use anyhow::{Context, Result};
use sqlx::{pool::PoolOptions, Pool, Sqlite};

#[tokio::main]
async fn main() -> Result<()> {
    if let Err(e) = test_migrate_shared_conn().await {
        println!("Shared conn test failed: {e}");
    }
    if let Err(e) = test_migrate_conn().await {
        println!("Detached conn test failed: {e}");
    }
    Ok(())
}

async fn migrate(pool: &mut Pool<Sqlite>, detach: bool) -> Result<()> {
    let mut conn = pool.acquire().await.context("acquire failed")?;
    if detach {
        let mut conn = conn.detach();
        sqlx::migrate!("./migrations").run_direct(&mut conn).await?;
    } else {
        sqlx::migrate!("./migrations").run(&mut conn).await?;
    }
    Ok(())
}

async fn test_migrate_conn() -> Result<()> {
    let mut pool: Pool<Sqlite> = PoolOptions::new()
        .max_connections(100)
        .connect("sqlite::memory:")
        .await?;
    migrate(&mut pool, true).await?;

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
    let mut pool: Pool<Sqlite> = PoolOptions::new()
        .max_connections(100)
        .connect("sqlite::memory:")
        .await?;

    migrate(&mut pool, false).await?;

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
