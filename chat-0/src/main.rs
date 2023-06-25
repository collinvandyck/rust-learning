use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

use chat_0::Server;

fn main() {
    let mut server = Server::new(3000);
    server.run().expect("Server died");
}

fn thread_fun() {
    let num_threads = 5;
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    let mut children = Vec::new();
    for id in 0..num_threads {
        let thread_tx = tx.clone();
        let child = thread::spawn(move || {
            thread_tx.send(id).unwrap();
            println!("Thread {} finished", id);
        });
        children.push(child);
    }

    let mut ids = Vec::with_capacity(num_threads as usize);
    for _ in 0..num_threads {
        ids.push(rx.recv())
    }
    for child in children {
        child.join().expect("oops, child panicked");
    }
    println!("{:?}", ids);
}
