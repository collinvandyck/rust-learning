use std::{
    net::TcpStream,
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

pub fn start() -> Sender<TcpStream> {
    let (tx, rx) = mpsc::channel();
    let _ = thread::spawn(move || run(rx));
    tx
}

fn run(rx: Receiver<TcpStream>) {
    for conn in rx.iter() {
        dbg!(conn);
    }
}
