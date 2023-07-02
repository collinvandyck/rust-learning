use std::io::Write;
use std::{
    io::{BufRead, BufReader, BufWriter},
    net::TcpStream,
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

enum Event {
    Client(Client),
    Line(usize, String),
    Quit(usize),
}

pub fn start() -> Sender<TcpStream> {
    let (tx_conn, rx_conn) = mpsc::channel();
    let (tx_event, rx_event) = mpsc::channel();
    let _ = thread::spawn(move || receive_conns(rx_conn, tx_event));
    let _ = thread::spawn(move || receive_events(rx_event));
    tx_conn
}

fn receive_events(rx: Receiver<Event>) {
    let mut clients = vec![];
    for event in rx {
        match event {
            Event::Client(client) => {
                clients.push(client);
            }
            Event::Line(id, line) => {
                clients.retain(|client| {
                    if client.id == id {
                        return true;
                    }
                    let event = Event::Line(id, line.clone());
                    client.tx.send(event).is_ok()
                });
            }
            Event::Quit(id) => {
                clients.retain(|client| client.id != id);
            }
        }
    }
}

fn receive_conns(rx: Receiver<TcpStream>, tx_event: Sender<Event>) {
    for (id, conn) in rx.iter().enumerate() {
        let tx_event_client = tx_event.clone();
        let client = new_client(id, conn, tx_event_client);
        let tx_res = tx_event.send(Event::Client(client));
        if tx_res.is_err() {
            break;
        }
    }
}

struct Client {
    id: usize,
    tx: Sender<Event>,
}

fn new_client(id: usize, conn: TcpStream, tx_event: Sender<Event>) -> Client {
    let (tx, rx) = mpsc::channel();

    // read from the conn
    let reader = BufReader::new(conn.try_clone().unwrap());
    thread::spawn(move || {
        for line in reader.lines() {
            if let Ok(line) = line {
                let res = tx_event.send(Event::Line(id, line));
                if res.is_err() {
                    break;
                }
            } else {
                tx_event.send(Event::Quit(id)).unwrap();
                break;
            }
        }
    });

    // read events from the coordinator.
    thread::spawn(move || {
        let mut writer = BufWriter::new(conn);
        for event in rx {
            if let Event::Line(id, line) = event {
                let msg = format!("{}: {}\n", id, line);
                let res = writer
                    .write_all(msg.as_bytes())
                    .and_then(|_| writer.flush());
                if res.is_err() {
                    break;
                }
            }
        }
    });

    Client { id, tx }
}
