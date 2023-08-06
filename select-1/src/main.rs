use std::future::Future;
use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
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
