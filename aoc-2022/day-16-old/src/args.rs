use crate::prelude::*;

#[derive(Parser)]
pub struct Args {
    #[arg(short, long, default_value = "example.txt")]
    pub filename: String,

    #[arg(short, long, default_value_t = 30)]
    pub minutes: usize,
}
