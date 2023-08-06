use tokio::io;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6142").await?;
    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            // copy data here
            let (mut read, mut write) = socket.split();
            if let Err(e) = io::copy(&mut read, &mut write).await {
                eprintln!("Failed to copy: {e:?}");
            }
        });
    }
}