use crate::scheduler::Scheduler;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test]
async fn test_scheduler() {
    let sched = Scheduler::new();
    let _ = sched
        .task("task", async {
            println!("task");
        })
        .await;
    sleep(Duration::from_millis(100)).await;
}
