use std::future::Future;
use std::io;
use std::time::Duration;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, oneshot};
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    test_select_loop().await;
    match test_accept_loop().await {
        Ok(()) => println!("Accept done"),
        Err(e) => eprintln!("Accept failed: {e:?}"),
    }
    receive_on_multiple_channels().await;
}

async fn test_select_loop() {
    let (tx1, mut rx1) = oneshot::channel();
    let (tx2, mut rx2) = oneshot::channel();
    tokio::spawn(async {
        let _ = tx1.send("one");
    });
    tokio::spawn(async {
        let _ = tx2.send("two");
    });
    let mut one = None;
    let mut two = None;
    while one.is_none() || two.is_none() {
        tokio::select! {
            val = (&mut rx1), if one.is_none() => one = Some(val),
            val = (&mut rx2), if two.is_none() => two = Some(val),
        }
    }
    println!("{one:?} {two:?}");
}

async fn get(rx1: impl Future<Output = &str>) {
    tokio::select! {
        val = rx1 => {
            println!("rx1 first with {val:?}");
        }
    }
}

fn process(socket: TcpStream) {
    println!("Socket connect: {socket:?}");
}

async fn test_accept_loop() -> io::Result<()> {
    let (tx, rx) = oneshot::channel();

    tokio::spawn(async move {
        sleep(Duration::from_millis(500)).await;
        tx.send(()).unwrap();
    });

    let mut listener = TcpListener::bind("localhost:3465").await?;

    tokio::select! {
        _ = async {
            loop {
                let (socket, _) = listener.accept().await?;
                tokio::spawn(async move { process(socket) });
                break;
            }
            // Help the rust type inferencer out
            println!("accept loop finished");
            Ok::<_, io::Error>(())
        } => (),
        _ = rx => {
            println!("terminating accept loop");
        }
    }

    Ok(())
}

async fn receive_on_multiple_channels() {
    let (tx1, mut rx1) = mpsc::channel(128);
    let (tx2, mut rx2) = mpsc::channel(128);
    let (tx3, mut rx3) = mpsc::channel(128);

    tokio::spawn(async move {
        let _ = tx1.send("foo").await;
    });

    loop {
        let msg = tokio::select! {
            Some(msg) = rx1.recv() => msg,
            Some(msg) = rx2.recv() => msg,
            Some(msg) = rx3.recv() => msg,
            else => {
                println!("All channels closed");
                break;
            }
        };

        println!("Got {:?}", msg);
        break;
    }
}
