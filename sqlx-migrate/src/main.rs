use anyhow::Result;
use sqlx::sqlite::SqlitePoolOptions;

#[tokio::main]
async fn main() -> Result<()> {
    create().await?;
    run().await?;
    Ok(())
}

async fn create() -> Result<()> {
    Ok(())
}

async fn run() -> Result<()> {
    let _pool = SqlitePoolOptions::new().connect("sqlite:test.db").await?;
    Ok(())
}
