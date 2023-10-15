use anyhow::{Context, Result};
use std::{
    io::{self, Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    process, thread,
    time::Duration,
};
use tracing::{info, Level};

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
    let runner = Runner::new()?;
    info!("Local addr: {:?}", runner.addr);
    runner.run()?;
    Ok(())
}

struct Runner {
    addr: SocketAddr,
    listener: TcpListener,
}

impl Runner {
    fn new() -> Result<Self> {
        let listener = TcpListener::bind("0.0.0.0:0").context("could not bind")?;
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
            if let Some(n) = read {
                if n == 0 {
                    return Ok(());
                }
                buf.write_all(&tmp[0..n]).context("write input")?;
            }
            // see if we have a newline in the buffer
            if let Some(pos) = buf.iter().position(|b| b == &b'\n') {
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
