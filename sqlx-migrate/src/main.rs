use anyhow::Result;
use sqlx::sqlite::SqlitePoolOptions;

#[tokio::main]
async fn main() -> Result<()> {
    let pool = SqlitePoolOptions::new().connect("test.db").await?;
    Ok(())
}
