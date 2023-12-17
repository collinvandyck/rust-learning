#![allow(dead_code, unused)]

use itertools::Itertools;

fn main() {
    let ex1 = include_str!("ex1.txt");
    let in1 = include_str!("in1.txt");
    let map = Map::from_input(ex1);
    println!("{map}");
}

struct Map {
    tiles: Vec<Vec<Tile>>,
    path: Vec<Tile>,
    fancy: bool,
}

impl Map {
    fn from_input(input: &str) -> Self {
        let mut map = Self {
            fancy: true,
            path: vec![],
            tiles: input
                .lines()
                .enumerate()
                .map(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .map(|(x, ch)| Tile::new(x, y, Glyph::from_char(ch)))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
        };
        map
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rendered = self
            .tiles
            .iter()
            .map(|row| {
                row.iter()
                    .map(|t| t.glyph.render(self.fancy))
                    .collect::<String>()
            })
            .join("\n");
        write!(f, "{rendered}")
    }
}

struct Tile {
    glyph: Glyph,
    x: usize,
    y: usize,
}

impl Tile {
    fn new(x: usize, y: usize, glyph: Glyph) -> Self {
        Self { x, y, glyph }
    }
}

enum Glyph {
    VPipe,
    HPipe,
    BendNE,
    BendNW,
    BendSE,
    BendSW,
    Ground,
    Start,
}

impl Glyph {
    fn from_char(ch: char) -> Self {
        match ch {
            '|' => Self::VPipe,
            '-' => Self::HPipe,
            'L' => Self::BendSW,
            'J' => Self::BendSE,
            '7' => Self::BendNE,
            'F' => Self::BendNW,
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => panic!("unexpected ch: {ch}"),
        }
    }
    fn render(&self, fancy: bool) -> char {
        match self {
            Self::VPipe if fancy => '║',
            Self::VPipe => '|',
            Self::HPipe if fancy => '═',
            Self::HPipe => '-',
            Self::BendNE if fancy => '╗',
            Self::BendNE => '7',
            Self::BendNW if fancy => '╔',
            Self::BendNW => 'F',
            Self::BendSE if fancy => '╝',
            Self::BendSE => 'J',
            Self::BendSW if fancy => '╚',
            Self::BendSW => 'L',
            Self::Ground => '.',
            Self::Start => 'S',
        }
    }
}
