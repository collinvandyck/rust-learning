#![allow(dead_code, unused)]

use std::{collections::HashMap, str::FromStr};

use once_cell::sync::Lazy;
use regex::Regex;
use strum::IntoEnumIterator;

fn main() {
    let example = include_str!("example.txt");
    let input = include_str!("input.txt");
    let bag = Cubes(HashMap::from([
        (Color::Red, 12),
        (Color::Green, 13),
        (Color::Blue, 14),
    ]));
    println!("p1ex={}", possible_games(example, &bag));
    println!("p1in={}", possible_games(input, &bag));
}

fn possible_games(input: &str, bag: &Cubes) -> u64 {
    input
        .lines()
        .map(|line| Game::from(line))
        .filter(|game| game.possible(&bag))
        .map(|g| g.id)
        .sum()
}

fn power_minimums(input: &str) {
    input.lines().map(Game::from).map(|g| {
        let min = g.minimum();
        (g, min)
    });
}

fn minimum_sets(input: &str) -> impl Iterator<Item = Cubes> + '_ {
    input
        .lines()
        .map(|line| Game::from(line))
        .map(|game| game.minimum())
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Cubes(HashMap<Color, u64>);

impl Cubes {
    fn get(&self, color: Color) -> u64 {
        self.0.get(&color).copied().unwrap_or_default()
    }
    fn power(&self) -> u64 {
        Color::iter().map(|c| self.get(c)).product()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Game {
    id: u64,
    turns: Vec<Cubes>,
}

impl Game {
    fn minimum(&self) -> Cubes {
        todo!()
    }
    fn possible(&self, bag: &Cubes) -> bool {
        self.turns.iter().all(|turn| {
            Color::iter().all(|color| {
                let tc = turn.get(color);
                let bg = bag.get(color);
                tc <= bg
            })
        })
    }
    fn from(s: impl AsRef<str>) -> Self {
        static GAME_RE: Lazy<Regex> = Lazy::new(|| Regex::new("^Game (.*): (.*)").unwrap());
        let caps = GAME_RE.captures(s.as_ref()).unwrap();
        let (id, rest) = (caps.get(1).unwrap().as_str(), caps.get(2).unwrap().as_str());
        Self {
            id: id.parse().unwrap(),
            turns: rest
                .split("; ")
                .map(|turn| {
                    Cubes(
                        turn.split(", ")
                            .map(|num_color| {
                                let mut iter = num_color.trim().split(" ");
                                let num = iter.next().unwrap().parse::<u64>().unwrap();
                                let color = Color::from_str(iter.next().unwrap().trim()).unwrap();
                                (color, num)
                            })
                            .collect::<HashMap<_, _>>(),
                    )
                })
                .collect(),
        }
    }
}

#[derive(
    Debug, Clone, Copy, Hash, PartialEq, Eq, strum_macros::EnumString, strum_macros::EnumIter,
)]
#[strum(serialize_all = "snake_case")]
enum Color {
    Red,
    Green,
    Blue,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cube_power() {
        let cubes = Cubes(HashMap::from([
            (Color::Red, 2),
            (Color::Green, 3),
            (Color::Blue, 4),
        ]));
        assert_eq!(cubes.power(), 24);
        let cubes = Cubes(HashMap::from([
            (Color::Red, 2),
            (Color::Green, 0),
            (Color::Blue, 4),
        ]));
        assert_eq!(cubes.power(), 0);
        let cubes = Cubes(HashMap::from([(Color::Red, 2), (Color::Blue, 4)]));
        assert_eq!(cubes.power(), 0);
    }

    #[test]
    fn test_parse_game() {
        let game = Game::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        assert_eq!(
            game,
            Game {
                id: 1,
                turns: vec![
                    Cubes(HashMap::from([(Color::Blue, 3), (Color::Red, 4)])),
                    Cubes(HashMap::from([
                        (Color::Red, 1),
                        (Color::Green, 2),
                        (Color::Blue, 6)
                    ])),
                    Cubes(HashMap::from([(Color::Green, 2)])),
                ]
            }
        );
    }
}
