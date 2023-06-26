use std::{
    io,
    net::{TcpListener, TcpStream},
    sync::mpsc::{self, RecvError, SendError, Sender},
    thread,
};
use thiserror::Error;

#[derive(Clone)]
pub struct Server {
    port: u32,
}

impl Server {
    pub fn new(port: u32) -> Self {
        Self { port }
    }

    // the main control thread
    pub fn run(&self) -> Result<(), ServerError> {
        let (tx, rx) = mpsc::channel();

        let server = self.clone();
        let tx1 = tx.clone();
        thread::spawn(move || server.accept(tx1));

        drop(tx);
        loop {
            let Event(msg, sender) = rx.recv()?;
            dbg!(msg);
            sender.send(Ok(()))?;
        }
    }

    pub fn accept(&self, tx: Sender<Event>) -> Result<(), ServerError> {
        let addr = format!("0.0.0.0:{}", self.port);
        let listener = TcpListener::bind(addr)?;
        for stream in listener.incoming() {
            let stream = stream?;
            let (etx, erx) = mpsc::channel();
            let msg = Message::Stream(stream);
            let event = Event(msg, etx);
            tx.send(event)?;
            let _ = erx.recv()?;
        }
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("accept failed")]
    Accept(#[from] io::Error),

    #[error("chan send failure")]
    SendFailure(#[from] mpsc::SendError<Event>),

    #[error("tx chan closed")]
    TxClosed(#[from] RecvError),

    #[error("could not send ack")]
    AckFailed(#[from] SendError<Result<(), RecvError>>),
}

pub struct Event(Message, Sender<Result<(), RecvError>>);

#[derive(Debug)]
pub enum Message {
    Stream(TcpStream),
}
