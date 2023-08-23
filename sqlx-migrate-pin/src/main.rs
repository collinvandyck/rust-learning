use std::{future::Future, pin::Pin};

use anyhow::Result;
use async_trait::async_trait;
use sqlx::{migrate::MigrateError, Pool, Sqlite};
use tokio::sync::{
    mpsc::{self, Receiver},
    oneshot,
};

#[tokio::main]
async fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel(1);
    tokio::spawn(async move {
        let mut migrator = MyMigrator {
            mpsc: rx,
            pool: None,
        };
        migrator.run().await;
    });
    let (resp_tx, resp_rx) = oneshot::channel();
    tx.send(Request { tx: resp_tx }).await?;
    let res = resp_rx.await?;
    println!("res: {:?}", res);
    Ok(())
}

struct Request {
    tx: oneshot::Sender<Response>,
}

#[derive(Debug)]
struct Response;

#[async_trait]
trait Migrator {
    async fn migrate(&mut self);
}

struct MyMigrator {
    mpsc: Receiver<Request>,
    pool: Option<Pool<Sqlite>>,
}

impl MyMigrator {
    async fn run(&mut self) {
        while let Some(request) = self.mpsc.recv().await {
            self.migrate().await;
            request.tx.send(Response).unwrap();
        }
    }
}

#[async_trait]
impl Migrator for MyMigrator {
    async fn migrate(&mut self) {
        let fut = async move {
            let pool = self.pool.take().unwrap();
            let conn = pool.acquire().await.unwrap();
            let mut conn = conn.detach();
            let fut = sqlx::migrate!("./db").run_direct(&mut conn);
            let fut: Pin<Box<dyn Future<Output = Result<(), MigrateError>> + Send>> = Box::pin(fut);
            let _ = fut.await;
        };
        fut.await;
    }
}
