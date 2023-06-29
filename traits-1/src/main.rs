use std::io::Write;

fn main() {
    let mut buf: Vec<u8> = vec![];
    let writer: &mut dyn Write = &mut buf;
    writer.write(b"123").expect("write failure");
    println!("{}", buf.len());
}
