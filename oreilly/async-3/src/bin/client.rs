use std::sync::Arc;

use async_3::utils::{self, ChatResult};
use async_3::FromClient;
use async_std::net::TcpStream;
use async_std::{io, prelude::*};

fn main() {}

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

fn parse_command(s: &String) -> Option<FromClient> {
    let s = s.split(" ").collect::<Vec<_>>();
    if s.len() <= 1 {
        return None;
    }
    let s = s.into_iter();
    let group_name = s.next().unwrap();
    match s.next().unwrap() {
        "join" => Some(FromClient::Join {
            group_name: Arc::new(group_name.into()),
        }),
        "post" => {
            let rest = s.collect::<Vec<_>>();
            if rest.is_empty() {
                None
            } else {
                let msg = rest.join(" ");
                Some(FromClient::Post {
                    group_name: Arc::new(group_name.into()),
                    message: msg.into(),
                })
            }
        }
        _ => None,
    }
}
