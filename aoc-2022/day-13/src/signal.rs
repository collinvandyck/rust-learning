use std::{fmt::Display, iter::Peekable, str::Chars};

#[derive(Debug, Clone)]
pub enum Packet {
    Value(u32),
    List(Vec<Packet>),
}

impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Packet::Value(v) => write!(f, "{}", *v),
            Packet::List(list) => {
                let mut buf = String::new();
                buf.push('[');
                let parts = list.iter().map(|p| format!("{p}")).collect::<Vec<String>>();
                buf.push_str(&parts.join(","));
                buf.push(']');
                write!(f, "{buf}")
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Pair {
    left: Packet,
    right: Packet,
}

impl Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}", &self.left, &self.right)
    }
}

impl Pair {
    pub fn is_ordered(&self) -> bool {
        Self::ordered(&self.left, &self.right)
    }
    fn ordered(left: &Packet, right: &Packet) -> bool {
        match (left, right) {
            (Packet::List(left), Packet::List(right)) => {
                let mut iter = left.iter().zip(right.iter());
                let res = iter.all(|(left, right)| Self::ordered(left, right));
                // all pairs are so far ordered. if the right
                // list ran out of items first, the inputs are not
                // in the right order.
                res && (left.len() <= right.len())
            }
            (Packet::Value(left), Packet::Value(right)) => left <= right,
            (left @ Packet::List(_), Packet::Value(right)) => {
                let right = Packet::List(vec![Packet::Value(*right)]);
                Self::ordered(left, &right)
            }
            (Packet::Value(left), right @ Packet::List(_)) => {
                let left = Packet::List(vec![Packet::Value(*left)]);
                Self::ordered(&left, right)
            }
        }
    }
}

pub fn parse_pair(left: String, right: String) -> Pair {
    Pair {
        left: parse_packet(left),
        right: parse_packet(right),
    }
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pairs() {
        let ordered: Vec<(&str, &str)> =
            vec![("[]", "[]"), ("[1]", "[1]"), ("[1,1,3,1,1]", "[1,1,5,1,1]")];
        for (one, two) in ordered {
            let one = parse_packet(one.to_string());
            let two = parse_packet(two.to_string());
            let pair = Pair {
                left: one,
                right: two,
            };
            if !pair.is_ordered() {
                assert!(false, "Pair {pair} was not ordered");
            }
        }
    }
}
