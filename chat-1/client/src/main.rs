use anyhow::Result;
use protocol::prelude::*;
use std::{
    io::{self, Write},
    process,
};

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        eprintln!("{err:?}");
        process::exit(1);
    }
}

async fn run() -> Result<()> {
    let config = protocol::Config::parse();
    println!("Got addr: {}", &config.addr);
    let name = get_name()?;
    println!("hi {name}.");
    Ok(())
}

fn get_name() -> Result<String> {
    let mut name = String::new();
    print!("Name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name).unwrap();
    name = name.trim().to_string();
    Ok(name)
}
