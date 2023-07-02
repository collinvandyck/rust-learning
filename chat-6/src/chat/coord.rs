use std::{
    net::TcpStream,
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

pub struct Coordinator {
    rx: Receiver<TcpStream>,
}

impl Coordinator {
    pub fn start() -> Sender<TcpStream> {
        let (tx, rx) = mpsc::channel();
        let coordinator = Self { rx };
        let _ = thread::spawn(move || coordinator.run());
        tx
    }

    fn run(&self) {
        for conn in self.rx.iter() {
            dbg!(conn);
        }
    }
}
