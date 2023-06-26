use std::{
    io,
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
            thread::spawn(move || {
                if let Err(err) = server.handle_client(id, stream) {
                    println!("client {} error: {}", id, err)
                } else {
                    println!("client {} disconnected", id)
                }
            });
        }
        Ok(())
    }

    fn controller(&self) -> Result<()> {
        Ok(())
    }

    fn handle_client(&self, id: usize, stream: TcpStream) -> Result<()> {
        let client = Client::new(id, stream);
        dbg!(client);
        Ok(())
    }
}

#[derive(Debug)]
struct Client {
    _id: usize,
    _stream: TcpStream,
}

impl Client {
    fn new(id: usize, stream: TcpStream) -> Client {
        Client {
            _id: id,
            _stream: stream,
        }
    }
}
