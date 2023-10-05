pub mod verify;
pub use verify::verify_client;
pub mod prelude {
    pub use async_trait::async_trait;
    pub use clap::Parser;
    pub use serde::{Deserialize, Serialize};
    pub use std::ops::{Deref, DerefMut};
    pub use std::time::Instant;
    pub use time::OffsetDateTime;
}
use std::{
    io::{self, Write},
    sync::{Arc, Mutex},
};

use anyhow::Result;
use prelude::*;

#[derive(Serialize, Deserialize, Clone, Parser)]
pub struct ServerConfig {
    pub addr: String,
}

#[async_trait]
pub trait IClient {
    async fn run(&mut self, config: ClientConfig) -> Result<()>;
}

#[derive(Parser)]
pub struct ClientConfig {
    /// the name of the user. if empty, the client will need to ask the user.
    #[arg(long)]
    pub name: Option<String>,

    /// the address to which to connect (e.g. localhost:8000).
    pub addr: String,

    /// clients should write to Stdout. you can use the write! macro to do this. the verifier will
    /// look at the output written to this to verify the output.
    #[clap(skip)]
    pub stdout: Stdout,
}

#[derive(Default, Clone)]
pub struct Stdout(Arc<Mutex<Option<Box<dyn Write + Send>>>>);

impl From<Vec<u8>> for Stdout {
    fn from(value: Vec<u8>) -> Self {
        Stdout(Arc::new(Mutex::new(Some(Box::new(value)))))
    }
}

impl From<Box<dyn Write + Send>> for Stdout {
    fn from(value: Box<dyn Write + Send>) -> Self {
        Stdout(Arc::new(Mutex::new(Some(value))))
    }
}

impl Write for Stdout {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut opt = self.lock().expect("lock fail");
        if let Some(opt) = opt.as_mut() {
            opt.write(buf)
        } else {
            io::stdout().write(buf)
        }
    }
    fn flush(&mut self) -> std::io::Result<()> {
        let mut opt = self.lock().expect("lock fail");
        if let Some(opt) = opt.as_mut() {
            opt.flush()
        } else {
            io::stdout().flush()
        }
    }
}

impl Deref for Stdout {
    type Target = Arc<Mutex<Option<Box<dyn Write + Send>>>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Stdout {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[test]
fn test_client_config() {
    fn is_send_sync<T: Send + Sync>() {}
    is_send_sync::<ClientConfig>()
}

/// The client/server protocol consists of sending events
#[derive(Serialize, Deserialize, Clone)]
pub enum Event {
    Client(ClientEvent),
    Server(ServerEvent),
}

/// ClientEvent is sent by the client
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ClientEvent {
    /// The client has identified themselves. This ident remains valid for the duration of the
    /// connection.
    Ident(User),

    /// The client has sent a message
    Message(Message),
}

/// ServerEvent is sent by the server
#[derive(Serialize, Deserialize, Clone)]
pub enum ServerEvent {
    /// Someone else sent a message
    Message(Message),
}

/// Represents a message in the chat
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    pub from: User,
    pub text: String,
    pub time: Timestamp,
}

/// Identifies a user
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub name: String,
}

/// A wrapper around time crate so we can attach methods
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Timestamp(OffsetDateTime);

impl Default for Timestamp {
    fn default() -> Self {
        let time = OffsetDateTime::now_utc();
        Self(time)
    }
}

impl Deref for Timestamp {
    type Target = OffsetDateTime;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Timestamp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
