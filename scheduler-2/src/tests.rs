use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::hooks::HookResult;
use crate::task::Type;
use crate::{hooks::Hooks, scheduler::Scheduler};
use anyhow::Result;
use async_trait::async_trait;
use tokio::time::sleep;

#[tokio::test]
async fn test_scheduler() -> Result<()> {
    let sched = Scheduler::builder().build();
    sched
        .run_task("task", async {
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
    let sched = Scheduler::builder().hooks(hooks.clone()).build();
    for _ in 0..10 {
        sched.run_task("task", async {}).await?;
    }
    sched.wait().await?;
    assert_eq!(10, hooks.get_count());
    Ok(())
}

#[tokio::test]
async fn test_scheduler_task_panic() -> Result<()> {
    let hooks = TestHooks::new();
    let sched = Scheduler::builder().hooks(hooks.clone()).build();

    // run a task that panics.
    sched
        .run_task("task", async {
            // panic the task.
            panic!("task panic");
        })
        .await?;
    sched.wait().await?;

    // verify that it was scheduled
    assert_eq!(1, hooks.get_count());

    // verify that we can still run tasks even after a panic.
    sched.run_task("task", async {}).await?;
    sched.wait().await?;
    assert_eq!(2, hooks.get_count());

    Ok(())
}

#[derive(Clone)]
struct TestHooks {
    count: Arc<Mutex<usize>>,
}

impl TestHooks {
    fn new() -> Self {
        TestHooks {
            count: Arc::new(Mutex::new(0)),
        }
    }
    fn get_count(&self) -> usize {
        let count = self.count.lock().unwrap();
        *count
    }
    fn bump_count(&self) {
        let mut count = self.count.lock().unwrap();
        *count = *count + 1;
    }
}

#[async_trait]
impl Hooks for TestHooks {
    async fn on_task_start(&self, typ: &Type) -> HookResult {
        println!("Hook: on_task_start: {:?}", typ);
        sleep(Duration::from_millis(5)).await;
        self.bump_count();
        Ok(())
    }

    async fn on_task_complete(&self, typ: &Type) -> HookResult {
        println!("Hook: on_task_complete: {:?}", typ);
        sleep(Duration::from_millis(5)).await;
        Ok(())
    }
}
