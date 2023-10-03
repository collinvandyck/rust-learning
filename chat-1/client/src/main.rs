use anyhow::{anyhow, bail, Context, Result};
use protocol::prelude::*;
use std::{
    io::{self, Write},
    net::ToSocketAddrs,
    process,
};
use tokio::net::TcpSocket;

#[allow(dead_code)]
struct Config {
    protocol: protocol::Config,
    name: String,
}

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        eprintln!("{err:?}");
        process::exit(1);
    }
}

async fn run() -> Result<()> {
    let config = Config {
        protocol: protocol::Config::parse(),
        name: get_name()?,
    };
    let addr = &config.protocol.addr;
    let addr = addr
        .to_socket_addrs()
        .context(format!("could not parse {addr}"))?
        .filter(|f| f.is_ipv4())
        .next()
        .ok_or(anyhow!("no ipv4 addrs found"))?;
    let socket = TcpSocket::new_v4()?;
    let tcp_stream = socket.connect(addr).await.context("could not connect")?;
    let (rx, tx) = tcp_stream.into_split();
    Ok(())
}

fn get_name() -> Result<String> {
    let mut name = String::new();
    print!("Name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name)?;
    name = name.trim().to_string();
    if name.is_empty() {
        bail!("empty name not allowed");
    }
    Ok(name)
}
