#![allow(dead_code)]

use std::{future::Future, pin::Pin, sync::Arc};
use tokio::sync::{self, oneshot};
#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
}

/// A Fn that produces new Fut futures.
///
/// It is wrapped in arc because the FutFn must be cloneable.
#[derive(Clone)]
struct FutFn(Arc<Box<dyn Fn() -> Fut + Send + Sync + 'static>>);

impl FutFn {
    fn new<F>(f: F) -> Self
    where
        F: Fn() -> Fut + Send + Sync + 'static,
    {
        let f = Box::new(f);
        Self(Arc::new(f))
    }
    fn spawn(&self) {
        let fut = self.0();
        fut.spawn();
    }
}

/// Fut is a wrapper around a Boxed future that can be spawned
struct Fut(Pin<Box<dyn Future<Output = ()> + Send + 'static>>);

impl Fut {
    fn new<F>(f: F) -> Self
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let f = Box::pin(f);
        Self(f)
    }
    // Consumes the future and passes it off to tokio.
    fn spawn(self) {
        tokio::spawn(self.0);
    }
}

async fn start_do_stuff() -> oneshot::Receiver<()> {
    let (tx, rx) = sync::oneshot::channel();
    let fut = Fut::new(async { do_stuff(tx).await });
    fut.spawn();
    rx
}

async fn do_stuff(tx: oneshot::Sender<()>) {
    println!("Doing stuff!");
    tx.send(()).unwrap();
}
