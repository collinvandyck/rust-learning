use std::collections::HashMap;
use strum::IntoEnumIterator;
mod parser;

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
    println!("p2ex={}", power_minimums(example));
    println!("p2in={}", power_minimums(input));
}

fn possible_games(input: &str, bag: &Cubes) -> u64 {
    input
        .lines()
        .map(Game::from)
        .filter(|g| g.possible(&bag))
        .map(|g| g.id)
        .sum()
}

fn power_minimums(input: &str) -> u64 {
    input
        .lines()
        .map(Game::from)
        .map(|g| g.minimum())
        .map(|c| c.power())
        .sum()
}

#[derive(Clone, PartialEq, Eq, Debug, Default)]
struct Cubes(HashMap<Color, u64>);

impl Cubes {
    fn get_or_zero(&self, color: Color) -> u64 {
        self.0.get(&color).copied().unwrap_or_default()
    }
    fn power(&self) -> u64 {
        Color::iter().map(|c| self.get_or_zero(c)).product()
    }
    fn set_maxes(&mut self, other: &Cubes) {
        for color in Color::iter() {
            if let Some(min) = match (self.0.get(&color), other.0.get(&color)) {
                (None, Some(b)) => Some(b),
                (Some(a), None) => Some(a),
                (Some(a), Some(b)) => Some(a.max(b)),
                _ => None,
            } {
                self.0.insert(color, *min);
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Game {
    id: u64,
    turns: Vec<Cubes>,
}

impl Game {
    fn minimum(&self) -> Cubes {
        let mut acc = Cubes::default();
        for turn in &self.turns {
            acc.set_maxes(turn);
        }
        acc
    }
    fn possible(&self, bag: &Cubes) -> bool {
        self.turns.iter().all(|turn| {
            Color::iter().all(|color| {
                let tc = turn.get_or_zero(color);
                let bg = bag.get_or_zero(color);
                tc <= bg
            })
        })
    }
    fn from(s: impl AsRef<str>) -> Self {
        let (rest, game) = parser::parse_game(s.as_ref()).unwrap();
        assert_eq!(rest, "");
        game
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

    #[test]
    fn test_game_minimum() {
        let game = Game::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        let min = game.minimum();
        assert_eq!(
            min,
            Cubes(HashMap::from([
                (Color::Red, 4),
                (Color::Green, 2),
                (Color::Blue, 6)
            ]))
        );
    }
}
