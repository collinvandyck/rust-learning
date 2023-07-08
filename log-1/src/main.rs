#[macro_use]
extern crate log;

fn main() {
    env_logger::init();
    info!("Hello!");
    error!("Hello!");
    warn!("Warning");
}
