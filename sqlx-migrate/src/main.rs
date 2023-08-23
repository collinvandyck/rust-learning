#![allow(dead_code, unused)]
use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use sqlx::{
    migrate::{Migrate, MigrateDatabase},
    sqlite::SqlitePoolOptions,
    Pool, Sqlite,
};
use tokio::sync::{
    mpsc::{self, Sender},
    oneshot, Mutex,
};

#[tokio::main]
async fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel(1);
    tokio::spawn(async move { migrate_task(rx) });
    let db_url = "sqlite://test.db";
    create(db_url).await?;
    let mut pool = connect(db_url).await?;
    migrate(&mut pool).await?;
    Ok(())
}

async fn migrate_task(mut rx: mpsc::Receiver<Request>) {
    while let Some(mut req) = rx.recv().await {
        match req {
            Request::Migrate(tx, ref mut pool) => {
                let mut pool = pool.lock().await;
                let res = migrate(&mut pool).await;
                let _ = tx.send(match res {
                    Ok(_) => Response::Success,
                    Err(e) => Response::Error(e.to_string()),
                });
            }
        }
    }
}

enum Request {
    Migrate(oneshot::Sender<Response>, Arc<Mutex<Pool<Sqlite>>>),
}

enum Response {
    Success,
    Error(String),
}

struct DefaultMigrator {
    tx: Arc<Sender<Request>>,
}

#[async_trait]
trait Migrator {
    async fn migrate(&self) -> Result<()>;
}

#[async_trait]
impl Migrator for DefaultMigrator {
    async fn migrate(&self) -> Result<()> {
        let db_url = "sqlite://test.db";
        create(db_url).await?;
        let mut pool = connect(db_url).await?;
        let (tx, rx) = oneshot::channel();
        let req = Request::Migrate(tx, Arc::new(Mutex::new(pool)));
        self.tx.send(req).await?;
        let res = rx.await?;
        match res {
            Response::Success => Ok(()),
            Response::Error(e) => Err(anyhow::anyhow!(e)),
        }
    }
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
