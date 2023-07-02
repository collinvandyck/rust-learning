use std::io::Write;
use std::process::{Command, Stdio};

fn main() {
    let mut child = Command::new("grep")
        .arg("-e")
        .arg("a.*e.*i.*o.*u")
        .stdin(Stdio::piped())
        .spawn()
        .expect("wtf");
    let mut stdin = child.stdin.take().unwrap();
    for word in vec!["Hi", "There", "aeiou"] {
        writeln!(stdin, "{}", word).expect("write fail");
    }
    drop(stdin);
    child.wait().expect("child wait");
}
