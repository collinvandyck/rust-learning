use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    loop {
        let (foobar, _addr) = listener.accept().await.unwrap();
        process(foobar).await;
    }
}

async fn process(socket: TcpStream) {
    let mut conn = Connection::new(socket);
    if let Some(frame) = conn.read_frame().await.unwrap() {
        println!("Got: {:?}", frame);
        let response = Frame::Error("unimplemented".to_string());
        conn.write_frame(&response).await.unwrap();
    }
}

