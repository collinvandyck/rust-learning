fn main() {
    let example = include_str!("example.txt");
    println!("p1ex={}", sum_nexts(example));
}

fn sum_nexts(input: &str) -> u64 {
    parse(input).iter().map(next_value).sum()
}

fn next_value(_seq: &Seq) -> u64 {
    todo!()
}

#[test]
fn test_next_value() {
    let seqs = parse("0   3   6   9  12  15");
    assert_eq!(seqs.len(), 1);
    assert_eq!(next_value(&seqs[0]), 18);
}

struct Seq(Vec<u64>);

fn parse(input: &str) -> Vec<Seq> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(Seq)
        .collect()
}
