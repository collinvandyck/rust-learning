use std::time::{Duration, Instant};

use crate::{Fut, FutFn};
use tokio::{
    sync::{self, mpsc},
    time::sleep,
};

#[tokio::test]
async fn test_simple() {
    let (tx, rx) = sync::oneshot::channel();
    let fut = Fut::new(async {
        tx.send(()).unwrap();
    });
    fut.spawn();
    rx.await.unwrap();
}

#[tokio::test]
async fn test_nested() {
    let (tx, rx) = sync::oneshot::channel();
    let fut = Fut::new(async {
        let (tx2, rx2) = sync::oneshot::channel();
        let fut2 = Fut::new(async {
            tx2.send(()).unwrap();
        });
        fut2.spawn();
        rx2.await.unwrap();
        tx.send(()).unwrap();
    });
    fut.spawn();
    rx.await.unwrap();
}

#[tokio::test]
async fn test_fut_fn_simple() {
    let (tx, mut rx) = mpsc::channel(1024);
    let fut_fn = FutFn::new(move || {
        let tx = tx.clone();
        Fut::new(async move {
            tx.send("hello").await.unwrap();
        })
    });

    fut_fn.spawn();
    fut_fn.spawn();

    assert_eq!(rx.recv().await.unwrap(), "hello");
    assert_eq!(rx.recv().await.unwrap(), "hello");
}

#[tokio::test]
async fn test_fut_fn() {
    let (tx, mut rx) = mpsc::channel(1);
    let ffn: FutFn = FutFn::new(move || {
        let tx = tx.clone();
        Fut::new(async move {
            sleep(Duration::from_millis(100)).await;
            tx.send(Instant::now()).await.unwrap();
        })
    });
    let times = 10;
    for _ in 0..times {
        let ffn = ffn.clone();
        tokio::spawn(async move {
            // prove that we can pass around the FutFn between threads.
            ffn.spawn();
        });
    }
    let mut last: Option<Instant> = None;
    for _ in 0..times {
        let instant = rx.recv().await.unwrap();
        if let Some(last) = last {
            let dur = instant - last;
            assert!(dur < Duration::from_millis(10));
        } else {
            last = Some(instant);
        }
    }
}
