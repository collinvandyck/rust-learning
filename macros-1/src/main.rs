mod macros;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let _v = vec![1, 2, 3];
    println!("Hello, world!");
    let _v = myvec!(1, 2, 3);
    println!("{_v:?}");
    Ok(())
}
