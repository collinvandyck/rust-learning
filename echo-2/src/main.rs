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
    fn test_echo_stream() {
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
        let mut stream = LineStream::new(stream, timeout).unwrap();
        stream.write_str("Hello").unwrap();
        stream.write_str(", World!\n").unwrap();
        loop {
            stream.poll().unwrap();
            if let Some(s) = stream.next_str().unwrap() {
                assert_eq!(s, String::from("Hello, World!"));
                break;
            }
        }
        stream.write_str("foobar\n").unwrap();
        loop {
            stream.poll().unwrap();
            if let Some(s) = stream.next_str().unwrap() {
                assert_eq!(s, String::from("foobar"));
                break;
            }
        }
    }

    #[test]
    #[traced_test]
    fn test_echo() {
        let runner = Runner::new(None).unwrap();
        let addr = runner.addr.clone();
        thread::spawn(move || {
            runner.run().expect("runner failed");
        });
        let mut stream = TcpStream::connect(&addr).unwrap();
        let timeout = Some(Duration::from_millis(200));
        stream.set_read_timeout(timeout).unwrap();
        let peer_addr = stream.peer_addr().unwrap();
        let local_addr = stream.local_addr().unwrap();
        debug!("Connected to {} from {}", peer_addr, local_addr);

        stream.write_all(b"Hello").unwrap();
        stream.write_all(b", World!\n").unwrap();

        let mut tmp = vec![0_u8; 4096];
        let mut buf = vec![];
        loop {
            match stream.read(&mut tmp) {
                Ok(0) => {
                    debug!("Nothing to read");
                    break;
                }
                Ok(n) => {
                    debug!("Read {n} bytes");
                    buf.write_all(&tmp[0..n]).unwrap();
                }
                Err(err) => match err.kind() {
                    io::ErrorKind::WouldBlock => {
                        debug!("Client would block");
                    }
                    _ => panic!("{err}"),
                },
            }
            if let Some(n) = buf.iter().position(|b| b == &b'\n') {
                debug!("Client found newline at {n} buf={buf:?}");
                let res = String::from_utf8_lossy(&buf[0..n]).to_string();
                assert_eq!(res, String::from("Hello, World!"));
                buf.drain(0..=n);
                debug!("Buf is now {buf:?}");
                assert!(buf.is_empty());
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
                if let Err(err) = Self::handle_stream(stream) {
                    eprintln!("Handle: {err:?}");
                }
            });
        }
    }

    fn handle_stream(stream: TcpStream) -> Result<()> {
        let timeout = Some(Duration::from_millis(200));
        let mut stream = LineStream::new(stream, timeout)?;
        loop {
            stream.poll()?;
            if let Some(s) = stream.next_str()? {
                stream.write_str(&s)?;
            }
        }
    }

    fn handle(stream: TcpStream) -> Result<()> {
        stream
            .set_read_timeout(Some(Duration::from_millis(200)))
            .context("set read timeout")?;
        let (mut tx, mut rx) = (
            stream.try_clone().context("could not clone stream")?,
            stream,
        );
        let mut buf = vec![];
        let mut tmp = vec![0_u8; 4096];
        loop {
            let read = match rx.read(&mut tmp) {
                Ok(n) => Some(n),
                Err(err) => match err.kind() {
                    io::ErrorKind::WouldBlock => None,
                    _ => {
                        return Err(err.into());
                    }
                },
            };
            debug!("Server read {read:?}");
            if let Some(n) = read {
                if n == 0 {
                    return Ok(());
                }
                buf.write_all(&tmp[0..n]).context("write input")?;
            }
            // see if we have a newline in the buffer
            if let Some(pos) = buf.iter().position(|b| b == &b'\n') {
                debug!(pos, "found newline");
                let write = match tx.write(&buf[0..=pos]) {
                    Ok(n) => Some(n),
                    Err(err) => match err.kind() {
                        io::ErrorKind::WouldBlock => None,
                        _ => {
                            return Err(err.into());
                        }
                    },
                };
                if let Some(n) = write {
                    if n == 0 {
                        return Ok(());
                    }
                    buf.drain(0..n);
                }
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

struct LineStream {
    stream: TcpStream,
    tmp: Vec<u8>,
    read: Vec<u8>,
    write: Vec<u8>,
}

impl LineStream {
    fn new(stream: TcpStream, timeout: Option<Duration>) -> Result<LineStream> {
        let tmp = vec![0_u8; 4096];
        let read = vec![];
        let write = vec![];
        let mut res = LineStream {
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

    fn write_str(&mut self, input: impl AsRef<str>) -> Result<()> {
        let s = input.as_ref();
        let bs = s.as_bytes();
        self.write.write_all(bs)?;
        Ok(())
    }

    fn next_str(&mut self) -> Result<Option<String>> {
        if let Some(n) = self.read.iter().position(|b| b == &b'\n') {
            let res = String::from_utf8_lossy(&self.read[0..n]).to_string();
            self.read.drain(0..=n);
            Ok(Some(res))
        } else {
            Ok(None)
        }
    }

    fn poll(&mut self) -> Result<()> {
        if !self.write.is_empty() {
            let n = match self.stream.write(&self.write) {
                Ok(0) => return Err(LineStreamError::Closed.into()),
                Ok(n) => Some(n),
                Err(err) => match err.kind() {
                    io::ErrorKind::WouldBlock => None,
                    _ => return Err(LineStreamError::IO(err).into()),
                },
            };
            if let Some(n) = n {
                self.write.drain(0..n);
            }
        }
        let n = match self.stream.read(&mut self.tmp) {
            Ok(0) => return Err(LineStreamError::Closed.into()),
            Ok(n) => Some(n),
            Err(err) => match err.kind() {
                io::ErrorKind::WouldBlock => None,
                _ => return Err(LineStreamError::IO(err).into()),
            },
        };
        if let Some(n) = n {
            self.read
                .write_all(&self.tmp[0..n])
                .map_err(LineStreamError::IO)?;
        }
        Ok(())
    }
}
