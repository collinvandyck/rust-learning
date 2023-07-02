mod server;

fn main() {
    let s = server::new(3000);
    s.serve().expect("server failed");
}
