use clap::Parser;
use regex::Regex;

fn main() {
    let config = Config::parse();
    println!("{}", &config.filename);
}

#[derive(Parser)]
struct Config {
    #[arg(short, default_value = "example.txt")]
    filename: String,
}

struct Blueprint {
    idx: usize,
}

impl Blueprint {
    fn parse(line: &str) -> Blueprint {
        let re = Regex::new(r#""#).unwrap();
    }
}
