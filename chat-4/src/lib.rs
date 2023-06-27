use std::{
    fmt::{Display, Formatter},
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
            let _ = send_event(tx.clone(), msg)?;
        }
        Ok(())
    }

    // the main control loop
    pub fn control(&self, tx: Sender<Event>, events: Receiver<Event>) -> Result<(), ServerError> {
        let mut state = State::new();
        loop {
            println!("Waiting for event, state: {}", state);
            let Event(mut msg, reply) = events.recv()?;
            reply.send(Ok(()))?;
            msg = dbg!(msg);
            match msg {
                Message::Conn(id, stream) => {
                    let tx = tx.clone();
                    match self.new_client(id, stream, tx) {
                        Ok(client) => state.add_client(client),
                        Err(e) => println!("New client error: {}", e),
                    }
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
        let client = Client::new(stream, id, control, tx, rx);
        Ok(client)
    }
}

// sends a message to the sender, and waits for an ack. an error will be returned
// if the send or the ack fails.
fn send_event(tx: Sender<Event>, msg: Message) -> Result<(), ServerError> {
    let (etx, erx) = mpsc::channel();
    let event = Event(msg, etx);
    println!("Sending event");
    tx.send(event)?;
    println!("Receiving ack");
    let _ = erx.recv()?;
    println!("Received ack");
    Ok(())
}

struct State {
    clients: Vec<Client>,
}

impl State {
    fn new() -> Self {
        let clients = Vec::new();
        Self { clients }
    }

    fn add_client(&mut self, client: Client) {
        self.clients.push(client);
    }
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "State {{ clients: {} }}", self.clients.len())
    }
}

struct Client {
    id: usize,              // the id
    control: Sender<Event>, // send events to the control loop
    tx: Sender<Event>,      // send messages to the client
    rx: Receiver<Event>,    // client will receive message from the control loop
}

impl Client {
    fn new(
        stream: TcpStream,
        id: usize,
        control: Sender<Event>,
        tx: Sender<Event>,
        rx: Receiver<Event>,
    ) -> Self {
        Self {
            id,
            control,
            tx,
            rx,
        }
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
