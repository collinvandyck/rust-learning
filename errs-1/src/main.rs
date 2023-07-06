use core::{fmt, num};
use std::{
    error::Error,
    fs::File,
    io::{self, Read},
    path::Path,
};

fn main() {
    let filenames = ["data.txt", "bad.txt", "notfound.txt"];
    for f in filenames {
        match file_double(f) {
            Ok(res) => println!("{f}: {res}"),
            Err(e) => eprintln!("{f}: {e}"),
        }
    }
}

fn file_double<T: AsRef<Path>>(p: T) -> Result<i32, Box<dyn Error>> {
    let mut file = File::open(p).map_err(CliError::IO)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf).map_err(CliError::IO)?;
    let buf = buf.trim().to_string();
    let res = buf.parse::<i32>().map_err(|e| CliError::Parse(buf, e))?;
    Ok(res * 2)
}

#[derive(Debug)]
enum CliError {
    IO(io::Error),
    Parse(String, num::ParseIntError),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            CliError::IO(ref err) => write!(f, "IO Error: {}", err),
            CliError::Parse(ref s, ref err) => write!(f, "Parse error for '{s}': {}", err),
        }
    }
}

impl Error for CliError {
    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            CliError::IO(ref err) => Some(err),
            CliError::Parse(_, ref err) => Some(err),
        }
    }
}
