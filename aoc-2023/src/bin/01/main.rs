use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let example = include_str!("example.txt");
    let examplep2 = include_str!("example-p2.txt");
    println!("p1 ex: {}", run(example, false));
    println!("p1 in: {}", run(input, false));
    println!("p2 ex: {}", run(examplep2, true));
    println!("p2 in: {}", run(input, true));
}

fn run(s: &str, scan_alpha: bool) -> u32 {
    let mut sum = 0;
    for s in s.lines() {
        let (first, last) = scan_digits(s, scan_alpha);
        let num = first * 10 + last;
        sum += num;
    }
    sum
}

fn scan_digits(s: &str, use_alpha: bool) -> (u32, u32) {
    let mut lu = HashMap::from([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);
    if use_alpha {
        lu.insert("one", 1);
        lu.insert("two", 2);
        lu.insert("three", 3);
        lu.insert("four", 4);
        lu.insert("five", 5);
        lu.insert("six", 6);
        lu.insert("seven", 7);
        lu.insert("eight", 8);
        lu.insert("nine", 9);
    }
    let mut digis = vec![];
    for i in 0..s.len() {
        let s = &s[i..];
        for k in lu.keys() {
            if s.starts_with(*k) {
                digis.push(lu.get(k).unwrap());
                break;
            }
        }
    }
    match digis[..] {
        [num] => (*num, *num),
        [first, .., last] => (*first, *last),
        _ => panic!("first and last not found"),
    }
}
