use std::{
    collections::HashMap,
    fmt::format,
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

struct SendReceive {
    tx: Sender<Message>,
    rx: Receiver<Message>,
}

struct Receive {
    rx: Receiver<Message>,
}

fn run() -> Res<()> {
    let (server_tx, server_rx) = mpsc::channel();
    let server = thread::spawn(move || {
        let _ = server(Receive { rx: server_rx });
    });
    for i in 1..5 {
        let tx = server_tx.clone();
        let (client_tx, rx) = mpsc::channel();
        server_tx.send(Message::Register {
            id: i,
            tx: client_tx,
        })?;
        let rw = SendReceive { tx, rx };
        thread::spawn(move || {
            let _ = client(i, rw);
        });
    }
    let res = server.join();
    res.map_err(|_| Error::Stopped())
}

fn server(rx: Receive) -> Res<()> {
    let mut clients = vec![];
    loop {
        let msg = rx.rx.recv()?;
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

fn client(id: i32, rw: SendReceive) -> Res<()> {
    let msg = Message::New {
        id,
        val: String::from("Hello"),
    };
    rw.tx.send(msg)?;
    loop {
        let msg = rw.rx.recv()?;
        println!("{} got msg: {:?}", id, msg);
    }
}
