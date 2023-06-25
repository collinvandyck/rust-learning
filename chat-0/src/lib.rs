use std::{
    io::Error,
    net::{SocketAddr, TcpListener, TcpStream},
    thread,
};

#[derive(Clone)]
pub struct Server {
    port: usize,
}

impl Server {
    pub fn new(port: usize) -> Server {
        Server { port }
    }

    pub fn run(&self) -> Result<(), Error> {
        let addr = format!("0.0.0.0:{}", self.port);
        println!("Listening on {}", addr);
        let listener = TcpListener::bind(addr)?;
        loop {
            let (stream, addr) = listener.accept()?;
            let server = self.clone();
            thread::spawn(move || {
                server.handle(stream, addr);
            });
        }
    }

    fn handle(&self, stream: TcpStream, addr: SocketAddr) {
        println!("Connected to {}", addr);
    }
}
