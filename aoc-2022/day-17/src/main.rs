mod prelude {
    pub use clap::Parser;
}
use prelude::*;

#[derive(Parser)]
struct Args {
    #[arg(short, default_value = "example.txt")]
    pub filename: String,
}

fn main() {
    let args = &Args::parse();
    println!("{}", args.filename);
}
