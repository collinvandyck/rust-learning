use crate::chat::error::Error;
use std::{
    net::{TcpListener, TcpStream},
    sync::mpsc::{self, Receiver},
    thread,
};

pub struct Server {
    port: u32,
}

pub fn new(port: u32) -> Server {
    Server::new(port)
}

impl Server {
    fn new(port: u32) -> Self {
        Self { port }
    }

    pub fn serve(&self) -> Result<(), Error> {
        let (tx, rx) = mpsc::channel();
        let coord = thread::spawn(move || Self::coordinator(rx));
        let addr = format!("0.0.0.0:{}", self.port);
        let listener = TcpListener::bind(&addr).or_else(|e| Err(Error::BindFailure(e)))?;
        println!("Listening on {}", addr);
        for conn in listener.incoming() {
            match conn {
                Err(e) => return Err(Error::AcceptFailure(e)),
                Ok(conn) => {
                    if tx.send(conn).is_err() {
                        break;
                    }
                }
            }
        }
        let _ = coord.join();
        Ok(())
    }

    fn coordinator(rx: Receiver<TcpStream>) {
        for stream in rx {
            dbg!(stream);
        }
    }
}
