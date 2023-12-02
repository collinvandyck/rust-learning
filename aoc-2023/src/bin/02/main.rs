#![allow(dead_code, unused)]

use std::{collections::HashMap, str::FromStr};

use once_cell::sync::Lazy;
use regex::Regex;

fn main() {
    let example = include_str!("example.txt");
}

#[derive(Debug, PartialEq, Eq)]
struct Game {
    id: u64,
    turns: Vec<HashMap<Color, u64>>,
}

impl Game {
    fn from(s: impl AsRef<str>) -> Self {
        static GAME_RE: Lazy<Regex> = Lazy::new(|| Regex::new("^Game (.*): (.*)").unwrap());
        let caps = GAME_RE.captures(s.as_ref()).unwrap();
        let (id, rest) = (caps.get(1).unwrap().as_str(), caps.get(2).unwrap().as_str());
        Self {
            id: id.parse().unwrap(),
            turns: rest
                .split("; ")
                .map(|turn| {
                    turn.split(", ")
                        .map(|num_color| {
                            let mut iter = num_color.trim().split(" ");
                            let num = iter.next().unwrap().parse::<u64>().unwrap();
                            let color = Color::from_str(iter.next().unwrap().trim()).unwrap();
                            (color, num)
                        })
                        .collect::<HashMap<_, _>>()
                })
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, strum_macros::EnumString)]
#[strum(serialize_all = "snake_case")]
enum Color {
    Red,
    Green,
    Blue,
}

#[test]
fn test_parse_game() {
    let game = Game::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
    assert_eq!(
        game,
        Game {
            id: 1,
            turns: vec![
                HashMap::from([(Color::Blue, 3), (Color::Red, 4)]),
                HashMap::from([(Color::Red, 1), (Color::Green, 2), (Color::Blue, 6)]),
                HashMap::from([(Color::Green, 2)]),
            ]
        }
    );
}
