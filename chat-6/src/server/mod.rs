use std::io;
mod error;

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

    pub fn serve(&self) -> Result<(), io::Error> {
        Ok(())
    }
}
