use std::{sync::mpsc, thread};

pub trait OffThreadExt: Iterator {
    // transforms this iterator into an off-thread iterator.
    // the next() calls happen on a separate worker thread so
    // the iterator and the body of your loop run concurrently.
    fn off_thread(self) -> mpsc::IntoIter<Self::Item>;
}

impl<T> OffThreadExt for T
where
    T: Iterator + Send + 'static,
    T::Item: Send + 'static,
{
    fn off_thread(self) -> mpsc::IntoIter<Self::Item> {
        // create a channel to transfer items from the worker thread.
        let (tx, rx) = mpsc::sync_channel(1024);

        thread::spawn(move || {
            println!("New thread!");
            for item in self {
                if tx.send(item).is_err() {
                    break;
                }
            }
            println!("Thread done.");
        });

        rx.into_iter()
    }
}

fn main() {
    let nums: Vec<i32> = (1..10).collect();
    nums.into_iter().off_thread().for_each(|i| {
        dbg!(i);
    })
}
