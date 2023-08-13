use crate::scheduler::Scheduler;
use anyhow::Result;

#[tokio::test]
async fn test_scheduler() -> Result<()> {
    let sched = Scheduler::new();
    sched
        .task("task", async {
            println!("task");
        })
        .await?;
    sched.wait().await?;
    Ok(())
}
