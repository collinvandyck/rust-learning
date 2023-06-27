use std::{
    io::{self, BufRead, BufReader, BufWriter, Write},
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
            let _ = send_event(&tx.clone(), msg)?;
        }
        Ok(())
    }

    // the main control loop
    pub fn control(&self, tx: Sender<Event>, events: Receiver<Event>) -> Result<(), ServerError> {
        let mut state: Vec<Client> = vec![];
        loop {
            let Event(mut msg, reply) = events.recv()?;
            reply.send(Ok(()))?;
            msg = dbg!(msg);
            match msg {
                Message::Conn(id, stream) => {
                    let tx = tx.clone();
                    match self.new_client(id, stream, tx) {
                        Ok(client) => state.push(client),
                        Err(e) => println!("New client error: {}", e),
                    }
                }
                Message::Chat(id, line) => {
                    println!("Chat message from {}: {}", id, line);
                    state.iter().for_each(|c| {
                        let msg = Message::Chat(id, line.clone());
                        let tx = c.tx.clone();
                        if send_event(&tx, msg).is_err() {
                            println!("Failed to send chat message to {}", c.id);
                        }
                    });
                }
            }
        }
    }

    // handles an incoming connection. tx is used to send messages back to the control loop.
    // it should return a receiver<Event> that is used to send messages to the client.
    fn new_client(
        &self,
        id: usize,
        stream: TcpStream,
        control: Sender<Event>,
    ) -> Result<Client, ServerError> {
        let (tx, rx) = mpsc::channel();
        let client = Client::new(stream, id, control, tx, rx)?;
        Ok(client)
    }
}

// sends a message to the sender, and waits for an ack. an error will be returned
// if the send or the ack fails.
fn send_event(tx: &Sender<Event>, msg: Message) -> Result<(), ServerError> {
    let (etx, erx) = mpsc::channel();
    let event = Event(msg, etx);
    println!("Sending event");
    tx.send(event)?;
    println!("Receiving ack");
    let _ = erx.recv()?;
    println!("Received ack");
    Ok(())
}

struct Client {
    id: usize,         // the id
    tx: Sender<Event>, // send messages to the client
}

impl Client {
    fn new(
        stream: TcpStream,
        id: usize,
        control: Sender<Event>,
        tx: Sender<Event>,
        incoming: Receiver<Event>,
    ) -> Result<Client, ServerError> {
        let mut reader = BufReader::new(stream.try_clone()?);
        let mut writer = BufWriter::new(stream);

        // spawn the thing that will read messages from the control loop
        thread::spawn(move || {
            loop {
                if let Ok(Event(msg, reply)) = incoming.recv() {
                    if reply.send(Ok(())).is_err() {
                        break;
                    }
                    match msg {
                        Message::Chat(id, line) => {
                            let msg = format!("{}: {}\n", id, line);
                            if writer
                                .write(msg.as_bytes())
                                .and_then(|_| writer.flush())
                                .is_err()
                            {
                                break;
                            }
                            println!("Sending chat message to {}: {}", id, line);
                        }
                        _ => {}
                    }
                } else {
                    break;
                }
            }
            println!("Client {} write loop exiting", id);
        });

        // spawn the thing that will read from the tcp socket
        let read_messages = control.clone();
        thread::spawn(move || {
            loop {
                let mut buf = String::new();
                if reader.read_line(&mut buf).is_err() {
                    break;
                }
                buf = buf.trim().to_string();
                if send_event(&read_messages, Message::Chat(id, buf)).is_err() {
                    break;
                }
            }
            println!("Client {} read loop exiting", id);
        });
        Ok(Client { id, tx })
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
    Chat(usize, String),
}
