mod chat;

fn main() {
    let s = chat::server::new(3000);
    s.serve().expect("server failed");
}
