use std::time::Duration;

use crate::hooks::HookResult;
use crate::task::TaskType;
use crate::{hooks::Hooks, scheduler::Scheduler};
use anyhow::Result;
use async_trait::async_trait;
use tokio::time::sleep;

#[tokio::test]
async fn test_scheduler() -> Result<()> {
    let sched = Scheduler::builder().build();
    sched
        .task("task", async {
            sleep(Duration::from_millis(0)).await;
            println!("task");
        })
        .await?;
    sched.wait().await?;
    Ok(())
}

#[tokio::test]
async fn test_scheduler_hooks() -> Result<()> {
    let hooks = TestHooks::new();
    let sched = Scheduler::builder().hooks(hooks).build();
    sched
        .task("task", async {
            sleep(Duration::from_millis(0)).await;
            println!("task");
        })
        .await?;
    sched.wait().await?;
    Ok(())
}

struct TestHooks;

impl TestHooks {
    fn new() -> Self {
        TestHooks
    }
}

#[async_trait]
impl Hooks for TestHooks {
    async fn on_task_start(&self, typ: &TaskType) -> HookResult {
        println!("Hook: on_task_start: {:?}", typ);
        Ok(())
    }
}
