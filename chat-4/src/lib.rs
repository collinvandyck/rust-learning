use std::io::Error;

pub struct Server {
    port: u32,
}

impl Server {
    pub fn new(port: u32) -> Self {
        Self { port }
    }

    pub fn run(&self) -> Result<(), Error> {
        Ok(())
    }
}
