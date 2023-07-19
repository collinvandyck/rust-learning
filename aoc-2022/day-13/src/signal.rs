#[derive(Debug)]
pub enum Packet {
    Integer(i32),
    List(Vec<Packet>),
}

pub fn parse_packet(line: String) -> Packet {
    Packet::Integer(42)
}
