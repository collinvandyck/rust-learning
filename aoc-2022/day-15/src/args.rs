use crate::prelude::*;
#[derive(Parser)]
pub struct Args {
    #[arg(short, default_value = "example.txt")]
    pub filename: String,

    #[arg(short, default_value_t = 10)]
    pub y: i32,

    #[arg(long, default_value_t = false)]
    pub print_map: bool,
}
