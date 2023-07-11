use std::fs;

fn main() {
    let s = fs::read_to_string("input.txt").unwrap();
    let res = first_marker(&s);
    println!("{res:?}")
}

fn first_marker(s: &str) -> Option<usize> {
    let chars = s.chars().collect::<Vec<_>>();
    let idx = chars.windows(4).enumerate().find(|(_i, chars)| {
        let mut chs = chars.to_vec();
        chs.sort();
        chs.windows(2).all(|v| v[0] != v[1])
    });
    idx.map(|i| i.0 + 4)
}

#[test]
fn test_first_marker() {
    assert_eq!(first_marker("bvwbjplbgvbhsrlpgdmjqwftvncz"), Some(5));
    assert_eq!(first_marker("nppdvjthqldpwncqszvftbrmjlhg"), Some(6));
    assert_eq!(first_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), Some(10));
    assert_eq!(first_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), Some(11));
}
