use std::{
    sync::mpsc::{self, Receiver, RecvError, SendError, Sender},
    thread,
};

fn main() {
    run().unwrap()
}

#[derive(Debug)]
enum Error {
    SendFailure(String),
}

impl<T> From<SendError<T>> for Error {
    fn from(value: SendError<T>) -> Self {
        let s = value.to_string();
        Error::SendFailure(s)
    }
}

type Res<T> = Result<T, Error>;

enum Message {
    Client { id: i32, tx: Sender<Message> },
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

    thread::spawn(move || {
        let _ = server(Receive { rx: server_rx });
    });

    for i in 1..5 {
        let tx = server_tx.clone();
        let (client_tx, rx) = mpsc::channel();
        server_tx.send(Message::Client {
            id: i,
            tx: client_tx,
        })?;
        let rw = SendReceive { tx, rx };
        thread::spawn(move || {
            let _ = client(i, rw);
        });
    }
    Ok(())
}

fn server(rx: Receive) -> Res<()> {
    Ok(())
}

fn client(id: i32, rw: SendReceive) -> Res<()> {
    Ok(())
}
