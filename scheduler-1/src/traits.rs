use async_trait::async_trait;

#[async_trait]
pub trait Foo {
    async fn hello(&self) -> i32 {
        42
    }
}
