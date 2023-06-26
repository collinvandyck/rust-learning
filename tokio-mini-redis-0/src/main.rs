use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        process(socket).await;
    }
}

async fn process(socket: TcpStream) {
    // allows us to write frames instead of bytes
    let mut conn = Connection::new(socket);

    // read a frame
    if let Some(frame) = conn.read_frame().await.unwrap() {
        println!("Got: {:?}", frame);

        // respond with an error
        let response = Frame::Error("unimplemented".to_string());
        conn.write_frame(&response).await.unwrap();
    }
}

