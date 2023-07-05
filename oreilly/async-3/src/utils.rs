use std::error::Error;

use async_std::net::TcpStream;

use crate::FromClient;

pub type ChatError = Box<dyn Error + Send + Sync + 'static>;
pub type ChatResult<T> = Result<T, ChatError>;

pub async fn send_as_json(&mut to_server: TcpStream, req: &FromClient) {}
