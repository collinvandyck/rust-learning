use tracing::{event, info, Level};
use tracing_attributes::instrument;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder().finish();
    let guard = tracing::subscriber::set_default(subscriber);
    event!(Level::INFO, age = 48, "hi there");
    drop(guard);
    info!("Starting");
    do_stuff().await;
    info!("Done");
}

#[instrument]
async fn do_stuff() {
    event!(Level::INFO, "foo bar");
    info!("hi");
}
