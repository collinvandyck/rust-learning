use clap::Parser;
use lazy_static::lazy_static;
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

lazy_static! {
    static ref RE: Regex = Regex::new(r#""#).unwrap();
}
impl Blueprint {
    fn parse(line: &str) -> Blueprint {}
}
