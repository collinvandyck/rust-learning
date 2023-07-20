use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[clap(short, long, default_value = "example.txt")]
    pub filename: String,

    #[clap(short, long)]
    pub part: Option<u8>,
}
