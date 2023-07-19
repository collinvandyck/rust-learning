#[derive(Debug)]
pub enum Packet {
    Value(i32),
    List(Vec<Packet>),
}

pub fn parse_packet(line: String) -> Packet {
    let mut chars = line.chars();
    parse_chars(&mut chars)
}

fn parse_chars(chars: &mut impl Iterator<Item = char>) -> Packet {
    Packet::Value(42)
}
