use std::{iter::Peekable, str::Chars};

#[derive(Debug)]
pub enum Packet {
    Value(u32),
    List(Vec<Packet>),
}

pub fn parse_packet(line: String) -> Packet {
    let mut chars = line.chars().peekable();
    parse_chars(&mut chars)
}

fn parse_chars(chars: &mut Peekable<Chars>) -> Packet {
    let ch = chars.peek().unwrap();
    match ch {
        '[' => {
            let list = consume_list(chars);
            Packet::List(list)
        }
        _ => {
            let num = consume_num(chars);
            Packet::Value(num)
        }
    }
}

fn consume_num(chars: &mut Peekable<Chars>) -> u32 {
    let mut factor = 1;
    let mut res = 0;
    loop {
        let ch = chars.peek().unwrap();
        if let Some(digit) = ch.to_digit(10) {
            chars.next();
            res += digit * factor;
            factor += 1;
            continue;
        }
        break;
    }
    res
}

fn consume_list(chars: &mut Peekable<Chars>) -> Vec<Packet> {
    chars.next(); // consume the '['
    let mut res = vec![];
    if let Some(']') = chars.peek() {
        chars.next();
        return res;
    }
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
