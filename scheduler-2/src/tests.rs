use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::hooks;
use crate::hooks::HookResult;
use crate::rules::{Rule, Rules};
use crate::scheduler::Response;
use crate::task::Type;
use crate::{hooks::Callback, scheduler::Scheduler};
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
    let sched = Scheduler::builder().hooks(hooks.clone().into()).build();
    let num = 10;
    for _ in 0..num {
        sched.run_task("task", async {}).await?;
    }
    sched.wait().await?;
    assert_eq!(num, hooks.get_count());

    Ok(())
}

#[tokio::test]
async fn test_scheduler_task_panic() -> Result<()> {
    let hooks = TestHooks::new();
    let sched = Scheduler::builder().hooks(hooks.clone().into()).build();

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

#[tokio::test]
async fn test_scheduler_rules() -> Result<()> {
    let hooks = TestHooks::new();
    let count = 10;
    let rules = Rules::builder()
        .rule(
            "foo",
            Rule {
                max_running: count,
                run_every: None,
            },
        )
        .rule(
            "bar",
            Rule {
                max_running: 5,
                run_every: None,
            },
        )
        .build();
    let sched = Scheduler::builder()
        .hooks(hooks.clone().into())
        .rules(rules)
        .build();

    let (tx, rx) = tokio::sync::mpsc::channel(1);
    for _ in 0..count {
        let tx = tx.clone();
        let res = sched
            .run_task("foo", async move {
                let _ = tx.send(()).await;
            })
            .await?;
        assert_eq!(res, Response::Accepted);
    }
    // allow the tasks to run.
    drop(rx);

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
impl Callback for TestHooks {
    async fn on_task_start(&self, typ: &Type) -> HookResult {
        println!("Hook: on_task_start: {:?}", typ);
        self.bump_count();
        Ok(())
    }

    async fn on_task_complete(&self, typ: &Type) -> HookResult {
        println!("Hook: on_task_complete: {:?}", typ);
        Ok(())
    }
}
