use serde::Serialize;
use serde_json;
use std::{
    collections::HashMap,
    fmt::Debug,
    io::{self, BufRead, BufReader, BufWriter, Write},
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
        let reader = Box::new(BufReader::new(stream.try_clone().unwrap()));
        let reader = Reader::new(reader);
        let writer = Box::new(BufWriter::new(stream));
        let writer = Writer::new(writer);
        self.handle_rw(reader, writer).expect("fail")
    }

    fn handle_rw(&self, mut reader: Reader, mut writer: Writer) -> io::Result<()> {
        writer.write("hello there!")?;
        let line = reader.read_line()?;
        println!("got: {}", line);
        Ok(())
    }

    #[allow(dead_code)]
    fn to_bs(&self) -> io::Result<()> {
        let mut m = HashMap::new();
        m.insert("port", self.port);
        let mut writer = vec![];
        let mut serializer = serde_json::Serializer::new(&mut writer);
        m.serialize(&mut serializer)?;
        dbg!(m);
        Ok(())
    }
}

trait MyIterator: Debug {
    type Item: Debug;

    fn next(&mut self) -> Option<Self::Item>;

    fn foo(&mut self) {
        let b = self.next().unwrap();
        println!("{:?}", b);
        println!("{:?}", self);
        println!("{:?}", self);
    }
}

struct Writer {
    w: Box<dyn Write>,
}

impl Writer {
    fn new(w: Box<dyn Write>) -> Self {
        Self { w }
    }

    fn write(&mut self, val: &str) -> io::Result<()> {
        self.w.write_all(val.as_bytes())?;
        self.w.write_all(b"\n")?;
        self.w.flush()?;
        Ok(())
    }
}

trait Something {
    fn something(&self);
}

trait Visible: Something {
    fn view(&self);
}

impl Visible for BufReader<TcpStream> {
    fn view(&self) {}
}

impl Something for BufReader<TcpStream> {
    fn something(&self) {}
}

struct Reader {
    r: Box<dyn BufRead>,
}

impl Reader {
    fn new(r: Box<dyn BufRead>) -> Self {
        Self { r }
    }

    fn read_line(&mut self) -> io::Result<String> {
        let mut buf = String::new();
        self.r.read_line(&mut buf)?;
        Ok(buf.trim().to_string())
    }
}
