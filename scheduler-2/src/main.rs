use std::time::Duration;

use anyhow::Result;
use scheduler_2::scheduler::Scheduler;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<()> {
    let s = Scheduler::new();
    for _ in 0..10 {
        let res = s
            .schedule("task1", async {
                tokio::time::sleep(std::time::Duration::from_millis(1)).await;
                println!("Hello world");
            })
            .await?;
        println!("{:?}", res);
    }
    sleep(Duration::from_millis(10)).await;
    Ok(())
}
