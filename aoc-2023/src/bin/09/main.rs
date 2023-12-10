fn main() {
    let example = include_str!("example.txt");
    let input = include_str!("input.txt");
    println!("p1ex={}", sum_nexts(example, true));
    println!("p1in={}", sum_nexts(input, true));
    println!("p2in={}", sum_nexts(input, false));
}

type Num = i64;

fn sum_nexts(input: &str, forward: bool) -> Num {
    parse(input)
        .into_iter()
        .map(|nums| next(nums.clone(), forward))
        .sum()
}

fn next(nums: Vec<Num>, forward: bool) -> Num {
    let mut stack = vec![nums];
    while !stack_done(&stack) {
        let last = stack.last().unwrap();
        let next = last
            .windows(2)
            .map(|s| (s[0], s[1]))
            .map(|(one, two)| two - one)
            .collect::<Vec<_>>();
        stack.push(next);
    }
    stack
        .iter()
        .rev()
        .flat_map(|f| if forward { f.last() } else { f.first() })
        .copied()
        .reduce(|a, b| if forward { a + b } else { b - a })
        .unwrap_or_default()
}

fn stack_done(stack: &Vec<Vec<Num>>) -> bool {
    stack
        .last()
        .map(|s| s.iter().all(|n| n == &0))
        .unwrap_or_default()
}

#[test]
fn test_outputs() {
    let input = include_str!("input.txt");
    assert_eq!(sum_nexts(input, true), 2098530125);
    assert_eq!(sum_nexts(input, false), 1016);
}

#[test]
fn test_next() {
    // test forward directions
    let nums = vec![0, 3, 6, 9, 12, 15];
    assert_eq!(next(nums, true), 18);
    let nums = vec![1, 3, 6, 10, 15, 21];
    assert_eq!(next(nums, true), 28);
    let nums = vec![10, 13, 16, 21, 30, 45];
    assert_eq!(next(nums, true), 68);

    // test backward direction
    let nums = vec![10, 13, 16, 21, 30, 45];
    assert_eq!(next(nums, false), 5);
}

fn parse(input: &str) -> Vec<Vec<Num>> {
    input.lines().map(parse_seq).collect()
}

fn parse_seq(input: &str) -> Vec<Num> {
    input
        .split_whitespace()
        .map(|s| s.parse::<Num>().unwrap())
        .collect()
}
