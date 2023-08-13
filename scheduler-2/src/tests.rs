use std::time::Duration;

use crate::scheduler::Scheduler;
use anyhow::Result;
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
