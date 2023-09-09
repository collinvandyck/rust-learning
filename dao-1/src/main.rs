use anyhow::Result;
use dao_1::dao;

#[tokio::main]
async fn main() -> Result<()> {
    let _dao = dao::Dao::new().await?;
    Ok(())
}
