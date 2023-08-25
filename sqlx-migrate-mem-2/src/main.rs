use anyhow::Result;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    Pool, Sqlite,
};
use std::str::FromStr;

fn main() {}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::connect;
    #[tokio::test]
    async fn test_migration() -> Result<()> {
        let pool = connect().await?;
        Ok(())
    }
}

async fn connect() -> Result<Pool<Sqlite>> {
    let connection_str = "sqlite://?mode=memory&cache=private";
    let opts = SqliteConnectOptions::from_str(connection_str)?;
    let mut pool = SqlitePoolOptions::new()
        .min_connections(1)
        .max_connections(100)
        .connect_with(opts)
        .await?;
    migrate(&mut pool).await?;
    Ok(pool)
}

async fn migrate(pool: &mut Pool<Sqlite>) -> Result<()> {
    let conn = pool.acquire().await?;
    let mut conn = conn.detach();
    sqlx::migrate!("./migrations").run_direct(&mut conn).await?;
    Ok(())
}
