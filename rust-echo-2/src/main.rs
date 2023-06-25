use rusty::Server;

fn main() {
    let func = Box::new(|s| format!("Hello, {}!", s));
    let s = Server::new(func);
    match s.run() {
        Err(e) => println!("Error: {}", e),
        _ => {}
    }
}
