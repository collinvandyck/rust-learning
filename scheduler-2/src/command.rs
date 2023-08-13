use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
};

type AsyncFuture = Box<dyn Future<Output = ()> + Send + 'static>;
type WrappedFuture = Arc<Mutex<Option<Pin<AsyncFuture>>>>;

/// Command wraps futures to be executed by the scheduler.
pub(crate) struct Command(WrappedFuture);

impl Command {
    pub(crate) fn new<F>(f: F) -> Self
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let future: WrappedFuture = Arc::new(Mutex::new(Some(Box::pin(f))));
        Self(future)
    }

    /// Runs the composed future by first taking ownership of the future and then
    /// awaiting it.
    pub(crate) async fn run(&mut self) {
        let fut = { self.0.lock().unwrap().take() };
        fut.unwrap().await;
    }
}
