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
    path: Vec<Move>,
    start: Tile,
    fancy: bool,
}

struct Move {
    tile: Tile,
    dir: Option<Dir>,
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
        self.set_start_tile();
    }

    fn set_start_tile(&mut self) {
        let mut dirs: Vec<_> = self
            .neighbors(self.start)
            .into_iter()
            .filter(|(dir, tile)| {
                tile.glyph
                    .connectors()
                    .iter()
                    .any(|f| f.contains(&dir.negate()))
            })
            .map(|(dir, _)| dir)
            .collect();
        assert_eq!(dirs.len(), 2);
        dirs.sort(); // sort by ord to make matching easier
        let glyph = match (dirs[0], dirs[1]) {
            (Dir::Down, Dir::Right) => Glyph::BendNW,
            (Dir::Down, Dir::Left) => Glyph::BendNE,
            (Dir::Up, Dir::Right) => Glyph::BendSW,
            (Dir::Up, Dir::Left) => Glyph::BendSE,
            (Dir::Left, Dir::Right) => Glyph::HPipe,
            (Dir::Up, Dir::Down) => Glyph::VPipe,
            (d1, d2) => panic!("Unknown dir combo {d1:?} & {d2:?}"),
        };
        self.set_glyph(self.start.x, self.start.y, glyph);
    }

    fn neighbors(&self, tile: Tile) -> Vec<(Dir, Tile)> {
        let neighbors = Dir::iter()
            .flat_map(|dir| self.neighbor(tile, dir).map(|t| (dir, t)))
            .collect::<Vec<_>>();
        neighbors
    }

    fn neighbor(&self, tile: Tile, dir: Dir) -> Option<Tile> {
        let Tile { glyph, x, y } = tile;
        match dir {
            Dir::Up => y.checked_sub(1).map(|y| (x, y)),
            Dir::Down => y.checked_add(1).map(|y| (x, y)),
            Dir::Left => x.checked_sub(1).map(|x| (x, y)),
            Dir::Right => x.checked_add(1).map(|x| (x, y)),
        }
        .and_then(|(x, y)| self.get(x, y))
        .filter(|tile| tile.glyph.is_pipe())
    }

    fn get(&self, x: usize, y: usize) -> Option<Tile> {
        self.tiles.get(y).and_then(|r| r.get(x)).copied()
    }

    fn set_glyph(&mut self, x: usize, y: usize, glyph: Glyph) {
        if let Some(row) = self.tiles.get_mut(y) {
            if let Some(tile) = row.get_mut(x) {
                tile.glyph = glyph;
            }
        }
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
    fn connectors(&self) -> Option<[Dir; 2]> {
        match self {
            Self::VPipe => Some([Dir::Up, Dir::Down]),
            Self::HPipe => Some([Dir::Left, Dir::Right]),
            Self::BendNE => Some([Dir::Left, Dir::Down]),
            Self::BendNW => Some([Dir::Right, Dir::Down]),
            Self::BendSE => Some([Dir::Left, Dir::Up]),
            Self::BendSW => Some([Dir::Right, Dir::Up]),
            _ => None,
        }
    }
    fn is_pipe(&self) -> bool {
        matches!(
            self,
            Self::VPipe | Self::HPipe | Self::BendNW | Self::BendNE | Self::BendSW | Self::BendSE
        )
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

#[derive(EnumIter, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn negate(&self) -> Dir {
        match self {
            Dir::Up => Dir::Down,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Right,
            Dir::Right => Dir::Left,
        }
    }
}
