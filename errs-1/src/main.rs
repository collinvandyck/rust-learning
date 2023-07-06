use std::{fs::File, io::Read, path::Path, process};

fn main() {
    let res = file_double("data.txt").unwrap_or_else(|e| {
        eprintln!("Failure: {e:?}");
        process::exit(1);
    });
    println!("Res: {res}");
}

fn file_double<T: AsRef<Path>>(p: T) -> Result<i32, String> {
    let mut file = File::open(p).map_err(|e| e.to_string())?;
    let mut buf = String::new();
    file.read_to_string(&mut buf).map_err(|e| e.to_string())?;
    let res = buf
        .trim()
        .parse::<i32>()
        .map_err(|e| format!(r#"could not parse '{buf}': {e} "#))?;
    Ok(res * 2)
}
