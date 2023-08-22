#![allow(dead_code, unused)]
use anyhow::Result;
use sqlx::{
    migrate::{Migrate, MigrateDatabase},
    sqlite::SqlitePoolOptions,
    Pool, Sqlite,
};

#[tokio::main]
async fn main() -> Result<()> {
    let db_url = "sqlite://test.db";
    create(db_url).await?;
    let mut pool = connect(db_url).await?;
    migrate(&mut pool).await?;
    Ok(())
}

async fn create(db_url: &str) -> Result<()> {
    if Sqlite::database_exists(db_url).await? {
        return Ok(());
    }
    Sqlite::create_database(db_url).await?;
    Ok(())
}

async fn migrate(pool: &mut Pool<Sqlite>) -> Result<()> {
    let mut conn = pool.acquire().await?;
    conn.ensure_migrations_table().await?;
    println!("Running migrations");
    sqlx::migrate!("db/migrations").run(&mut conn).await?;
    println!("Current migrations:");
    let migrations = conn.list_applied_migrations().await?;
    for migration in migrations {
        println!("\t{}", migration.version);
    }
    sqlx::migrate!("db/migrations").run(&mut conn).await?;

    Ok(())
}

async fn connect(db_url: &str) -> Result<Pool<Sqlite>> {
    let pool = SqlitePoolOptions::new().connect("sqlite://test.db").await?;
    Ok(pool)
}
