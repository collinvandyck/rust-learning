use anyhow::{bail, Result};
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
    let _config = protocol::Config::parse();
    let _name = get_name()?;
    Ok(())
}

fn get_name() -> Result<String> {
    let mut name = String::new();
    print!("Name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name).unwrap();
    name = name.trim().to_string();
    if name.is_empty() {
        bail!("empty name not allowed");
    }
    Ok(name)
}
