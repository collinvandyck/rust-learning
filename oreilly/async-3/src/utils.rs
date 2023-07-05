use std::error::Error;

use async_std::{
    io::{prelude::BufReadExt, WriteExt},
    stream::{Stream, StreamExt},
};
use serde::{de::DeserializeOwned, Serialize};

pub type ChatError = Box<dyn Error + Send + Sync + 'static>;
pub type ChatResult<T> = Result<T, ChatError>;

pub async fn send_as_json<S, P>(outbound: &mut S, packet: &P) -> ChatResult<()>
where
    S: async_std::io::Write + Unpin,
    P: Serialize,
{
    let mut json = serde_json::to_string(packet)?;
    json.push('\n');
    outbound.write_all(json.as_bytes()).await?;
    Ok(())
}

pub fn receive_as_json<S, P>(inbound: S) -> impl Stream<Item = ChatResult<P>>
where
    S: async_std::io::BufRead + Unpin,
    P: DeserializeOwned,
{
    inbound.lines().map(|line| -> ChatResult<P> {
        let line = line?;
        let parsed = serde_json::from_str::<P>(&line)?;
        Ok(parsed)
    })
}
