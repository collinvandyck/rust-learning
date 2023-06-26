use std::{
    io::{BufRead, BufReader, BufWriter, Error, Write},
    net::{TcpListener, TcpStream},
    sync::{
        mpsc::{self, Receiver, SyncSender},
        Arc, Mutex,
    },
    thread,
};

fn main() {
    Server::start(3000).unwrap();
}

#[derive(Clone)]
struct Server {
    listener: Arc<TcpListener>,
    rx: Arc<Mutex<Receiver<String>>>,
    tx: Arc<Mutex<SyncSender<String>>>,
}

impl Server {
    fn start(port: u32) -> Result<Server, Error> {
        let addr = format!("0.0.0.0:{}", port);
        let listener = TcpListener::bind(addr)?;
        let listener = Arc::new(listener);

        let (tx, rx) = mpsc::sync_channel(1);
        let tx: Arc<Mutex<SyncSender<String>>> = Arc::new(Mutex::new(tx));
        let rx: Arc<Mutex<Receiver<String>>> = Arc::new(Mutex::new(rx));

        let server = Server { listener, rx, tx };
        server.run()?;
        Ok(server)
    }

    fn run(&self) -> Result<(), Error> {
        let server = self.clone();
        thread::spawn(move || server.listen_rx());
        for stream in self.listener.incoming() {
            let stream = stream?;
            let server = self.clone();
            thread::spawn(move || server.handle(stream));
        }
        Ok(())
    }

    // listens for messages from connected clients
    fn listen_rx(&self) {
        loop {
            let val = self.rx.lock().unwrap().recv().unwrap();
            println!("Got val: {}", val);
        }
    }

    fn handle(&self, stream: TcpStream) {
        let reader = BufReader::new(stream.try_clone().unwrap());
        let writer = BufWriter::new(stream);
        let rx = self.handle_read(reader);
        let tx = self.handle_write(writer);
        // loop, reading from the client sending to the server channel
        loop {
            let val = rx.recv().unwrap();
            self.tx.lock().unwrap().send(val).unwrap();
        }
    }

    fn handle_read(&self, mut reader: BufReader<TcpStream>) -> Receiver<String> {
        let (tx, rx): (SyncSender<String>, Receiver<String>) = mpsc::sync_channel(1);
        thread::spawn(move || loop {
            let mut buf = String::new();
            if reader.read_line(&mut buf).is_err() {
                break;
            }
            let buf = buf.trim().to_string();
            if tx.try_send(buf).is_err() {
                break;
            }
        });
        rx
    }

    fn handle_write(&self, mut writer: BufWriter<TcpStream>) -> SyncSender<String> {
        let (tx, rx): (SyncSender<String>, Receiver<String>) = mpsc::sync_channel(1);
        thread::spawn(move || loop {
            let val = match rx.try_recv() {
                Ok(val) => val,
                Err(_) => break,
            };
            if writer.write(val.as_bytes()).is_err() {
                break;
            }
            if writer.write(b"\n").is_err() {
                break;
            }
            if writer.flush().is_err() {
                break;
            }
        });
        tx
    }
}
