use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    process("example.txt");
}

fn process(filename: &str) {
    let exprs = parse(filename);
    dbg!(exprs);
}

#[allow(dead_code)]
#[derive(Debug)]
enum Expr {
    CmdCd { name: String },
    CmdLs,
    DirHeader { name: String },
    FileInfo { size: u64, name: String },
}

fn parse(filename: &str) -> Vec<Expr> {
    let file = File::open(filename).unwrap();
    let read = BufReader::new(file);
    read.lines()
        .map(|line| {
            let line = line.unwrap();
            let split = line.split(' ').collect::<Vec<_>>();
            match split[..] {
                ["$", "cd", name] => Expr::CmdCd {
                    name: name.to_string(),
                },
                ["$", "ls"] => Expr::CmdLs,
                ["dir", name] => Expr::DirHeader {
                    name: name.to_string(),
                },
                [size, name] => {
                    let size = size.parse::<u64>().unwrap();
                    Expr::FileInfo {
                        size: size,
                        name: name.to_string(),
                    }
                }
                _ => panic!("boom"),
            }
        })
        .collect()
}
