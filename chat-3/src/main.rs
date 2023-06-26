use core::fmt;
use std::{
    io::{self, BufRead, BufReader, BufWriter, Error, Write},
    net::{TcpListener, TcpStream},
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
};

fn main() {
    let s = Server::new(3000);
    s.start().expect("failure");
}

type Result<T> = std::result::Result<T, Error>;

enum Event {
    NewClient {
        client: usize,
        outgoing: OutgoingEvents,
    },
    Message {
        client: usize,
        val: String,
    },
    ClientQuit {
        client: usize,
    },
}

type IncomingEvents = Arc<Mutex<Receiver<Event>>>;
type OutgoingEvents = Arc<Mutex<Sender<Event>>>;

#[derive(Clone)]
struct Server {
    port: u32,
    tx: OutgoingEvents,
    rx: IncomingEvents,
}

impl Server {
    fn new(port: u32) -> Server {
        let (tx, rx) = mpsc::channel();
        let tx = Arc::new(Mutex::new(tx));
        let rx = Arc::new(Mutex::new(rx));
        Server { port, tx, rx }
    }

    fn start(&self) -> Result<()> {
        let (tx, rx) = mpsc::channel();

        let server = self.clone();
        let ttx = tx.clone();
        thread::spawn(move || {
            let res = server.controller();
            let _ = ttx.send(Quit::from("controller", res));
        });

        let server = self.clone();
        let ttx = tx.clone();
        thread::spawn(move || {
            let res = server.listen();
            let _ = ttx.send(Quit::from("listener", res));
        });

        drop(tx);
        let val = rx.recv().unwrap();
        println!("Server quitting: {}", val);
        val.into()
    }

    fn listen(&self) -> Result<()> {
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
        let mut ougoings = vec![];
        loop {
            let event = match self.rx.lock().unwrap().recv() {
                Ok(event) => event,
                Err(_) => return Result::Err(Error::new(io::ErrorKind::Other, "chan closed")),
            };
            match event {
                Event::NewClient { client, outgoing } => {
                    println!("Got new client with id: {}", client);
                    ougoings.push(outgoing);
                }
                Event::Message { client, val } => {
                    println!("New message from client: {}: {}", client, val)
                }
                Event::ClientQuit { client } => {
                    println!("Client with id: {} quit", client)
                }
            }
        }
    }

    fn send(&self, event: Event) -> Result<()> {
        match self.tx.lock().unwrap().send(event) {
            Ok(()) => Ok(()),
            Err(_) => Result::Err(Error::new(io::ErrorKind::Other, "send failed")),
        }
    }

    fn handle_client(&self, id: usize, stream: TcpStream) -> Result<()> {
        let (tx, rx) = mpsc::channel();
        let outgoing = Arc::clone(&self.tx);
        let incoming = Arc::new(Mutex::new(rx));
        let client = Client::new(id, stream, outgoing, incoming);
        let tx = Arc::new(Mutex::new(tx));
        self.send(Event::NewClient {
            client: id,
            outgoing: tx,
        })?;
        let res = client.run();
        self.send(Event::ClientQuit { client: id })?;
        res
    }
}

#[derive(Debug, Clone)]
struct Client {
    id: usize,
    writer: Arc<Mutex<BufWriter<TcpStream>>>,
    reader: Arc<Mutex<BufReader<TcpStream>>>,
    tx: OutgoingEvents,
    rx: IncomingEvents,
}

impl Client {
    fn new(id: usize, stream: TcpStream, tx: OutgoingEvents, rx: IncomingEvents) -> Client {
        let reader = BufReader::new(stream.try_clone().unwrap());
        let writer = BufWriter::new(stream);
        let reader = Arc::new(Mutex::new(reader));
        let writer = Arc::new(Mutex::new(writer));
        Client {
            id,
            reader,
            writer,
            tx,
            rx,
        }
    }

    fn run(&self) -> Result<()> {
        println!("Client {} running...", self.id);
        let (tx, rx) = mpsc::channel();

        let client = self.clone();
        let ttx = tx.clone();
        thread::spawn(move || {
            let _ = client.read();
            ttx.send(true);
        });

        let client = self.clone();
        let ttx = tx.clone();
        thread::spawn(move || {
            let _ = client.write();
            ttx.send(true);
        });

        drop(tx);
        rx.recv();
        Ok(())
    }

    fn read(&self) -> Result<()> {
        loop {
            let mut buf = String::new();
            self.reader.lock().unwrap().read_line(&mut buf)?;
            buf = buf.trim().to_string();
            self.tx.lock().unwrap().send(Event::Message {
                client: self.id,
                val: buf,
            });
        }
    }

    fn write(&self) -> Result<()> {
        loop {
            let event = self.rx.lock().unwrap().recv().unwrap();
            if let Event::Message { client, val } = event {
                let msg = format!("{}: {}\n", client, val);
                self.writer.lock().unwrap().write_all(msg.as_bytes())?;
            }
        }
    }
}

impl fmt::Display for Client {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "client:{}", self.id)
    }
}

#[derive(Debug)]
struct Quit {
    name: String,
    res: Result<()>,
    failure: bool,
}

impl Quit {
    fn from(name: &str, res: Result<()>) -> Quit {
        let name = name.to_string();
        let failure = res.is_err();
        Quit { name, failure, res }
    }
}

impl From<Quit> for Result<()> {
    fn from(value: Quit) -> Self {
        value.res
    }
}

impl fmt::Display for Quit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.failure {
            write!(f, "{} quit unexpectedly: {:?}", self.name, self.res)
        } else {
            write!(f, "{} quit", self.name)
        }
    }
}
