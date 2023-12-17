#![allow(dead_code, unused)]

use itertools::Itertools;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use tracing::{debug, info};

fn main() {
    tracing_subscriber::fmt().without_time().init();
    let ex1 = include_str!("ex1.txt");
    let in1 = include_str!("in1.txt");
    let map = Map::from_input(ex1);
    info!("map:\n{map}");
}

struct Map {
    tiles: Vec<Vec<Tile>>,
    path: Vec<Tile>,
    start: Tile,
    fancy: bool,
}

impl Map {
    fn from_input(input: &str) -> Self {
        let tiles = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, ch)| {
                        let glyph = Glyph::from_char(ch);
                        Tile::new(x, y, glyph)
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let start = tiles
            .iter()
            .flat_map(|r| r.iter())
            .find(|t| matches!(t.glyph, Glyph::Start))
            .copied()
            .unwrap();
        info!("Start: {start}");
        let mut map = Self {
            fancy: true,
            path: vec![],
            start,
            tiles,
        };
        map.walk();
        map
    }
    fn walk(&mut self) {
        info!("Walking.");
        let conn = self.neighbors(self.start);
    }
    fn neighbors(&self, tile: Tile) -> Vec<(Dir, Tile)> {
        info!(%tile, "Neighbors");
        Dir::iter()
            .flat_map(|dir| self.neighbor(tile, dir).map(|t| (dir, t)))
            .collect::<Vec<_>>()
    }
    fn neighbor(&self, tile: Tile, dir: Dir) -> Option<Tile> {
        let Tile { glyph, x, y } = tile;
        match dir {
            Dir::Up => todo!(),
            Dir::Down => todo!(),
            Dir::Left => todo!(),
            Dir::Right => todo!(),
        }
    }
    fn get(&self, x: usize, y: usize) -> Option<Tile> {
        self.tiles.get(y).and_then(|r| r.get(x)).copied()
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

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
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

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}::({},{})::{}",
            self.glyph,
            self.x,
            self.y,
            self.glyph.render(true)
        )
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
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

#[derive(EnumIter, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}
