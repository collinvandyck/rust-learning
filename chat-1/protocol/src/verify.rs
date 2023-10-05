use super::*;
use std::future::Future;
use std::net::SocketAddr;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;

pub async fn verify_client<Fut>(client: impl Fn(ClientConfig) -> Fut)
where
    Fut: Future<Output = Result<()>> + Send + 'static,
{
    let server = Server::new().await;
    let addr = format!("{:?}", &server.addr);
    let name = Some(String::from("test-name"));
    let buffer = Stdout::default();
    let stdout: Box<dyn io::Write + Send> = Box::new(buffer.clone());
    let stdout = super::Stdout::from(stdout);
    let config = ClientConfig { addr, name, stdout };
    let client = client(config);
    let client = tokio::spawn(async move { client.await });
    let (stream, _) = server.listener.accept().await.unwrap();
    let (stream_rx, mut stream_tx) = stream.into_split();
    let mut reader = BufReader::new(stream_rx);
    let mut buf = String::new();
    reader.read_line(&mut buf).await.unwrap();
    let event = serde_json::from_str::<ClientEvent>(&buf).unwrap();
    match event {
        ClientEvent::Ident(User { name }) => assert_eq!(name, "test-name"),
        _ => panic!("bad event: {event:?}"),
    }
    let event = ServerEvent::Message(Message {
        from: User {
            name: String::from("other-user"),
        },
        text: String::from("hi there"),
        time: Timestamp::default(),
    });
    let event = serde_json::to_string(&event).unwrap();
    let event = format!("{event}\n");
    stream_tx.write_all(event.as_bytes()).await.unwrap();
    stream_tx.flush().await.unwrap();
    drop(stream_tx);
    client.await.unwrap().unwrap();
    let out = buffer.output();
    assert_eq!(out, "other-user: hi there\n");
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

impl Stdout {
    fn output(&self) -> String {
        let b = self.buf.lock().expect("lock fail");
        let b = b.clone();
        use std::str;
        str::from_utf8(&b).expect("not valid utf8").to_string()
    }
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
