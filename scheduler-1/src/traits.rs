use async_trait::async_trait;

#[async_trait]
pub trait Does {
    async fn hello(&self) -> i32 {
        42
    }
}

pub struct DefaultDoes;

#[async_trait]
impl Does for DefaultDoes {}
