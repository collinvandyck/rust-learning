use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
};

use anyhow::Result;
use async_trait::async_trait;

use crate::task::TaskType;

/// Hooks defines the trait that clients can implement to provide
/// callbacks to scheduler lifecycle methods.
#[async_trait]
pub trait Hooks {
    /// Called when the task has been scheduled, but before the task
    /// actually starts executing.
    async fn on_task_start(&self, typ: TaskType) -> Result<()>;
}

type AsyncFuture = Box<dyn Future<Output = Result<()>> + Send + 'static>;
type WrappedFuture = Arc<Mutex<Option<Pin<AsyncFuture>>>>;

pub struct DefaultHooks {
    start: WrappedFuture,
}

impl DefaultHooks {
    fn new() -> Self {
        let fut = async move { Ok(()) };
        let start: WrappedFuture = Arc::new(Mutex::new(Some(Box::pin(fut))));
        DefaultHooks { start }
    }
}

#[async_trait]
impl Hooks for DefaultHooks {
    async fn on_task_start(&self, _typ: TaskType) -> Result<()> {
        Ok(())
    }
}
