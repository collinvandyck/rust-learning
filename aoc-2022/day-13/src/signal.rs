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
    pub fn cmp(&self, other: &Packet) -> Ordering {
        let debug = false;
        let res = self.do_cmp(other, debug, 0);
        if debug {
            println!("{res:?}");
        }
        res
    }
    fn do_cmp(&self, other: &Packet, debug: bool, depth: usize) -> Ordering {
        use Ordering::*;
        use Packet::*;
        if debug {
            let indent = "  ".repeat(depth);
            println!("{indent}{} {}", self, &other);
        }
        match (self, other) {
            (List(left), List(right)) => left
                .iter()
                .zip(right.iter())
                .map(|(left, right)| left.do_cmp(right, debug, depth + 1))
                .find(|ord| ord != &Equal)
                .unwrap_or_else(|| left.len().cmp(&right.len())),
            (Value(left), Value(right)) => {
                // compare values
                left.cmp(right)
            }
            (left @ List(_), Value(right)) => {
                // convert right to a list
                let right = Packet::List(vec![Packet::Value(*right)]);
                left.do_cmp(&right, debug, depth + 1)
            }
            (Value(left), right @ List(_)) => {
                // convert left to a list
                let left = Packet::List(vec![Packet::Value(*left)]);
                left.do_cmp(right, debug, depth + 1)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Pair {
    pub left: Packet,
    pub right: Packet,
}

impl Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}", &self.left, &self.right)
    }
}

impl Pair {
    pub fn is_ordered(&self) -> bool {
        self.left.cmp(&self.right) != Ordering::Greater
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
    let mut buf = String::new();
    while let Some(ch) = chars.peek() {
        if ch.is_ascii_digit() {
            buf.push(*ch);
            chars.next();
            continue;
        }
        break;
    }
    buf.parse::<u32>().unwrap()
}

#[test]
fn test_consume_num() {
    let mut s = "5323".chars().peekable();
    assert_eq!(5323, consume_num(&mut s));
    let mut s = "10".chars().peekable();
    assert_eq!(10, consume_num(&mut s));
    let mut s = "7".chars().peekable();
    assert_eq!(7, consume_num(&mut s));
    let mut s = "0".chars().peekable();
    assert_eq!(0, consume_num(&mut s));
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
            ("[1]", "[1]", true),
            ("[1,1,3,1,1]", "[1,1,5,1,1]", true),
            ("[[1],[2,3,4]]", "[[1],4]", true),
            ("[9]", "[[8,7,6]]", false),
            ("[[4,4],4,4]", "[[4,4],4,4,4]", true),
            ("[7,7,7,7]", "[7,7,7]", false),
            ("[]", "[3]", true),
            ("[[[]]]", "[[]]", false),
            ("[10]", "[9]", false),
            (
                "[1,[2,[3,[4,[5,6,7]]]],8,9]",
                "[1,[2,[3,[4,[5,6,0]]]],8,9]",
                false,
            ),
        ];
        for (one, two, ordered) in ordered {
            println!();
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
