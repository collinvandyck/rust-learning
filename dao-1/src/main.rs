use anyhow::Result;
use dao_1::dao::DB;

#[tokio::main]
async fn main() -> Result<()> {
    let dao = DB::new_db().await?;
    let _dao = dao.clone();
    Ok(())
}

#[tokio::test]
async fn test_dao() -> Result<()> {
    use dao_1::dao::User;
    #[double]
    use dao_1::dao::DB;
    use mockall_double::double;
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
