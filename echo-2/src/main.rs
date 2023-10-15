use anyhow::{Context, Result};
use clap::Parser;
use std::{
    io::{self, Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    process, thread,
    time::Duration,
};
use tracing::{debug, info, Level};

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_test::traced_test;

    #[test]
    #[traced_test]
    fn test_read_buffer() {
        let mut input = "Collin".as_bytes();
        let mut buf = vec![0_u8; 2];
        let n = input.read(&mut buf).unwrap();
        assert_eq!(n, 2);
        assert_eq!(&buf, "Co".as_bytes());
        let n = input.read(&mut buf).unwrap();
        assert_eq!(n, 2);
        assert_eq!(&buf, "ll".as_bytes());
        let n = input.read(&mut buf).unwrap();
        assert_eq!(n, 2);
        assert_eq!(&buf, "in".as_bytes());
        let n = input.read(&mut buf).unwrap();
        assert_eq!(n, 0);
        assert_eq!(&buf, "in".as_bytes());
    }

    #[test]
    #[traced_test]
    fn test_echo() {
        let runner = Runner::new(None).unwrap();
        let addr = runner.addr.clone();
        thread::spawn(move || {
            runner.run().expect("runner failed");
        });
        let stream = TcpStream::connect(&addr).unwrap();
        let timeout = Some(Duration::from_millis(200));
        let peer_addr = stream.peer_addr().unwrap();
        let local_addr = stream.local_addr().unwrap();
        debug!("Connected to {} from {}", peer_addr, local_addr);
        let mut stream = Stream::new(stream, timeout).unwrap();
        stream.write("Hello").unwrap();
        stream.write(", World!\n").unwrap();
        loop {
            stream.poll().unwrap();
            if let Some(s) = stream.next_str().unwrap() {
                assert_eq!(s, String::from("Hello, World!\n"));
                info!("Got hello world");
                break;
            }
        }
        stream.write("foobar\n").unwrap();
        loop {
            stream.poll().unwrap();
            if let Some(s) = stream.next_str().unwrap() {
                assert_eq!(s, String::from("foobar\n"));
                break;
            }
        }
    }
}

#[derive(clap::Parser, Debug)]
struct Args {
    #[arg(short, default_value_t = 8000)]
    port: u32,
}

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();
    if let Err(err) = run() {
        eprintln!("{err:?}");
        process::exit(1);
    }
}

fn run() -> Result<()> {
    let args = Args::parse();
    let runner = Runner::new(Some(args.port))?;
    info!("Local addr: {:?}", runner.addr);
    runner.run()?;
    Ok(())
}

struct Runner {
    addr: SocketAddr,
    listener: TcpListener,
}

impl Runner {
    fn new(port: Option<u32>) -> Result<Self> {
        let bind = format!("0.0.0.0:{}", port.unwrap_or_default());
        let listener = TcpListener::bind(&bind).context("could not bind")?;
        let addr = listener.local_addr()?;
        let res = Self { addr, listener };
        Ok(res)
    }

    fn run(&self) -> Result<()> {
        loop {
            let (stream, addr) = self.listener.accept().context("accept failure")?;
            info!("Client addr: {addr}");
            thread::spawn(move || {
                if let Err(err) = Self::handle(stream) {
                    eprintln!("Handle: {err:?}");
                }
            });
        }
    }

    fn handle(stream: TcpStream) -> Result<()> {
        let timeout = Some(Duration::from_millis(200));
        let mut stream = Stream::new(stream, timeout)?;
        loop {
            stream.poll()?;
            if let Some(s) = stream.next_str()? {
                stream.write(&s)?;
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
enum LineStreamError {
    #[error("Stream closed")]
    Closed,
    #[error("IO error: {0}")]
    IO(#[from] io::Error),
}

struct Stream {
    stream: TcpStream,
    tmp: Vec<u8>,
    read: Vec<u8>,
    write: Vec<u8>,
}

impl Stream {
    fn new(stream: TcpStream, timeout: Option<Duration>) -> Result<Stream> {
        let tmp = vec![0_u8; 4096];
        let read = vec![];
        let write = vec![];
        let mut res = Stream {
            stream,
            tmp,
            read,
            write,
        };
        res.set_timeout(timeout)?;
        Ok(res)
    }

    fn set_timeout(&mut self, timeout: Option<Duration>) -> Result<()> {
        self.stream.set_read_timeout(timeout)?;
        self.stream.set_write_timeout(timeout)?;
        Ok(())
    }

    fn write(&mut self, input: impl AsRef<[u8]>) -> Result<()> {
        self.write.write_all(input.as_ref()).map_err(|e| e.into())
    }

    fn next_str(&mut self) -> Result<Option<String>> {
        if let Some(n) = self.read.iter().position(|b| b == &b'\n') {
            let res = String::from_utf8_lossy(&self.read[0..=n]).to_string();
            self.read.drain(0..=n);
            Ok(Some(res))
        } else {
            Ok(None)
        }
    }

    fn poll(&mut self) -> Result<()> {
        if !self.write.is_empty() {
            let n = Self::res_to_option(self.stream.write(&self.write))?;
            if let Some(n) = n {
                debug!("Wrote {n} bytes");
                self.write.drain(0..n);
            }
        }
        let n = Self::res_to_option(self.stream.read(&mut self.tmp))?;
        if let Some(n) = n {
            debug!("Read {n} bytes");
            self.read
                .write_all(&self.tmp[0..n])
                .map_err(LineStreamError::IO)?;
        }
        Ok(())
    }

    fn res_to_option(res: std::result::Result<usize, io::Error>) -> Result<Option<usize>> {
        let n = match res {
            Ok(0) => return Err(LineStreamError::Closed.into()),
            Ok(n) => Some(n),
            Err(err) => match err.kind() {
                io::ErrorKind::WouldBlock => None,
                _ => return Err(LineStreamError::IO(err).into()),
            },
        };
        Ok(n)
    }
}
