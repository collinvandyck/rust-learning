use std::{
    io::{BufRead, BufReader, BufWriter, Error, Write},
    net::{TcpListener, TcpStream},
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
        let addr = format!("0.0.0.0:{}", self.port);
        println!("{}: Listening on {}", self.since(), addr);
        let listener = TcpListener::bind(addr)?;
        loop {
            let (stream, addr) = listener.accept()?;
            let now = self.since();
            println!("{}: Connected to {}", now, addr);
            let mut conn = Conn::new(stream);
            conn.run();
        }
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

struct Conn {
    io: IO,
}

impl Conn {
    fn new(stream: TcpStream) -> Conn {
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
