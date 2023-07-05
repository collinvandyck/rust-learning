mod shared_channel;

use std::sync::mpsc::{Receiver, Sender};
pub use std::{
    fmt::{Debug, Display},
    sync::mpsc,
    thread,
};

mod prelude {
    pub use crate::shared_channel::*;
}

use prelude::*;

fn main() {
    let nums: Vec<i32> = (1..10).collect();
    nums.into_iter().off_thread().for_each(|i| {
        dbg!(i);
    });

    let (tx, rx) = mpsc::channel();
    tx.send(42).unwrap();
    let th = thread::spawn(move || rx.into_iter().next());
    assert_eq!(42, th.join().unwrap().unwrap());

    let (tx, rx) = mpsc::channel();
    tx.send(42).unwrap();
    let th = thread::spawn(move || {
        rx.into_iter()
            .off_thread()
            .map(|i| format!("{i}"))
            .off_thread()
            .map(|s| s.parse::<i32>())
            .flatten()
            .off_thread()
            .next()
    });
    assert_eq!(42, th.join().unwrap().unwrap());

    let (tx, rx) = shared_channel::<Message<i32>>();
    for _ in 0..10 {
        let rx = rx.clone();
        thread::spawn(move || {
            for msg in rx {
                println!("Thread processing {msg:?}");
                let val = msg.0;
                msg.reply(val).unwrap();
            }
        });
    }
    let (msg, msgrx) = Message::new(84);
    tx.send(msg).unwrap();
    let received = msgrx.recv().unwrap();
    assert_eq!(84, received);
}

#[derive(Debug, Clone)]
struct Message<T>(T, Sender<T>);

impl<T> Message<T> {
    fn new(val: T) -> (Message<T>, Receiver<T>) {
        let (tx, rx) = mpsc::channel();
        let msg = Message(val, tx);
        (msg, rx)
    }
    fn reply(&self, val: T) -> Result<(), mpsc::SendError<T>> {
        self.1.send(val)
    }
}

pub trait OffThreadExt: Iterator {
    // transforms this iterator into an off-thread iterator.
    // the next() calls happen on a separate worker thread so
    // the iterator and the body of your loop run concurrently.
    fn off_thread(self) -> mpsc::IntoIter<Self::Item>;
}

impl<T> OffThreadExt for T
where
    T: Iterator + Send + 'static,
    T::Item: Send + Display + Debug + 'static,
{
    fn off_thread(self) -> mpsc::IntoIter<Self::Item> {
        // create a channel to transfer items from the worker thread.
        let (tx, rx) = mpsc::sync_channel(1024);

        thread::spawn(move || {
            for item in self {
                println!("New thread processing {item:#?}");
                if tx.send(item).is_err() {
                    break;
                }
            }
            println!("Thread done.");
        });

        rx.into_iter()
    }
}
