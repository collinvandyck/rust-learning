use std::{path::Path, process};

fn main() {
    let res = file_double("data.txt").unwrap_or_else(|e| {
        eprintln!("Failure: {e:?}");
        process::exit(1);
    });
    println!("Res: {res}");
}

fn file_double<T: AsRef<Path>>(p: T) -> Result<i32, String> {
    Ok(5)
}
