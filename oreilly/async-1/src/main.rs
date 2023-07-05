use std::io;

fn main() -> io::Result<()> {
    use async_std::task;
    let res = task::block_on(foo());
    assert_eq!(res, "Foo");
    Ok(())
}

async fn foo() -> String {
    "Foo".into()
}
