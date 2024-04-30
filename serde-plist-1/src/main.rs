use serde::{Deserialize, Serialize};

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
#[serde(untagged)]
enum KeepAlive {
    Bool(bool),
    Map { successful_exit: bool },
}

/// `Plist` represents our service definition in struct form. The `plist` crate allows us to neatly
/// serde between the struct and XML file representation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
struct Plist {
    #[serde(default)]
    keep_alive: Option<KeepAlive>,
}

#[test]
fn test_deser1() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
        <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
        <plist version="1.0">
        <dict>
            <key>KeepAlive</key>
            <true/>
        </dict>
        </plist>"#;
    let pl: Plist = ::plist::from_bytes(xml.as_bytes()).unwrap();
    assert!(matches!(pl.keep_alive, Some(KeepAlive::Bool(true))));
}

#[test]
fn test_deser2() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
        <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
        <plist version="1.0">
        <dict>
            <key>KeepAlive</key>
            <dict>
                <key>SuccessfulExit</key>
                <false/>
            </dict>
        </dict>
        </plist>"#;
    let pl: Plist = ::plist::from_bytes(xml.as_bytes()).unwrap();
    assert!(matches!(
        pl.keep_alive,
        Some(KeepAlive::Map {
            successful_exit: false
        })
    ));
}
