use std::io::Error;

pub struct Server {
    port: usize,
}

impl Server {
    pub fn new(port: usize) -> Server {
        Server { port }
    }

    pub fn run(&self) -> Result<(), Error> {
        Ok(())
    }
}
