use crate::task::Type;
use async_trait::async_trait;
use std::sync::Arc;

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

pub type HookResult = Result<(), Arc<anyhow::Error>>;

#[derive(Default)]
pub struct Hooks(pub Option<Box<dyn Callback + Send + Sync + 'static>>);

#[async_trait]
impl Callback for Hooks {
    /// Called when the task has been scheduled, but before the task
    /// actually starts executing.
    async fn on_task_start(&self, typ: &Type) -> HookResult {
        if let Some(cb) = &self.0 {
            cb.on_task_start(typ).await
        } else {
            Ok(())
        }
    }

    /// Called when the task has been scheduled, but before the task
    /// actually starts executing.
    async fn on_task_complete(&self, typ: &Type) -> HookResult {
        if let Some(cb) = &self.0 {
            cb.on_task_complete(typ).await
        } else {
            Ok(())
        }
    }
}
