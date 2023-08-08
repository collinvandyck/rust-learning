use scheduler_1::scheduler::Scheduler;

#[tokio::main]
async fn main() {
    let s = Scheduler::new();
    println!("ok");
}
