use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
};

pub(crate) struct Command(WrappedFuture);
type AsyncFuture = Box<dyn Future<Output = ()> + Send + 'static>;
type WrappedFuture = Arc<Mutex<Option<Pin<AsyncFuture>>>>;

impl Command {
    pub(crate) fn new<F>(f: F) -> Self
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let future: WrappedFuture = Arc::new(Mutex::new(Some(Box::pin(f))));
        Self(future)
    }

    pub(crate) async fn run(&mut self) {
        if let Some(fut) = {
            let mut future = self.0.lock().unwrap();
            let fut = future.take();
            fut
        } {
            fut.await;
        }
    }
}
