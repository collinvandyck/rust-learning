use anyhow::Result;
use sqlx::sqlite::SqlitePoolOptions;

#[tokio::main]
async fn main() -> Result<()> {
    run().await?;
    Ok(())
}

async fn run() -> Result<()> {
    let _pool = SqlitePoolOptions::new().connect("sqlite:test.db").await?;
    Ok(())
}
