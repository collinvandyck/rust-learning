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

    async_blocks();

    Ok(())
}

fn async_blocks() {
    let foo = async { 5 };
    let bar = async { 6 };
    let bar = task::block_on(bar);
    let foo = task::block_on(foo);
    println!("foo: {}, bar: {}", foo, bar);
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

    let r = rand::thread_rng().gen_range(10..11);

    // this jams up the async runtime b/c it's blocking
    //thread::sleep(time::Duration::from_millis(r * 10));

    // this is the async version which yields to the runtime
    task::sleep(time::Duration::from_millis(r * 10)).await;

    println!("{id:?} produced {res:?}");
    res
}

async fn foo() -> String {
    "Foo".into()
}
