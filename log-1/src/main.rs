#[macro_use]
extern crate log;

fn main() {
    env_logger::init();
    info!("Hello!");
    error!("Hello!");
    warn!("Warning");

    let val = std::env::var("foo").unwrap_or("UNKNOWN".to_string());
    println!("foo:  {}", val);
}
