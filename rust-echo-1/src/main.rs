use std::{
    io::{BufRead, BufReader, BufWriter, Error, Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

fn main() {
    println!("Starting server!");
    match server() {
        Ok(_) => {}
        Err(e) => println!("Error: {}", e),
    }
}

fn server() -> Result<(), Error> {
    let l = TcpListener::bind("0.0.0.0:3000")?;
    loop {
        let (stream, _addr) = l.accept()?;
        thread::spawn(move || {
            let mut echo = Echo::from(stream);
            echo.handle()
        });
    }
}

// marker trait
trait ReadWrite: Read + Write {}
impl<T: Read + Write> ReadWrite for T {}

struct Echo<RW: ReadWrite> {
    reader: BufReader<RW>,
    writer: BufWriter<RW>,
}

impl<RW: ReadWrite> Echo<RW> {
    fn handle(&mut self) -> Result<(), Error> {
        loop {
            let mut line = String::new();
            self.reader.read_line(&mut line)?;
            line = line.trim_end_matches("\n").to_string();
            self.writer.write(line.as_bytes())?;
            self.writer.write(b"\n")?;
            self.writer.flush()?;
        }
    }
}

impl<RW: ReadWrite> Drop for Echo<RW> {
    fn drop(&mut self) {
        println!("Connection closed.");
    }
}

impl From<TcpStream> for Echo<TcpStream> {
    fn from(stream: TcpStream) -> Self {
        let reader = BufReader::new(stream.try_clone().unwrap());
        let writer = BufWriter::new(stream);
        Echo { reader, writer }
    }
}
