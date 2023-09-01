use std::time::Duration;

use anyhow::Result;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let id = std::process::id();
    println!("pid: {id}");
    loop {
        tokio::select! {
            sig = sig::signals() => {
                match sig {
                    Ok(sig) => {
                        println!("Got sig: {sig:?}");
                    }
                    Err(e) => {
                        eprintln!("Could not listen for signals: {e}");
                        sleep(Duration::from_secs(1)).await;
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
enum Signal {
    HUP,
    INT,
    TERM,
}

#[cfg(not(windows))]
pub(crate) mod sig {
    use super::*;
    use tokio::signal::unix::{signal, SignalKind};
    pub(crate) async fn signals() -> Result<Signal> {
        let mut sig_hup = signal(SignalKind::hangup())?;
        let mut sig_int = signal(SignalKind::interrupt())?;
        let mut sig_term = signal(SignalKind::terminate())?;
        loop {
            tokio::select! {
                _ = sig_hup.recv() => return Ok(Signal::HUP),
                _ = sig_int.recv() => return Ok(Signal::INT),
                _ = sig_term.recv() => return Ok(Signal::TERM),
            }
        }
    }
}

#[cfg(windows)]
pub(crate) mod sig {
    use super::*;
    use tokio::signal::unix::{signal, SignalKind};
    pub(crate) async fn signals() -> Result<Signal> {
        let mut sig_hup = signal(SignalKind::hangup())?;
        let mut sig_int = signal(SignalKind::interrupt())?;
        let mut sig_term = signal(SignalKind::terminate())?;
        loop {
            tokio::select! {
                _ = sig_hup.recv() => return Ok(Signal::HUP),
                _ = sig_int.recv() => return Ok(Signal::INT),
                _ = sig_term.recv() => return Ok(Signal::TERM),
            }
        }
    }
}
