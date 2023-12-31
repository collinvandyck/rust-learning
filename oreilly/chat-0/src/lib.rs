use std::{
    io::{BufRead, BufReader, BufWriter, Error, Write},
    net::{TcpListener, TcpStream},
    sync::mpsc::{self, Receiver, Sender, SyncSender},
    time::SystemTime,
};

use chrono::Duration;

#[derive(Clone)]
pub struct Server {
    port: usize,
    now: SystemTime,
}

impl Server {
    pub fn new(port: usize) -> Server {
        let now = SystemTime::now();
        Server { port, now }
    }

    pub fn run(&mut self) -> Result<(), Error> {
        let listener = self.listener()?;
        let (tx, _rx): (SyncSender<Message>, Receiver<Message>) = mpsc::sync_channel(1);
        loop {
            let (stream, addr) = listener.accept()?;
            println!("{}: Connected to {}", self.since(), addr);
            tx.send(Message {
                value: format!("{} connected", addr),
            })
            .unwrap();
            let tx = tx.clone();
            let mut conn = Conn::new(stream, tx);
            conn.run();
        }
    }

    fn listener(&self) -> Result<TcpListener, Error> {
        let addr = format!("0.0.0.0:{}", self.port);
        let listener = TcpListener::bind(addr)?;
        Ok(listener)
    }

    fn since(&mut self) -> String {
        let now = SystemTime::now();
        let since = now.duration_since(self.now).unwrap();
        let chrono_since = Duration::from_std(since).unwrap();
        let res = format!("{}s", chrono_since.num_seconds());
        self.now = SystemTime::now();
        res
    }
}

struct Message {
    value: String,
}

struct Conn {
    io: IO,
}

impl Conn {
    fn new(stream: TcpStream, _out: SyncSender<Message>) -> Conn {
        let io = IO::from_tcp(stream);
        Conn { io }
    }

    fn run(&mut self) {
        match self.run_err() {
            Err(_) => println!("Conn quit."),
            _ => {}
        }
    }

    fn run_err(&mut self) -> Result<(), Error> {
        loop {
            println!("Loop starting...");
            let line = self.io.read_line()?;
            println!("Received: {}", line);
            self.io.write_line(line)?;
        }
    }
}

struct IO {
    reader: BufReader<TcpStream>,
    writer: BufWriter<TcpStream>,
}

impl IO {
    fn from_tcp(stream: TcpStream) -> Self {
        let reader = BufReader::new(stream.try_clone().unwrap());
        let writer = BufWriter::new(stream);
        Self { reader, writer }
    }

    fn write_line(&mut self, val: String) -> Result<(), Error> {
        self.writer.write(val.as_bytes())?;
        self.writer.write(b"\n")?;
        self.writer.flush()?;
        Ok(())
    }

    fn read_line(&mut self) -> Result<String, Error> {
        let mut buf = String::new();
        self.reader.read_line(&mut buf)?;
        let buf = buf.trim();
        Ok(buf.to_string())
    }
}
