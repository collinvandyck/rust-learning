use std::{cmp::Ordering, fmt::Display, iter::Peekable, str::Chars};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd)]
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

impl Packet {
    fn cmp(&self, other: &Packet) -> Ordering {
        use Ordering::*;
        use Packet::*;
        match (self, other) {
            (List(left), List(right)) => left
                .iter()
                .zip(right.iter())
                .map(|(left, right)| left.cmp(right))
                .find(|ord| ord != &Equal)
                .unwrap_or_else(|| left.len().cmp(&right.len())),
            (Value(left), Value(right)) => {
                // compare values
                left.cmp(right)
            }
            (left @ List(_), Value(right)) => {
                // convert right to a list
                let right = Packet::List(vec![Packet::Value(*right)]);
                left.cmp(&right)
            }
            (Value(left), right @ List(_)) => {
                // convert left to a list
                let left = Packet::List(vec![Packet::Value(*left)]);
                left.cmp(&right)
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
        left.cmp(&right) != Ordering::Greater
    }
    fn ordered_old(left: &Packet, right: &Packet) -> bool {
        println!("ordered {left} x {right}");
        match (left, right) {
            (Packet::List(left), Packet::List(right)) => {
                let mut iter = left.iter().zip(right.iter());
                let ok = iter.all(|(left, right)| Self::ordered(left, right));
                // all pairs are so far ordered. if the right
                // list ran out of items first, the inputs are not
                // in the right order.
                if ok {
                    println!("--> zipped values are ordered");
                    let lens_ok = left.len() <= right.len();
                    println!("--> lens ok? {lens_ok}");
                    lens_ok
                } else {
                    println!("--> zipped values are not ordered");
                    false
                }
            }
            (Packet::Value(left), Packet::Value(right)) => {
                let res = left <= right;
                println!("--> compare values: {res}");
                res
            }
            (left @ Packet::List(_), Packet::Value(right)) => {
                let right = Packet::List(vec![Packet::Value(*right)]);
                println!("--> right to list");
                Self::ordered(left, &right)
            }
            (Packet::Value(left), right @ Packet::List(_)) => {
                let left = Packet::List(vec![Packet::Value(*left)]);
                println!("--> left to list");
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
        let ordered: Vec<(&str, &str, bool)> = vec![
            ("[]", "[]", true),
            ("[1]", "[1]", true),
            ("[1,1,3,1,1]", "[1,1,5,1,1]", true),
            ("[[1],[2,3,4]]", "[[1],4]", true),
            ("[[4,4],4,4]", "[[4,4],4,4,4]", true),
            ("[9]", "[[8,7,6]]", false),
        ];
        let ordered: Vec<(&str, &str, bool)> = vec![("[[1],[2,3,4]]", "[[1],4]", true)];
        for (one, two, ordered) in ordered {
            let one = parse_packet(one.to_string());
            let two = parse_packet(two.to_string());
            let pair = Pair {
                left: one,
                right: two,
            };
            if ordered && !pair.is_ordered() {
                assert!(false, "Pair {} x {} was not ordered", pair.left, pair.right);
            }
            if !ordered && pair.is_ordered() {
                assert!(false, "Pair {} x {} was ordered", pair.left, pair.right);
            }
        }
    }
}
