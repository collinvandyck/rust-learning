use std::{
    io,
    net::{TcpListener, TcpStream},
    sync::mpsc::{self, Receiver, RecvError, SendError, Sender},
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

    // starts background tasks and listens for incoming conns
    pub fn run(&self) -> Result<(), ServerError> {
        let (tx, rx) = mpsc::channel();

        let server = self.clone();
        thread::spawn(move || server.control(rx));

        let addr = format!("0.0.0.0:{}", self.port);
        let listener = TcpListener::bind(addr)?;
        for stream in listener.incoming() {
            let stream = stream?;
            let msg = Message::Stream(stream);
            let _ = self.send(tx.clone(), msg)?;
        }
        Ok(())
    }

    // the main control loop
    pub fn control(&self, rx: Receiver<Event>) -> Result<(), ServerError> {
        loop {
            let Event(msg, sender) = rx.recv()?;
            dbg!(msg);
            sender.send(Ok(()))?;
        }
    }

    fn send(&self, tx: Sender<Event>, msg: Message) -> Result<(), ServerError> {
        let (etx, erx) = mpsc::channel();
        let event = Event(msg, etx);
        tx.send(event)?;
        let _ = erx.recv()?;
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("accept failed")]
    Accept(#[from] io::Error),

    #[error("rx chan closed")]
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
