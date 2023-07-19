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
    let ch = chars.next().unwrap();
    match ch {
        '[' => {
            let list = consume_list(chars);
            Packet::List(list)
        }
        _ => Packet::Value(42),
    }
}

fn consume_list(chars: &mut impl Iterator<Item = char>) -> Vec<Packet> {
    let mut res = vec![];
    loop {
        let packet = parse_chars(chars);
        res.push(packet);
        match chars.next().unwrap() {
            ',' => continue,
            ']' => return res,
            ch => panic!("Unexpected char: {ch}"),
        }
    }
}
