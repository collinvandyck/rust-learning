mod args;
mod error;

mod prelude {
    pub use crate::args::*;
    pub use crate::error::*;
}

use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    process,
};

use prelude::*;

fn main() {
    let args = Args::parse();
    if let Err(e) = run(&args) {
        eprintln!("{e}");
        process::exit(1);
    }
}

fn run(args: &Args) -> CatResult<()> {
    let readers = readers(&args)?;
    for reader in readers {
        consume(args, reader)?;
    }
    Ok(())
}

fn consume(_args: &Args, reader: impl BufRead) -> CatResult<()> {
    for line in reader.lines() {
        let line = line?;
        let line = line.trim_end();
        println!("{line}");
    }
    Ok(())
}

fn readers(args: &Args) -> CatResult<Vec<Box<dyn BufRead>>> {
    let mut res = vec![];
    for file in &args.files {
        let file = File::open(file);
        match file {
            Err(e) => return Err(Error::IO(e)),
            Ok(file) => {
                let file = BufReader::new(file);
                res.push(Box::new(file) as Box<dyn BufRead>);
            }
        }
    }
    if res.len() == 0 {
        // add stdin
        let reader = BufReader::new(io::stdin());
        res.push(Box::new(reader) as Box<dyn BufRead>);
    }
    Ok(res)
}
