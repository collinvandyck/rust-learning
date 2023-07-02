use crate::chat::error::Error;
use std::net::TcpListener;

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
        let addr = format!("0.0.0.0:{}", self.port);
        let listener = TcpListener::bind(&addr).or_else(|e| Err(Error::BindFailure(e)))?;
        println!("Listening on {}", addr);
        for conn in listener.incoming() {
            let conn = match conn {
                Ok(c) => Ok(c),
                Err(e) => Err(Error::AcceptFailure(e)),
            }?;
            dbg!(conn);
        }
        Ok(())
    }
}
