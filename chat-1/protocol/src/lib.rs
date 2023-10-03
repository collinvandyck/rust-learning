pub mod prelude {
    pub use clap::Parser;
    pub use serde::{Deserialize, Serialize};
    pub use std::ops::{Deref, DerefMut};
    pub use std::time::Instant;
    pub use time::OffsetDateTime;
}
use prelude::*;

/// Both the client and server can use this Clap config when starting
#[derive(Serialize, Deserialize, Clone, Parser)]
pub struct Config {
    pub addr: String,
}

/// The client/server protocol consists of sending events
#[derive(Serialize, Deserialize, Clone)]
pub enum Event {
    Client(ClientEvent),
    Server(ServerEvent),
}

/// ClientEvent is sent by the client
#[derive(Serialize, Deserialize, Clone)]
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
#[derive(Serialize, Deserialize, Clone)]
pub struct Message {
    pub from: User,
    pub text: String,
    pub time: Timestamp,
}

/// Identifies a user
#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub name: String,
}

/// A wrapper around time crate so we can attach methods
#[derive(Serialize, Deserialize, Clone)]
pub struct Timestamp(OffsetDateTime);

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
