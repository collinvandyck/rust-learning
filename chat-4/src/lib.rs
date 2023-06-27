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
        let tx2 = tx.clone();
        thread::spawn(move || server.control(tx2, rx));

        let addr = format!("0.0.0.0:{}", self.port);
        let listener = TcpListener::bind(addr)?;
        for (id, stream) in listener.incoming().enumerate() {
            let stream = stream?;
            let msg = Message::Conn(id, stream);
            let _ = self.send(tx.clone(), msg)?;
        }
        Ok(())
    }

    // the main control loop
    pub fn control(&self, tx: Sender<Event>, events: Receiver<Event>) -> Result<(), ServerError> {
        loop {
            let Event(mut msg, reply) = events.recv()?;
            msg = dbg!(msg);
            match msg {
                Message::Conn(id, stream) => {
                    // ack that we got the event
                    reply.send(Ok(()))?;
                    let tx = tx.clone();
                    match self.handle_conn(stream, tx) {
                        Ok(rx) => {}
                        Err(e) => {
                            println!("Error handling conn: {}", e);
                        }
                    }
                }
            }
        }
    }

    // handles an incoming connection. tx is used to send messages back to the control loop.
    // it should return a receiver<Event> that is used to send messages to the client.
    fn handle_conn(
        &self,
        _stream: TcpStream,
        _tx: Sender<Event>,
    ) -> Result<Receiver<Event>, ServerError> {
        Err(ServerError::NewClientFailure)
    }

    // sends a message to the sender, and waits for an ack. an error will be returned
    // if the send or the ack fails.
    fn send(&self, tx: Sender<Event>, msg: Message) -> Result<(), ServerError> {
        let (etx, erx) = mpsc::channel();
        let event = Event(msg, etx);
        println!("Sending event");
        tx.send(event)?;
        println!("Receiving ack");
        let _ = erx.recv()?;
        println!("Received ack");
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
    TxClosed {
        #[from]
        source: RecvError,
    },

    #[error("could not send ack")]
    AckFailed(#[from] SendError<Result<(), RecvError>>),

    #[error("Failed to build client")]
    NewClientFailure,
}

pub struct Event(Message, Sender<Result<(), RecvError>>);

#[derive(Debug)]
pub enum Message {
    Conn(usize, TcpStream),
}
