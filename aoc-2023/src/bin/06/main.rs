fn main() {
    let example = include_str!("example.txt");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Race {
    time: u64,
    distance: u64,
}

fn parse(input: &str) -> Vec<Race> {
    let mut lines = input.lines();
    let times = line_to_nums(lines.next().unwrap());
    let distances = line_to_nums(lines.next().unwrap());
    times
        .zip(distances)
        .map(|(time, distance)| Race { time, distance })
        .collect()
}

fn line_to_nums(input: &str) -> impl Iterator<Item = u64> + '_ {
    input
        .splitn(2, ":")
        .nth(1)
        .unwrap()
        .split(" ")
        .flat_map(|n| n.parse::<u64>().ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let games = parse(include_str!("example.txt"));
        assert_eq!(
            games,
            vec![
                Race {
                    time: 7,
                    distance: 9
                },
                Race {
                    time: 15,
                    distance: 40
                },
                Race {
                    time: 30,
                    distance: 200
                },
            ]
        );
    }
}
