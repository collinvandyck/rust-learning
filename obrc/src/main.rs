use std::{
    env, fs,
    io::{BufRead, BufReader},
};

fn main() {
    let file = fs::OpenOptions::new()
        .read(true)
        .open(env::var("OBRC_FILE").expect("no OBRC_FILE env var set"))
        .expect("could not open file");
    let mut file = BufReader::new(file);
    let mut buf = String::new();
    let mut count = 0;
    loop {
        let _n = match file.read_line(&mut buf) {
            Ok(0) => break,
            Ok(n) => n,
            Err(err) => panic!("read failed: {err}"),
        };
        count += 1;
        let sep = buf.find(';').expect("no sep");
        buf.clear();
    }
    println!("Total: {count}");
}
