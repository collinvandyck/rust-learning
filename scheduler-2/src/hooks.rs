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

    /// Called when the task has completed executing.
    ///
    /// TODO: how to convey task return status?
    async fn on_task_complete(&self, typ: TaskType) -> Result<()>;
}

pub struct DefaultHooks {}

#[async_trait]
impl Hooks for DefaultHooks {
    async fn on_task_start(&self, _typ: TaskType) -> Result<()> {
        Ok(())
    }
    async fn on_task_complete(&self, _typ: TaskType) -> Result<()> {
        Ok(())
    }
}
