mod chat;

fn main() {
    let s = chat::server::new(3000);
    match s.serve() {
        Err(e) => println!("Server died: {}", e),
        _ => {}
    };
}
