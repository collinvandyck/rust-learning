use std::{
    io::{self},
    net::{TcpListener, TcpStream},
    thread,
};

fn main() {
    let s = Server::new(3000);
    s.start().expect("failure");
}

type Result<T> = io::Result<T>;

#[derive(Clone)]
struct Server {
    port: u32,
}

impl Server {
    fn new(port: u32) -> Server {
        Server { port }
    }

    fn start(&self) -> Result<()> {
        let addr = format!("0.0.0.0:{}", self.port);
        let listener = TcpListener::bind(addr)?;
        for (id, stream) in listener.incoming().enumerate() {
            let stream = stream?;
            let server = self.clone();
            thread::spawn(move || server.handle(id, stream));
        }
        Ok(())
    }

    fn handle(&self, id: usize, stream: TcpStream) {
        let client = Client { id, stream };
        dbg!(client);
    }
}

#[derive(Debug)]
struct Client {
    id: usize,
    stream: TcpStream,
}
