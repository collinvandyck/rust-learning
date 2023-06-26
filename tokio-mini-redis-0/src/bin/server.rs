use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use bytes::Bytes;
use mini_redis::{Command, Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    let db = Arc::new(Mutex::new(HashMap::new()));
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let db = db.clone();
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

async fn process(socket: TcpStream, db: Db) {
    // allows us to write frames instead of bytes
    let mut conn = Connection::new(socket);

    while let Some(frame) = conn.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Command::Set(cmd) => {
                let key = cmd.key().to_string();
                let val = cmd.value().clone();
                let mut db = db.lock().unwrap();
                db.insert(key, val);
                Frame::Simple("OK".to_string())
            }
            Command::Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(val) = db.get(cmd.key()) {
                    Frame::Bulk(val.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented: {:?}", cmd),
        };
        conn.write_frame(&response).await.unwrap();
    }
}
