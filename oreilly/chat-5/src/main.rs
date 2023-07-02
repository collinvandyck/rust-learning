use crate::server::Server;

mod server;

fn main() {
    Server::new(3000).run().expect("server failure")
}
