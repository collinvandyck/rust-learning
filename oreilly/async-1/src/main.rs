use std::{
    io,
    time::{self, SystemTime},
};

fn main() -> io::Result<()> {
    use async_std::task;
    let res = task::block_on(foo());
    assert_eq!(res, "Foo");

    let v = (0..2)
        .map(|_| task::spawn_local(time()))
        .collect::<Vec<_>>();

    dbg!(v);

    Ok(())
}

async fn time() -> SystemTime {
    time::SystemTime::now()
}

async fn foo() -> String {
    "Foo".into()
}
