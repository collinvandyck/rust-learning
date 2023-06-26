use std::{
    sync::mpsc::{self, Receiver, RecvError, SendError, Sender},
    thread, vec,
};

fn main() {
    run().unwrap()
}

#[derive(Debug)]
enum Error {
    ChanClosed(String),
    Stopped(),
}

impl<T> From<SendError<T>> for Error {
    fn from(value: SendError<T>) -> Self {
        let s = value.to_string();
        Error::ChanClosed(s)
    }
}

impl From<RecvError> for Error {
    fn from(value: RecvError) -> Self {
        let s = value.to_string();
        Error::ChanClosed(s)
    }
}

type Res<T> = Result<T, Error>;

#[derive(Debug, Clone)]
enum Message {
    Register { id: i32, tx: Sender<Message> },
    New { id: i32, val: String },
}

struct Channel {
    tx: Option<Sender<Message>>,
    rx: Option<Receiver<Message>>,
}

impl Channel {
    fn from_tx_rx(tx: Sender<Message>, rx: Receiver<Message>) -> Self {
        Self {
            tx: Some(tx),
            rx: Some(rx),
        }
    }
    fn from_rx(rx: Receiver<Message>) -> Self {
        Self {
            tx: None,
            rx: Some(rx),
        }
    }
    fn send(&self, msg: Message) -> Res<()> {
        match &self.tx {
            Some(tx) => Ok(tx.send(msg)?),
            None => Ok(()),
        }
    }
    fn recv(&self) -> Res<Message> {
        match &self.rx {
            Some(rx) => Ok(rx.recv()?),
            None => Err(Error::ChanClosed("Channel closed".to_string())),
        }
    }
}

fn run() -> Res<()> {
    let (server_tx, server_rx) = mpsc::channel();
    let server = thread::spawn(move || {
        let _ = server(Channel::from_rx(server_rx));
    });
    for i in 1..5 {
        let tx = server_tx.clone();
        let (client_tx, rx) = mpsc::channel();
        server_tx.send(Message::Register {
            id: i,
            tx: client_tx,
        })?;
        let rw = Channel::from_tx_rx(tx, rx);
        thread::spawn(move || {
            let _ = client(i, rw);
        });
    }
    let res = server.join();
    res.map_err(|_| Error::Stopped())
}

fn server(rw: Channel) -> Res<()> {
    let mut clients = vec![];
    loop {
        let msg = rw.recv()?;
        match msg {
            Message::Register { id, tx } => {
                println!("New client with id: {}", id);
                clients.push(tx);
            }
            Message::New { id, val } => {
                println!("{} sent: {}", id, val);
                let msg = Message::New {
                    id,
                    val: val.to_string(),
                };
                clients = clients
                    .into_iter()
                    .filter(|c| c.send(msg.clone()).is_ok())
                    .collect::<Vec<_>>();
            }
        }
    }
}

fn client(id: i32, rw: Channel) -> Res<()> {
    let msg = Message::New {
        id,
        val: String::from("Hello"),
    };
    rw.send(msg)?;
    loop {
        let msg = rw.recv()?;
        println!("{} got msg: {:?}", id, msg);
    }
}
