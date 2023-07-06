use std::{fmt::Display, ops::Deref, sync::Arc};

use serde::{Deserialize, Serialize};

pub mod utils;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ArcString(Arc<String>);

impl From<&str> for ArcString {
    fn from(value: &str) -> Self {
        ArcString(Arc::new(value.to_string()))
    }
}

impl From<&String> for ArcString {
    fn from(value: &String) -> Self {
        ArcString::from(value.as_str())
    }
}

impl Deref for ArcString {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for ArcString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum FromClient {
    Join {
        group_name: ArcString,
    },
    Post {
        group_name: ArcString,
        message: ArcString,
    },
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum FromServer {
    Message {
        group_name: ArcString,
        message: ArcString,
    },
    Error(String),
}

#[test]
fn test_fromclient_json() {
    let from_client = FromClient::Post {
        group_name: "Dogs".into(),
        message: "Samoyeds rock!".into(),
    };
    let json = serde_json::to_string(&from_client).unwrap();
    assert_eq!(
        json,
        r#"{"Post":{"group_name":"Dogs","message":"Samoyeds rock!"}}"#
    );
    assert_eq!(
        serde_json::from_str::<FromClient>(&json).unwrap(),
        from_client
    );
}
