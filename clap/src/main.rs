use clap::Parser;

#[derive(Parser, Debug)]
#[command(author="Collin", version, about, long_about=None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,

    /// other thing idk
    other: Option<String>,

    /// things that foo
    #[arg(short)]
    foos: Vec<String>,
}

fn main() {
    let args = Args::parse();
    for _ in 0..args.count {
        println!("Hello, {}!", args.name);
    }
    let other: Option<&str> = args.other.as_deref();
    dbg!(other);

    for x in &args.foos {
        println!("foo: {x}")
    }
    for x in &args.foos {
        println!("foo: {x}")
    }
}
