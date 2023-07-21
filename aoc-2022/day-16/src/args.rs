use crate::prelude::*;

#[derive(Parser)]
pub struct Args {
    #[arg(short, long, default_value = "example.txt")]
    pub filename: String,
}
