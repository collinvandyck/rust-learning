use anyhow::{Context, Result};
use sqlx::{pool::PoolOptions, Pool, Sqlite};

#[tokio::main]
async fn main() -> Result<()> {
    let bs = [
        MigrateBehavior::Shared,
        MigrateBehavior::Detach,
        MigrateBehavior::Leak,
    ];
    for b in bs {
        if let Err(e) = test_migrate_conn(b.clone()).await {
            println!("{b:?} conn test failed: {e}");
        }
    }
    Ok(())
}

#[derive(Debug, Clone)]
enum MigrateBehavior {
    Shared,
    Detach,
    Leak,
}

async fn migrate(pool: &mut Pool<Sqlite>, behavior: MigrateBehavior) -> Result<()> {
    let mut conn = pool.acquire().await.context("acquire failed")?;
    match behavior {
        MigrateBehavior::Shared => {
            sqlx::migrate!("./migrations").run(&mut conn).await?;
        }
        MigrateBehavior::Detach => {
            let mut conn = conn.detach();
            sqlx::migrate!("./migrations").run_direct(&mut conn).await?;
        }
        MigrateBehavior::Leak => {
            let mut conn = conn.leak();
            sqlx::migrate!("./migrations").run_direct(&mut conn).await?;
        }
    }
    Ok(())
}

async fn new_pool() -> Result<Pool<Sqlite>> {
    let pool = PoolOptions::new()
        .min_connections(1)
        .max_connections(100)
        .connect("sqlite::memory:")
        .await?;
    Ok(pool)
}

async fn test_migrate_conn(b: MigrateBehavior) -> Result<()> {
    let mut pool = new_pool().await?;
    migrate(&mut pool, b).await?;

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
