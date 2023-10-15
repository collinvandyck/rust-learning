use anyhow::{Context, Result};
use clap::Parser;
use std::{
    io::{self, Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    process,
    str::Utf8Error,
    thread,
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
            if let Some(s) = stream.next().unwrap() {
                assert_eq!(s, String::from("Hello, World!\n"));
                info!("Got hello world");
                break;
            }
        }
        stream.write("foobar\n").unwrap();
        loop {
            stream.poll().unwrap();
            if let Some(s) = stream.next().unwrap() {
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
            if let Some(s) = stream.next()? {
                stream.write(&s)?;
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
enum StreamError {
    #[error("Stream closed")]
    Closed,

    #[error("IO error: {0}")]
    IO(#[from] io::Error),

    #[error("Invalid UTF8: {0}")]
    InvalidUTF8(#[from] Utf8Error),
}

/// A non-blocking wrapper around `TcpStream` that buffers input and output.
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

    // writes the supplied data to the buffer but does not send it immediately
    fn write(&mut self, input: impl AsRef<[u8]>) -> Result<()> {
        self.write.write_all(input.as_ref()).map_err(|e| e.into())
    }

    // returns the next line of input read, if it exists.
    fn next(&mut self) -> Result<Option<String>> {
        if let Some(n) = self.read.iter().position(|b| b == &b'\n') {
            let res = std::str::from_utf8(&self.read[0..=n])
                .map_err(StreamError::InvalidUTF8)?
                .to_string();
            self.read.drain(0..=n);
            Ok(Some(res))
        } else {
            Ok(None)
        }
    }

    // attempts to read and write once. should be called in a loop.
    fn poll(&mut self) -> Result<()> {
        if !self.write.is_empty() {
            if let Some(n) = self
                .stream
                .write(&self.write)
                .map_or_else(Self::map_poll_err, Self::map_poll_n)?
            {
                debug!("Wrote {n} bytes");
                self.write.drain(0..n);
            }
        }
        if let Some(n) = self
            .stream
            .read(&mut self.tmp)
            .map_or_else(Self::map_poll_err, Self::map_poll_n)?
        {
            debug!("Read {n} bytes");
            self.read
                .write_all(&self.tmp[0..n])
                .map_err(StreamError::IO)?;
        }
        Ok(())
    }

    fn map_poll_err(err: io::Error) -> Result<Option<usize>> {
        match err.kind() {
            io::ErrorKind::WouldBlock => Ok(None),
            _ => Err(StreamError::IO(err).into()),
        }
    }

    fn map_poll_n(n: usize) -> Result<Option<usize>> {
        if n == 0 {
            Err(StreamError::Closed.into())
        } else {
            Ok(Some(n))
        }
    }
}
