use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use bytes::Bytes;

use mini_redis::{Command, Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

type Db = Arc<Mutex<HashMap<String,Bytes>>>;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    let db = Arc::new(Mutex::new(HashMap::new()));
    loop {
        let (foobar, _addr) = listener.accept().await.unwrap();
        let db = db.clone();
        tokio::spawn(async move {
            process(db, foobar).await;
        });
    }
}

async fn process(db: Db, socket: TcpStream) {
    let mut conn = Connection::new(socket);
    while let Some(frame) = conn.read_frame().await.unwrap() {
        println!("Got frame: {frame:?}");
        let response = match Command::from_frame(frame).unwrap() {
            Command::Set(cmd) => {
                println!("Set frame");
                db.lock().unwrap().insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Command::Get(cmd) => {
                println!("Get frame");
                let db = db.lock().unwrap();
                let value = db.get(cmd.key());
                if let Some(value) = value {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("Unsupported command: {:?}", cmd),
        };
        println!("Writing response: {response:?}");
        conn.write_frame(&response).await.unwrap();
    }
}

