use std::{
    io::{BufRead, BufReader, BufWriter, Error, Read, Write},
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
                if let Err(_) = server.handle(stream, addr) {
                    println!("Client {} closed", addr)
                }
            });
        }
    }

    fn handle(&self, stream: TcpStream, addr: SocketAddr) -> Result<(), Error> {
        println!("Connected to {}", addr);
        let mut io = IO::from_tcp(stream);
        loop {
            let line = io.read_line()?;
            println!("Received: {}", line);
            io.write_line(line)?;
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
