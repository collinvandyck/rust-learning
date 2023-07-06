use core::num;
use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};

fn main() {
    let filenames = ["data.txt", "notfound.txt"];
    for f in filenames {
        let res = file_double(f);
        println!("{f}: {res:?}");
    }
}

fn file_double<T: AsRef<Path>>(p: T) -> Result<i32, CliError> {
    let mut file = File::open(p).map_err(CliError::IO)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf).map_err(CliError::IO)?;
    let res = buf.trim().parse::<i32>().map_err(CliError::Parse)?;
    Ok(res * 2)
}

#[derive(Debug)]
enum CliError {
    IO(io::Error),
    Parse(num::ParseIntError),
}
