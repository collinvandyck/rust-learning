#![warn(clippy::all, clippy::pedantic)]

use std::{
    error::Error,
    io::{BufRead, BufReader, BufWriter, Write},
    net::{TcpListener, TcpStream},
    sync::mpsc::{self, Receiver, SyncSender},
    thread,
};

type ClientID = usize;

enum Event {
    Client(ClientID, SyncSender<Event>),
    Line(ClientID, String),
    Quit(ClientID),
}

// starts a tcp listener on port 3000. clients which connect
// on this port have their messages relayed to other clients.
fn main() -> Result<(), Box<dyn Error>> {
    let (tx, rx) = mpsc::sync_channel(1024);
    thread::spawn(move || controller_thread(rx));
    let port = 3000;
    let addr = format!("0.0.0.0:{port}");
    println!("Listening on {addr}");
    let listener = TcpListener::bind(addr)?;
    for (id, conn) in listener.incoming().enumerate() {
        let conn = conn?;
        let tx = tx.clone();
        thread::spawn(move || handle_conn(id, conn, tx));
    }
    Ok(())
}

// the main controller thread. manages the collection of active clients.
// incoming messages are relayed to the other clients.
fn controller_thread(rx: Receiver<Event>) {
    let mut clients = vec![];
    for event in rx {
        match event {
            Event::Client(id, tx) => clients.push((id, tx)),
            Event::Quit(id) => clients.retain(|(cid, _)| id != *cid),
            Event::Line(id, line) => clients.retain(|client| match client {
                (client_id, _) if id == *client_id => true,
                (_, tx) => tx.send(Event::Line(id, line.clone())).is_ok(),
            }),
        }
    }
}

// handlesa new conn in its own thread. spawns a separate thread to read input
// from the client. messages from the client are sent to the control thread.
// messages received from the control thread are written to the client.
fn handle_conn(id: ClientID, conn: TcpStream, control_tx: SyncSender<Event>) {
    let (tx, rx) = mpsc::sync_channel(1);
    if control_tx.send(Event::Client(id, tx)).is_err() {
        return;
    }
    let reader = BufReader::new(conn.try_clone().unwrap());
    thread::spawn(move || {
        for line in reader.lines() {
            if let Ok(line) = line {
                if control_tx.send(Event::Line(id, line)).is_err() {
                    break;
                }
            } else {
                break;
            }
        }
        control_tx.send(Event::Quit(id)).unwrap();
    });
    let mut writer = BufWriter::new(conn);
    for event in rx {
        if let Event::Line(id, line) = event {
            let msg = format!("{id}: {line}\n");
            let res = writer
                .write_all(msg.as_bytes())
                .and_then(|_| writer.flush());
            if res.is_err() {
                break;
            }
        }
    }
}
