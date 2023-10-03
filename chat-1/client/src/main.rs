#![allow(dead_code)]
use anyhow::Result;
use protocol::prelude::*;
use std::{
    io::{self, Write},
    net::ToSocketAddrs,
    process,
};
use tokio::net::TcpSocket;

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        eprintln!("{err:?}");
        process::exit(1);
    }
}

#[derive(Debug, thiserror::Error)]
enum ClientError {
    #[error("Could not parse {0}: {1}")]
    AddrParseFailure(String, io::Error),

    #[error("no ipv4 addrs found")]
    NoIpV4Addrs,

    #[error("could not connect to {0}: {1}")]
    CouldNotConnect(String, io::Error),

    #[error("Could not create socket: {0}")]
    SocketCreate(io::Error),

    #[error("Empty name not allowed")]
    EmptyName,
}

async fn run() -> Result<()> {
    let config = protocol::Config::parse();
    let name = get_name()?;
    let addr = &config.addr;
    let socket_addr = addr
        .to_socket_addrs()
        .map_err(|e| ClientError::AddrParseFailure(addr.to_string(), e))?
        .filter(|f| f.is_ipv4())
        .next()
        .ok_or(ClientError::NoIpV4Addrs)?;
    let socket = TcpSocket::new_v4().map_err(ClientError::SocketCreate)?;
    let tcp_stream = socket
        .connect(socket_addr)
        .await
        .map_err(|e| ClientError::CouldNotConnect(addr.into(), e))?;
    let (_rx, _tx) = tcp_stream.into_split();
    Ok(())
}

fn get_name() -> Result<String> {
    let mut name = String::new();
    print!("Name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name)?;
    name = name.trim().to_string();
    if name.is_empty() {
        return Err(ClientError::EmptyName.into());
    }
    Ok(name)
}
