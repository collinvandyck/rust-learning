use anyhow::Result;
use dao_1::dao;

#[tokio::main]
async fn main() -> Result<()> {
    let _dao = dao::DB::new_dao().await?;
    Ok(())
}
