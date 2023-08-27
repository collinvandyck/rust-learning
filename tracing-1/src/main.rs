use tracing::{event, info, Level};
use tracing_attributes::instrument;

#[tokio::main]
async fn main() {
    do_stuff().await;
    println!("Done");
}

#[instrument]
async fn do_stuff() {
    event!(Level::INFO, "foo bar");
    info!("hi");
}
