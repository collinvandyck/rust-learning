fn main() {
    let example = include_str!("example.txt");
    println!("p1ex={}", sum_nexts(example));
}

fn sum_nexts(input: &str) -> u64 {
    todo!()
}

struct Seq(Vec<u64>);

fn parse(input: &str) -> Vec<Seq> {
    input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|s| s.trim().parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(Seq)
        .collect()
}
