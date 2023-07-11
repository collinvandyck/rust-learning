use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    process("example.txt");
}

fn process(filename: &str) {
    let mut state = State::new();
    let tokens = lex(filename);
    let mut iter = tokens.into_iter().peekable();
    loop {
        let token = iter.next();
        if token.is_none() {
            break;
        }
        let token = token.unwrap();
        match token {
            Token::CmdCd { ref name } => state.cd(name.to_string()),
            Token::CmdLs => {
                // consume as long as the next token is a dir header or file info
                loop {
                    let peek = iter.peek();
                    match peek {
                        Some(Token::DirHeader { name }) => state.add_dir(name.to_string()),
                        Some(Token::FileInfo { size, name }) => {
                            state.add_file(name.to_string(), *size)
                        }
                        _ => break,
                    }
                    iter.next();
                }
            }
            _ => panic!("Invalid token sequence"),
        }
    }
}

struct State {
    root: FSDir,
    pwd: Option<String>,
}

impl State {
    fn new() -> Self {
        Self {
            root: FSDir {
                name: "/".into(),
                children: vec![],
            },
            pwd: None,
        }
    }
    fn cd(&mut self, path: String) {
        self.pwd = Some(path);
    }
    fn add_dir(&mut self, dir: String) {}
    fn add_file(&mut self, name: String, size: u64) {}
}

#[test]
fn test_split_dir() {
    assert_eq!(split_dir("/"), Vec::<&str>::new());
    assert_eq!(split_dir("/abc/def"), ["abc", "def"]);
    assert_eq!(split_dir("/abc"), ["abc"]);
}

fn split_dir(s: &str) -> Vec<&str> {
    let mut skip = 0;
    if s.starts_with('/') {
        skip = 1;
    }
    let res = s.split("/").skip(skip).collect::<Vec<_>>();
    if dbg!(res[0]) == "" {
        res[1..].to_vec()
    } else {
        res
    }
}

struct FSDir {
    name: String,
    children: Vec<FS>,
}

struct FSFile {
    name: String,
    size: usize,
}

enum FS {
    Dir(FSDir),
    File(FSFile),
}

impl FS {}

#[allow(dead_code)]
#[derive(Debug)]
enum Token {
    CmdCd { name: String },
    CmdLs,
    DirHeader { name: String },
    FileInfo { size: u64, name: String },
}

fn lex(filename: &str) -> Vec<Token> {
    let file = File::open(filename).unwrap();
    let read = BufReader::new(file);
    read.lines()
        .map(|line| {
            let line = line.unwrap();
            let split = line.split(' ').collect::<Vec<_>>();
            match split[..] {
                ["$", "cd", name] => Token::CmdCd {
                    name: name.to_string(),
                },
                ["$", "ls"] => Token::CmdLs,
                ["dir", name] => Token::DirHeader {
                    name: name.to_string(),
                },
                [size, name] => {
                    let size = size.parse::<u64>().unwrap();
                    Token::FileInfo {
                        size: size,
                        name: name.to_string(),
                    }
                }
                _ => panic!("boom"),
            }
        })
        .collect()
}
