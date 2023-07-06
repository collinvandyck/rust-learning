use async_3::utils::{self, ChatResult};
use async_3::{FromClient, FromServer};
use async_std::net::TcpStream;
use async_std::{io, prelude::*, task};

fn main() -> ChatResult<()> {
    let address = std::env::args().nth(1).expect("Usage: client ADDRESS:PORT");
    task::block_on(async {
        let socket = TcpStream::connect(address).await?;
        socket.set_nodelay(true)?;
        let to_server = send_commands(socket.clone());
        let from_server = handle_replies(socket);
        from_server.race(to_server).await?;
        Ok(())
    })
}

async fn send_commands(mut to_server: TcpStream) -> ChatResult<()> {
    println!(
        "Commands:\n\
             join GROUP\n\
             post GROUP MESSAGE\n\
             Type Ctrl-D to close."
    );
    let mut command_lines = io::BufReader::new(io::stdin()).lines();
    while let Some(command_result) = command_lines.next().await {
        let command = command_result?;
        let request = match parse_command(&command) {
            Some(request) => request,
            None => continue,
        };
        utils::send_as_json(&mut to_server, &request).await?;
        to_server.flush().await?;
    }
    Ok(())
}

async fn handle_replies(from_server: TcpStream) -> ChatResult<()> {
    let buffered = io::BufReader::new(from_server);
    let mut reply_stream = utils::receive_as_json(buffered);
    while let Some(reply) = reply_stream.next().await {
        match reply? {
            FromServer::Message {
                group_name,
                message,
            } => {
                println!("message posted to {group_name}: {message}")
            }
            FromServer::Error(message) => {
                println!("error from server: {message}")
            }
        }
    }
    Ok(())
}

fn parse_command(s: &String) -> Option<FromClient> {
    let parts = s.split(' ').collect::<Vec<_>>();
    if parts.len() <= 1 {
        eprintln!("Invalid command: {s}");
        return None;
    }
    let mut iter = parts.into_iter();
    let group = iter.next().unwrap();
    match iter.next().unwrap() {
        "join" => Some(FromClient::Join {
            group_name: group.into(),
        }),
        "post" => {
            let rest = iter.collect::<Vec<_>>();
            if rest.is_empty() {
                eprintln!("No message");
                None
            } else {
                let msg = &rest.join(" ");
                Some(FromClient::Post {
                    group_name: group.into(),
                    message: msg.into(),
                })
            }
        }
        _ => None,
    }
}
