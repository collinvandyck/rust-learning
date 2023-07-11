use std::fs;

fn main() {
    let s = fs::read_to_string("input.txt").unwrap();
    let res = start_of_packet(&s);
    println!("{res:?}")
}

fn start_of_packet(s: &str) -> Option<usize> {
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
    assert_eq!(start_of_packet("bvwbjplbgvbhsrlpgdmjqwftvncz"), Some(5));
    assert_eq!(start_of_packet("nppdvjthqldpwncqszvftbrmjlhg"), Some(6));
    assert_eq!(
        start_of_packet("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
        Some(10)
    );
    assert_eq!(
        start_of_packet("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
        Some(11)
    );
}
