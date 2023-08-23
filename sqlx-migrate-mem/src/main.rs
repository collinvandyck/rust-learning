use anyhow::Result;
use sqlx::{pool::PoolOptions, Pool, Sqlite};

#[tokio::main]
async fn main() -> Result<()> {
    let pool: Pool<Sqlite> = PoolOptions::new()
        .max_connections(100)
        .connect("sqlite::memory:")
        .await?;
    let mut conn = pool.acquire().await?;
    sqlx::migrate!("./migrations").run(&mut conn).await?;
    Ok(())
}
