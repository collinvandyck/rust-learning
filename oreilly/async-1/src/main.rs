use std::{
    io, thread,
    time::{self, SystemTime},
};

use async_std::task;
use async_std::task::block_on;
use rand::Rng;

fn main() -> io::Result<()> {
    let res = task::block_on(foo());
    assert_eq!(res, "Foo");

    block_on(get_times(10));

    Ok(())
}

async fn get_times(num: i32) {
    let v = (0..num)
        .map(|_| task::spawn_local(time()))
        .collect::<Vec<_>>();
    for handle in v {
        handle.await;
    }
}

async fn time() -> SystemTime {
    let id = thread::current().id();
    let res = time::SystemTime::now();

    let r = rand::thread_rng().gen_range(0..10);
    thread::sleep(time::Duration::from_millis(r * 10));

    println!("{id:?} produced {res:?}");
    res
}

async fn foo() -> String {
    "Foo".into()
}
