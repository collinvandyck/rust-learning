use std::{
    io::{BufRead, BufReader, BufWriter, Error, Write},
    net::{TcpListener, TcpStream},
    sync::Arc,
    thread,
};

#[derive(Clone)]
pub struct Server {
    func: Arc<Box<dyn Fn(String) -> String + Send + Sync>>,
}

impl Server {
    pub fn new(func: Box<dyn Fn(String) -> String + Send + Sync>) -> Self {
        Self {
            func: Arc::new(func),
        }
    }

    pub fn run(&self) -> Result<(), Error> {
        let l = TcpListener::bind("0.0.0.0:3000")?;
        loop {
            let (stream, _addr) = l.accept()?;
            let server = Arc::new(self.clone());
            thread::spawn(move || server.handle(stream));
        }
    }

    fn handle(&self, stream: TcpStream) -> Result<(), Error> {
        let mut reader = BufReader::new(&stream);
        let mut writer = BufWriter::new(&stream);
        loop {
            let mut line = String::new();
            reader.read_line(&mut line)?;
            line = line.trim().to_string();
            println!("{}", line);
            line = (self.func)(line.to_string());
            println!("{}", line);
            writer.write(line.as_bytes())?;
            writer.write(b"\n")?;
            writer.flush()?;
        }
    }
}
