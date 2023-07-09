use std::{
    fs::File,
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
        let opponent_move = Choice::from(lhs).unwrap();
        let desired_outcome = Outcome::from(rhs).unwrap();
        let your_move = opponent_move.for_opponent_outcome(&desired_outcome);
        game.make_move(&your_move, &opponent_move);
    }
    println!("Score: {}", game.score);
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn for_opponent_outcome(&self, outcome: &Outcome) -> Choice {
        match (self, outcome) {
            (Choice::Rock, Outcome::Win) => Choice::Paper,
            (Choice::Rock, Outcome::Lose) => Choice::Scissors,
            (Choice::Paper, Outcome::Win) => Choice::Scissors,
            (Choice::Paper, Outcome::Lose) => Choice::Rock,
            (Choice::Scissors, Outcome::Win) => Choice::Rock,
            (Choice::Scissors, Outcome::Lose) => Choice::Paper,
            _ => *self,
        }
    }
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
    fn score(&self) -> i32 {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }
}

#[derive(Debug)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    fn from(value: &char) -> Option<Self> {
        match value {
            'X' => Some(Outcome::Lose),
            'Y' => Some(Outcome::Draw),
            'Z' => Some(Outcome::Win),
            _ => None,
        }
    }
    fn score(&self) -> i32 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        }
    }
}

struct Game {
    score: i32,
}

impl Game {
    fn new() -> Self {
        Self { score: 0 }
    }
    fn make_move(&mut self, you: &Choice, opponent: &Choice) -> i32 {
        let outcome = you.outcome(&opponent);
        let score = outcome.score() + you.score();
        self.score += score;
        score
    }
}

impl Choice {
    fn from(value: &char) -> Option<Self> {
        match value {
            'A' => Some(Choice::Rock),
            'B' => Some(Choice::Paper),
            'C' => Some(Choice::Scissors),
            _ => None,
        }
    }
}
