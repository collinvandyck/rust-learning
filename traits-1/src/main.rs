use std::io::Write;

fn main() {
    let mut buf: Vec<u8> = vec![];

    // this is a reference to a trait type, which is a trait object.
    // it must be a pointer because the size of whatever the Write
    // implementation cannot be known usually at compile time.
    let writer: &mut dyn Write = &mut buf;

    writer.write(b"123").expect("write failure");
    println!("{}", buf.len());
}
