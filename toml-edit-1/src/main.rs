use anyhow::anyhow;
use anyhow::bail;
use anyhow::Result;
use configparser::ini::Ini;
use tokio::fs;
use toml::Table;

#[tokio::main]
async fn main() -> Result<()> {
    let home = home::home_dir().ok_or_else(|| anyhow!("No home directory"))?;
    let config = home.join(".aws").join("config");
    if !config.is_file() {
        bail!("expected {config:?} to exist");
    }

    let contents = fs::read_to_string(&config).await?;
    let _ini = Ini::new().load(&config).map_err(|err| anyhow!("{err}"))?;
    println!("{_ini:?}");
    Ok(())
}
