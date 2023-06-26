use std::{
    io::{self, Error},
    net::{TcpListener, TcpStream},
    sync::mpsc,
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

enum Quit {
    Ok(String),
    Failure(String, Result<()>),
}

impl Quit {
    fn from(name: String, res: Result<()>) -> Quit {
        match res {
            Ok(_) => Quit::Ok(name),
            Err(_) => Quit::Failure(name, res),
        }
    }
}

impl Server {
    fn new(port: u32) -> Server {
        Server { port }
    }

    fn start(&self) -> Result<()> {
        let (tx, rx) = mpsc::channel();

        let server = self.clone();
        let ttx = tx.clone();
        thread::spawn(move || {
            if let Err(err) = server.controller() {
                println!("controller error: {}", err)
            }
            let _ = ttx.send("controller quit");
        });

        let server = self.clone();
        let ttx = tx.clone();
        thread::spawn(move || {
            if let Err(err) = server.listen() {
                println!("listener error: {}", err)
            }
            let _ = ttx.send("listener quit");
        });
        drop(tx);
        let val = rx.recv().unwrap();
        println!("Server quitting: {:?}", val);
        Ok(())
    }

    fn listen(&self) -> Result<()> {
        let addr = format!("0.0.0.0:{}", self.port);
        let listener = TcpListener::bind(addr)?;
        for (id, stream) in listener.incoming().enumerate().take(1) {
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
        Err(Error::new(io::ErrorKind::Other, "bloop"))
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
