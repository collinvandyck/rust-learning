#![allow(dead_code)]
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
    let mut client = Client::new(config);
    if let Err(err) = client.run().await {
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
                Some(event) = server_rx.recv() => {
                    self.server_event(&event).await?;
                }
                Some(input) = user_rx.recv() => {
                    println!("User input: {input}");
                }
                else => break,
            }
        }
        println!("Quitting");
        Ok(())
    }

    async fn server_event(&mut self, input: &str) -> Result<()> {
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
    use protocol::{ServerEvent, Timestamp, User};
    use std::{
        net::SocketAddr,
        sync::{Arc, Mutex},
        time::Duration,
    };
    use tokio::net::TcpListener;

    #[tokio::test]
    async fn test_client() {
        let server = Server::new().await;
        let addr = format!("{:?}", &server.addr);
        let name = Some(String::from("test-name"));
        let output = Stdout::default();
        let stdout: Box<dyn io::Write + Send> = Box::new(output.clone());
        let stdout = protocol::Stdout::from(stdout);
        let config = protocol::ClientConfig { addr, name, stdout };
        let mut client = Client::new(config);
        tokio::spawn(async move {
            client.run().await.unwrap();
        });
        let (stream, _addr) = server.listener.accept().await.unwrap();
        let (stream_rx, stream_tx) = stream.into_split();
        let mut reader = BufReader::new(stream_rx);
        let mut writer = BufWriter::new(stream_tx);
        let mut buf = String::new();
        reader.read_line(&mut buf).await.unwrap();
        let event = serde_json::from_str::<ClientEvent>(&buf).unwrap();
        match event {
            ClientEvent::Ident(User { name }) => assert_eq!(name, "test-name"),
            _ => panic!("bad event: {event:?}"),
        }
        let event = ServerEvent::Message(protocol::Message {
            from: User {
                name: String::from("other-user"),
            },
            text: String::from("hi there"),
            time: Timestamp::default(),
        });
        let event = serde_json::to_string(&event).unwrap();
        let event = format!("{event}\n");
        writer.write_all(event.as_bytes()).await.unwrap();
        writer.flush().await.unwrap();
        drop(writer);
        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    struct Server {
        listener: TcpListener,
        addr: SocketAddr,
    }

    impl Server {
        async fn new() -> Self {
            let listener = TcpListener::bind("0.0.0.0:0").await.unwrap();
            let addr = listener.local_addr().unwrap();
            Self { listener, addr }
        }
    }

    #[derive(Default, Clone)]
    struct Stdout {
        buf: Arc<Mutex<Vec<u8>>>,
    }

    impl Write for Stdout {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            let mut b = self.buf.lock().unwrap();
            std::io::Write::write(&mut *b, buf)
        }
        fn flush(&mut self) -> io::Result<()> {
            let mut b = self.buf.lock().unwrap();
            std::io::Write::flush(&mut *b)
        }
    }
}
