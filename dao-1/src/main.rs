#![allow(dead_code, unused)]

use anyhow::Result;
use dao_1::dao::User;
#[double]
use dao_1::dao::DB;
use mockall_double::double;

#[tokio::main]
async fn main() -> Result<()> {
    let _dao = DB::new_db().await?;
    Ok(())
}

#[tokio::test]
async fn test_dao() -> Result<()> {
    let mut db = DB::new();
    db.expect_get_users().returning(|| {
        Ok(vec![User {
            id: 1,
            name: "test".to_string(),
            age: 20,
        }])
    });
    let users = db.get_users().await?;
    assert_eq!(users.len(), 1);
    Ok(())
}
