use crate::chat::error::Error;
use std::{
    net::{TcpListener, TcpStream},
    sync::mpsc::Sender,
};

use super::coord;

pub struct Server {
    port: u32,
    coord: Sender<TcpStream>,
}

pub fn new(port: u32) -> Server {
    Server::new(port)
}

impl Server {
    fn new(port: u32) -> Self {
        let coord = coord::start();
        Self { port, coord }
    }

    pub fn serve(&self) -> Result<(), Error> {
        let addr = format!("0.0.0.0:{}", self.port);
        let listener = TcpListener::bind(&addr).or_else(|e| Err(Error::BindFailure(e)))?;
        println!("Listening on {}", addr);
        for conn in listener.incoming() {
            match conn {
                Err(e) => return Err(Error::AcceptFailure(e)),
                Ok(conn) => {
                    if self.coord.send(conn).is_err() {
                        break;
                    }
                }
            }
        }
        Ok(())
    }
}
