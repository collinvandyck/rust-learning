use std::time::Duration;

use tokio::{sync::oneshot, time::sleep};
use tracing::{event, info, Level};
use tracing_attributes::instrument;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder().finish();
    let guard = tracing::subscriber::set_default(subscriber);
    event!(Level::INFO, age = 48, "hi there");

    let (tx, rx) = oneshot::channel();

    tokio::spawn(async move {
        tracing_subscriber::fmt::init();
        info!("in async thing");
        sleep(Duration::from_millis(10)).await;
        info!("done async thing");
        tx.send(()).unwrap();
    });

    rx.await.unwrap();

    run().await;
    sleep(Duration::from_millis(50)).await;
    drop(guard);
}

#[instrument]
async fn run() {
    info!("Starting");
    do_stuff().await;
    info!("Done");
}

#[instrument]
async fn do_stuff() {
    event!(Level::INFO, "foo bar");
    info!("do_stuff");
}
