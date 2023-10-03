use protocol::prelude::*;

fn main() {
    let config = protocol::Config::parse();
    println!("Got addr: {}", &config.addr);
}
