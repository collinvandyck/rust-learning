use std::io::{Read, Write};
use std::{
    io::{BufRead, BufReader, BufWriter},
    net::TcpStream,
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

enum Event {
    NewClient(Client),
    Line { client: usize, line: String },
}

pub fn start() -> Sender<TcpStream> {
    let (tx_conn, rx_conn) = mpsc::channel();
    let (tx_event, rx_event) = mpsc::channel();
    let tx_event_2 = tx_event.clone();
    let _ = thread::spawn(move || receive_conns(rx_conn, tx_event));
    let _ = thread::spawn(move || receive_events(rx_event, tx_event_2));
    tx_conn
}

fn receive_events(rx: Receiver<Event>, tx: Sender<Event>) {
    let mut clients = vec![];
    for event in rx {
        match event {
            Event::NewClient(client) => {
                clients.push(client);
            }
            Event::Line {
                client: client_id,
                line,
            } => {
                for client in &clients {
                    let res = client.tx.send(Event::Line {
                        client: client_id,
                        line: line.clone(),
                    });
                    if res.is_err() {
                        println!("Could not send event to client with id: {}", client_id);
                    }
                }
            }
        }
    }
}

fn receive_conns(rx: Receiver<TcpStream>, tx_event: Sender<Event>) {
    for (id, conn) in rx.iter().enumerate() {
        let tx_event_client = tx_event.clone();
        let client = new_client(id, conn, tx_event_client);
        let tx_res = tx_event.send(Event::NewClient(client));
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
            match line {
                Ok(line) => {
                    let res = tx_event.send(Event::Line { client: id, line });
                    if res.is_err() {
                        break;
                    }
                }
                _ => break,
            }
        }
        println!("Client tx thread stopping");
    });

    // read events from the coordinator.
    thread::spawn(move || {
        let mut writer = BufWriter::new(conn);
        for event in rx {
            match event {
                Event::Line { client, line } => {
                    let msg = format!("{}: {}\n", client, line);
                    let res = writer
                        .write_all(msg.as_bytes())
                        .and_then(|_| writer.flush());
                    if res.is_err() {
                        break;
                    }
                }
                _ => {}
            }
        }
        println!("Client rx thread stopping");
    });

    Client { id, tx }
}
