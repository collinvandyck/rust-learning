use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
};

fn main() {
    run();
}

fn run() {
    let mut game = Game::new();
    let file = File::open("strategy.txt").unwrap();
    let read = BufReader::new(file);
    for line in read.lines() {
        let line = line.unwrap();
        let chars: Vec<char> = line.chars().take(3).collect();
        let lhs = chars.get(0).unwrap();
        let rhs = chars.get(2).unwrap();
        let lhs_choice = Choice::from(lhs).unwrap();
        let rhs_choice = Choice::from(rhs).unwrap();
        game.make_move(lhs_choice, rhs_choice);
    }
}

#[derive(Debug, PartialEq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn outcome(&self, other: &Self) -> Outcome {
        match (self, other) {
            (Choice::Rock, Choice::Scissors) => Outcome::Win,
            (Choice::Scissors, Choice::Rock) => Outcome::Lose,
            (Choice::Paper, Choice::Rock) => Outcome::Win,
            (Choice::Rock, Choice::Paper) => Outcome::Lose,
            (Choice::Scissors, Choice::Paper) => Outcome::Win,
            (Choice::Paper, Choice::Scissors) => Outcome::Lose,
            _ => Outcome::Draw,
        }
    }
}

#[derive(Debug)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

struct Player {
    score: i32,
}

impl Player {
    fn new() -> Self {
        Self { score: 0 }
    }
}

struct Game {
    player_one: Player,
    player_two: Player,
}

impl Game {
    fn new() -> Self {
        Self {
            player_one: Player::new(),
            player_two: Player::new(),
        }
    }
    fn make_move(&mut self, choice_one: Choice, choice_two: Choice) {
        let outcome = choice_one.outcome(&choice_two);
        dbg!((choice_one, choice_two, outcome));
    }
}

impl Choice {
    fn from(value: &char) -> Option<Self> {
        match value {
            'A' | 'X' => Some(Choice::Rock),
            'B' | 'Y' => Some(Choice::Paper),
            'C' | 'Z' => Some(Choice::Scissors),
            _ => None,
        }
    }
}
