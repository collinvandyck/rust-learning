use crate::server::Server;

mod io;
mod server;

fn main() {
    Server::new(3000).serve().expect("server failure")
}
