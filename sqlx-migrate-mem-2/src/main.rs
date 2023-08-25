fn main() {}

#[cfg(test)]
mod tests {
    use anyhow::{anyhow, Result};
    use sqlx::{
        sqlite::{SqliteConnectOptions, SqlitePoolOptions},
        Pool, Sqlite,
    };
    use std::str::FromStr;

    #[derive(sqlx::FromRow, Debug, PartialEq, Eq)]
    struct Record(String);

    #[tokio::test]
    async fn test_migration() -> Result<()> {
        let mut pool = connect().await?;
        let recs = get_records(&mut pool)
            .await
            .map_err(|e| anyhow!("test check failed: {e}"))?;
        assert_eq!(recs.len(), 1);
        assert_eq!(recs.get(0), Some(&Record("collin".to_string())));
        Ok(())
    }

    async fn get_records(pool: &mut Pool<Sqlite>) -> Result<Vec<Record>> {
        let mut conn = pool.acquire().await?;
        let recs = sqlx::query_as::<_, Record>("select name from foo")
            .fetch_all(&mut conn)
            .await?;
        Ok(recs)
    }

    async fn connect() -> Result<Pool<Sqlite>> {
        let _connection_str = "sqlite://?mode=memory&cache=private";
        let connection_str = "sqlite::memory:";
        let opts = SqliteConnectOptions::from_str(connection_str)?;
        let mut pool = SqlitePoolOptions::new()
            .min_connections(100)
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
        println!("migrated!");
        let recs = get_records(pool)
            .await
            .map_err(|e| anyhow!("migrate check failed: {e}"))?;
        assert_eq!(recs.len(), 1);
        assert_eq!(recs.get(0), Some(&Record("collin".to_string())));

        Ok(())
    }
}
