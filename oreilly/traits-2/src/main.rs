use std::{
    io::{self, Write},
    str,
};

fn main() {
    println!("Hello, world!");
}

trait Visible {
    fn draw(&self, canvas: &mut Canvas);

    fn hit_test(&self, x: i32, y: i32) -> bool;
}

#[allow(dead_code)]
struct Broom {}

#[allow(unused)]
impl Visible for Broom {
    fn draw(&self, canvas: &mut Canvas) {}
    fn hit_test(&self, x: i32, y: i32) -> bool {
        false
    }
}

#[allow(dead_code)]
struct Canvas {}

pub struct Sink;

impl Write for Sink {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

trait IsEmoji {
    fn is_emoji(&self) -> bool;
}

impl IsEmoji for str {
    fn is_emoji(&self) -> bool {
        self.len() > 0
    }
}

#[allow(dead_code)]
struct HtmlDocument;

trait WriteHtml {
    fn write_html(&mut self, html: &HtmlDocument) -> io::Result<()>;
}

impl<W: Write> WriteHtml for W {
    fn write_html(&mut self, _html: &HtmlDocument) -> io::Result<()> {
        self.write(&[b'H']).map(|_| ())
    }
}

#[test]
fn test_write_html() {
    let mut b = vec![];
    b.write_html(&HtmlDocument {}).expect("failure");
    assert_eq!(&vec![b'H'], &b)
}
