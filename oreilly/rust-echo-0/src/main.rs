use std::io::BufRead;
use std::{
    io::{BufReader, BufWriter, Error, Write},
    net::{TcpListener, TcpStream},
};

struct Echoer {
    stream: TcpStream,
}

impl Echoer {
    fn new(stream: TcpStream) -> Self {
        Self { stream }
    }

    fn handle(stream: TcpStream) -> Result<(), Error> {
        let e = Echoer::new(stream);
        e.run()
    }

    fn run(&self) -> Result<(), Error> {
        let mut reader = BufReader::new(&self.stream);
        let mut writer = BufWriter::new(&self.stream);
        loop {
            let mut line = String::new();
            reader.read_line(&mut line)?;
            line = line.trim_end_matches("\n").to_string();
            println!("Read line: {}", line);
            writer.write(line.as_bytes())?;
            writer.write(b"\n")?;
            writer.flush()?;
        }
    }
}

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
        let (stream, addr) = l.accept()?;
        println!("Connection received on {:?}.", addr);
        std::thread::spawn(move || {
            if Echoer::handle(stream).is_err() {
                println!("Connection to {} closed.", addr);
            }
        });
    }
}
