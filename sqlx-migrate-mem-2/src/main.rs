fn main() {}

#[derive(sqlx::FromRow, Debug, PartialEq, Eq)]
struct Record(String);

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use sqlx::{
        sqlite::{SqliteConnectOptions, SqlitePoolOptions},
        Pool, Sqlite,
    };
    use std::str::FromStr;

    #[tokio::test]
    async fn test_migration() -> Result<()> {
        let pool = connect().await?;
        let mut conn = pool.acquire().await?;

        let recs = sqlx::query_as::<_, Record>("select name from foo")
            .fetch_all(&mut conn)
            .await?;
        assert_eq!(recs.len(), 1);
        assert_eq!(recs.get(0), Some(&Record("collin".to_string())));

        Ok(())
    }

    async fn connect() -> Result<Pool<Sqlite>> {
        let connection_str = "sqlite://?mode=memory&cache=private";
        let opts = SqliteConnectOptions::from_str(connection_str)?;
        let mut pool = SqlitePoolOptions::new()
            .min_connections(2)
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

        Ok(())
    }
}
