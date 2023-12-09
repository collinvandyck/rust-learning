fn main() {
    let example = include_str!("example.txt");
    let input = include_str!("input.txt");
    println!("p1ex={}", margin_of_error(example, false));
    println!("p1in={}", margin_of_error(input, false));
    println!("p2ex={}", margin_of_error(example, true));
    println!("p2in={}", margin_of_error(input, true));
}

fn margin_of_error(input: &str, sq: bool) -> u64 {
    let games = if sq {
        vec![squash(parse(input))]
    } else {
        parse(input)
    };
    games.into_iter().map(|r| r.ways_to_beat()).product()
}

fn squash(games: Vec<Race>) -> Race {
    let mut ts = String::new();
    let mut ds = String::new();
    for game in games {
        ts = format!("{ts}{}", game.time);
        ds = format!("{ds}{}", game.distance);
    }
    Race {
        time: ts.parse().unwrap(),
        distance: ds.parse().unwrap(),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn ways_to_beat(&self) -> u64 {
        (1..self.time)
            .map(|n| self.distance(n))
            .filter(|n| *n > self.distance)
            .count()
            .try_into()
            .unwrap()
    }
    fn distance(&self, warmup: u64) -> u64 {
        if warmup == 0 || warmup >= self.time {
            return 0;
        }
        let rem_time = self.time - warmup;
        warmup * rem_time
    }
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
    fn test_race() {
        let race = Race {
            time: 7,
            distance: 9,
        };
        assert_eq!(race.ways_to_beat(), 4);
    }

    #[test]
    fn test_margin_of_error() {
        let moe = margin_of_error(include_str!("example.txt"), false);
        assert_eq!(moe, 288);
    }

    #[test]
    fn test_squash() {
        let races = parse(include_str!("example.txt"));
        let race = squash(races);
        assert_eq!(
            race,
            Race {
                time: 71530,
                distance: 940200,
            }
        );
    }

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
