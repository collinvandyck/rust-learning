use clap::Parser;

fn main() {
    let args = Args::parse();
    println!("{}", &args.filename);
}

#[derive(Parser)]
struct Args {
    #[arg(short, default_value = "example.txt")]
    filename: String,
}
