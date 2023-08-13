use crate::task::Type;
use async_trait::async_trait;
use std::sync::Arc;

pub type HookResult = Result<(), Arc<anyhow::Error>>;

// TODO: compose hooks trait in a struct.
pub(crate) type Wrapped = Option<Box<dyn Callback + Send + Sync + 'static>>;

/// Hooks defines the trait that clients can implement to provide
/// callbacks to scheduler lifecycle methods.
#[async_trait]
pub trait Callback {
    /// Called when the task has been scheduled, but before the task
    /// actually starts executing.
    async fn on_task_start(&self, typ: &Type) -> HookResult;

    /// Called when the task has been scheduled, but before the task
    /// actually starts executing.
    async fn on_task_complete(&self, typ: &Type) -> HookResult;
}
