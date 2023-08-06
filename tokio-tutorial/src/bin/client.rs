use mini_redis::client;

#[tokio::main]
async fn main() {
    let t1 = tokio::spawn(async {
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();
        let res = client.get("foo").await;
    });
    let t2 = tokio::spawn(async {
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();
        client.set("foo", "bar".into()).await;
    });
    t1.await.unwrap();
    t2.await.unwrap();
}