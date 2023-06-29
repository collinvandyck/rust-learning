use std::{
    io::{self, BufRead, BufReader, BufWriter},
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

    fn handle(&self, mut stream: TcpStream) {
        stream = dbg!(stream);
        let reader = BufReader::new(stream.try_clone().unwrap());
        let writer = BufWriter::new(stream);
        let mut rw = ReaderWriter::new(reader, writer);
        rw.write("hello there!").expect("welp");
    }
}

struct ReaderWriter<R, W> {
    reader: R,
    writer: W,
}

impl<R, W> ReaderWriter<R, W>
where
    R: BufRead,
    W: io::Write,
{
    fn new(reader: R, writer: W) -> Self {
        Self { reader, writer }
    }

    fn write(&mut self, val: &str) -> io::Result<()> {
        self.writer.write_all(val.as_bytes())?;
        self.writer.write_all(b"\n")?;
        self.writer.flush()?;
        Ok(())
    }
}

trait Reader {
    fn next(&mut self) -> io::Result<String>;
}

impl<T: BufRead> Reader for T {
    fn next(&mut self) -> io::Result<String> {
        let mut buf = String::new();
        buf = self.read_line(&mut buf).map(|_| buf)?;
        buf = buf.trim().to_string();
        Ok(buf)
    }
}

trait Writer {
    fn send(&mut self, val: &str) -> io::Result<()>;
}

impl<T: io::Write> Writer for T {
    fn send(&mut self, val: &str) -> io::Result<()> {
        self.write(val.as_bytes())?;
        self.write(b"\n")?;
        self.flush()?;
        Ok(())
    }
}
