use std::{
    io, thread,
    time::{self, SystemTime},
};

use async_std::task::block_on;

fn main() -> io::Result<()> {
    use async_std::task;
    let res = task::block_on(foo());
    assert_eq!(res, "Foo");

    block_on(get_times(10));

    Ok(())
}

async fn get_times(num: i32) {
    use async_std::task;
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
    println!("{id:?} produced {res:?}");
    res
}

async fn foo() -> String {
    "Foo".into()
}
