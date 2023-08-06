use std::future::Future;
use tokio::sync::oneshot;
use tokio::net::{TcpListener, TcpStream};
use std::io;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    test_select_loop().await;
    match test_accept_loop().await {
        Ok(()) => println!("Accept done"),
        Err(e) => eprintln!("Accept failed: {e:?}"),
    }
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

async fn get(rx1: impl Future<Output=&str>) {
    tokio::select! {
        val = rx1 => {
            println!("rx1 first with {val:?}");
        }
    }
}

fn process(socket: TcpStream) {
   println!("Socket connect: {socket:?}") ;
}

async fn test_accept_loop() -> io::Result<()> {
    let (tx, rx) = oneshot::channel();

    tokio::spawn(async move {
        sleep(Duration::from_secs(60)).await;
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