#![allow(dead_code)]

use std::io::{self, BufRead, ErrorKind, Read, Write};

fn main() {
    let mut src: &[u8] = b"Hello, World!";
    let mut dst: Vec<u8> = Vec::default();
    match copy(&mut src, &mut dst) {
        Ok(written) => println!("Wrote {} bytes", written),
        Err(e) => panic!("Failed to write: {}", e),
    }
    dbg!(dst);

    grep("foo", io::stdin().lock()).expect("could not grep");
}

const DEFAULT_BUF_SIZE: usize = 8 * 1024;

fn grep<R>(target: &str, reader: R) -> io::Result<()>
where
    R: BufRead,
{
    for line in reader.lines() {
        let line = line?;
        if line.contains(target) {
            println!("{}", line);
        }
    }
    Ok(())
}

// a low level copy that handles interrupted errors. normal code uses
// higher level APIs to avoid having to do this.
pub fn copy<R, W>(reader: &mut R, writer: &mut W) -> io::Result<u64>
where
    R: Read + ?Sized,
    W: Write + ?Sized,
{
    let mut buf = [0; DEFAULT_BUF_SIZE];
    let mut written = 0;
    loop {
        let len = match reader.read(&mut buf) {
            Ok(0) => return Ok(written),
            Ok(len) => len,
            Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        };
        writer.write_all(&buf[..len])?;
        written += len as u64;
    }
}
