use std::{
    io,
    net::{TcpListener, TcpStream},
    thread,
};

#[derive(Clone)]
pub struct Server {
    port: u32,
}

impl Server {
    pub fn new(port: u32) -> Server {
        Server { port }
    }

    pub fn run(&self) -> io::Result<()> {
        let addr = format!("0.0.0.0:{}", self.port);
        let listener = TcpListener::bind(addr)?;
        for stream in listener.incoming() {
            let stream = stream?;
            let server = self.clone();
            thread::spawn(move || server.handle(stream));
        }
        Ok(())
    }

    fn handle(&self, stream: TcpStream) {
        dbg!(stream);
    }
}
