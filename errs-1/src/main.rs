use std::{path::Path, process};

fn main() {
    match file_double("data.txt") {
        Ok(v) => println!("Got value: {v}"),
        Err(e) => {
            eprintln!("Failure: {e:?}");
            process::exit(1);
        }
    }
}

fn file_double<T: AsRef<Path>>(p: T) -> Result<i32, String> {
    Ok(5)
}
