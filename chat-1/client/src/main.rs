use anyhow::Result;
use protocol::{prelude::*, ClientEvent, Message, ServerEvent};
use std::{
    io::{self, Write},
    net::ToSocketAddrs,
    process, thread,
};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpSocket,
    },
    sync::mpsc::{self, Receiver, Sender},
};

#[tokio::main]
async fn main() {
    let config = protocol::ClientConfig::parse();
    if let Err(err) = Client::start(config).await {
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

struct Client {
    config: protocol::ClientConfig,
}

impl Client {
    async fn start(config: protocol::ClientConfig) -> Result<()> {
        let mut client = Self::new(config);
        client.run().await
    }

    fn new(config: protocol::ClientConfig) -> Self {
        Self { config }
    }

    async fn run(&mut self) -> Result<()> {
        let name = match self.config.name.clone() {
            Some(name) => name,
            None => get_name()?,
        };
        let addr = &self.config.addr;
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
        let mut user_rx = read_user_input();
        let (server_rx, server_tx) = tcp_stream.into_split();
        let server_rx = BufReader::new(server_rx);
        let mut server_tx = BufWriter::new(server_tx);
        send_server(ClientEvent::Ident(protocol::User { name }), &mut server_tx).await?;
        let mut server_rx = read_server(server_rx).await;
        loop {
            tokio::select! {
                event = server_rx.recv() => {
                    let Some(event) = event else { break };
                    self.handle_server_event(&event).await?;
                }
                input = user_rx.recv() => {
                    let Some(input) = input else { break };
                    println!("User input: {input}");
                }
                else => break,
            }
        }
        Ok(())
    }

    async fn handle_server_event(&mut self, input: &str) -> Result<()> {
        let event = serde_json::from_str::<ServerEvent>(input)?;
        match event {
            ServerEvent::Message(message) => {
                let Message {
                    from,
                    text,
                    time: _time,
                } = message;
                let name = from.name;
                let out = format!("{name}: {text}\n");
                write!(&mut self.config.stdout, "{out}")?;
            }
        }
        Ok(())
    }
}

async fn send_server(event: ClientEvent, writer: &mut BufWriter<OwnedWriteHalf>) -> Result<()> {
    let event = serde_json::to_string(&event)?;
    writer.write_all(event.as_bytes()).await?;
    writer.write_all("\n".as_bytes()).await?;
    writer.flush().await?;
    Ok(())
}

async fn read_server(mut reader: BufReader<OwnedReadHalf>) -> Receiver<String> {
    let (tx, rx) = mpsc::channel(1024);
    tokio::spawn(async move {
        loop {
            let mut buf = String::new();
            if reader.read_line(&mut buf).await.is_err() {
                break;
            }
            let buf = buf.trim().to_string();
            if buf.is_empty() {
                break;
            }
            if tx.send(buf).await.is_err() {
                break;
            }
        }
    });
    rx
}

fn read_user_input() -> Receiver<String> {
    let (tx, rx) = mpsc::channel(1024);
    thread::spawn(move || {
        let _ = read_stdin_lines(tx);
    });
    rx
}

fn read_stdin_lines(tx: Sender<String>) -> Result<()> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    loop {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;
        let buf = buf.trim().to_string();
        let _ = rt.block_on(tx.send(buf));
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_harness() {
        protocol::verify_client(Client::start).await;
    }
}
