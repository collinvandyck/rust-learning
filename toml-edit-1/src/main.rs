use anyhow::anyhow;
use anyhow::bail;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let home = home::home_dir().ok_or_else(|| anyhow!("No home directory"))?;
    let config = home.join(".aws").join("config");
    if !config.is_file() {
        bail!("expected {config:?} to exist");
    }
    println!("Hello, world!");
    Ok(())
}
