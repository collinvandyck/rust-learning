use std::sync::{
    mpsc::{self, Receiver, Sender},
    Arc, Mutex,
};

#[derive(Clone)]
pub struct SharedReceiver<T>(pub Arc<Mutex<Receiver<T>>>);

impl<T> Iterator for SharedReceiver<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let guard = self.0.lock().unwrap();
        guard.recv().ok()
    }
}

pub fn shared_channel<T>() -> (Sender<T>, SharedReceiver<T>) {
    let (tx, rx) = mpsc::channel();
    let rx = SharedReceiver(Arc::new(Mutex::new(rx)));
    (tx, rx)
}
