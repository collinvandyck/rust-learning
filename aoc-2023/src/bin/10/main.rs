#![allow(dead_code, unused)]

use std::{collections::HashSet, thread::yield_now, time::Instant};

use itertools::Itertools;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use tracing::{debug, info};

fn main() {
    tracing_subscriber::fmt().init();
    let start = Instant::now();
    let ex1 = include_str!("ex1.txt");
    let ex2 = include_str!("ex2.txt");
    let ex3 = include_str!("ex3.txt");
    let ex4 = include_str!("ex4.txt");
    let in1 = include_str!("in1.txt");
    info!("p1ex1={}", farthest_distance(ex1));
    info!("p1in1={}", farthest_distance(in1));
    info!("p2ex1={}", enclosed_area(ex1));
    info!("p2ex2={}", enclosed_area(ex2));
    info!("p2ex3={}", enclosed_area(ex3));
    info!("p2ex4={}", enclosed_area(ex4));
    //info!("p2in1={}", enclosed_area(in1));
    info!(elapsed = ?start.elapsed(), "Done")
}

fn farthest_distance(input: &str) -> usize {
    let map = Map::from_input(input);
    map.path.len() / 2
}

fn enclosed_area(input: &str) -> usize {
    let mut map = Map::from_input(input);
    assert!(map.path_is_clockwise());
    map.mark_interior();
    info!("\n{map}");
    0
}

struct Map {
    tiles: Vec<Vec<Tile>>,
    path: Vec<Move>,
    start: Tile,
    fancy: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Move {
    tile: Tile,
    dir: Option<Dir>,
}

impl Move {
    fn new(tile: Tile, dir: Option<Dir>) -> Self {
        Self { tile, dir }
    }
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
        let mut map = Self {
            fancy: true,
            path: vec![],
            start,
            tiles,
        };
        map.set_start_tile();
        map.chart_loop();
        map
    }

    fn mark_interior(&mut self) {
        let mut visited: HashSet<Tile> = HashSet::default();
        // fill from the top and bottom
        for x in 0..self.cols() {
            for (x, y) in [(x, 0), (x, self.rows() - 1)] {
                self.flood_exterior(x, y, &mut visited);
            }
        }
        // fill from the left and the right
        for y in 0..self.rows() {
            for (x, y) in [(0, y), (self.cols() - 1, y)] {
                self.flood_exterior(x, y, &mut visited);
            }
        }
    }

    // given a tile, mark it as exterior if it is ground and explore the different edges
    fn flood_exterior(&mut self, x: usize, y: usize, visited: &mut HashSet<Tile>) {
        let tile = self.get(x, y).unwrap();
        if tile.glyph != Glyph::Ground {
            return;
        }
        let mut queue = vec![tile];
        while let Some(tile) = queue.pop() {
            if visited.contains(&tile) {
                continue;
            }
            visited.insert(tile);
            if tile.glyph == Glyph::Ground {
                self.set_status(&tile, Status::Exterior);
                for dir in Dir::iter() {
                    if let Some(tile) = self.neighbor(tile, dir) {
                        queue.push(tile);
                    }
                }
            }
        }
    }

    fn set_status(&mut self, tile: &Tile, status: Status) {
        let tile = self.get_mut(tile.x, tile.y).unwrap();
        tile.status = status;
    }

    fn rows(&self) -> usize {
        self.tiles.len()
    }

    fn cols(&self) -> usize {
        self.tiles.get(0).map(|r| r.len()).unwrap_or_default()
    }

    fn chart_loop(&mut self) {
        let mut visited: HashSet<Tile> = HashSet::default();
        let start = self.get(self.start.x, self.start.y).unwrap();
        visited.insert(start);
        self.path.push(Move::new(start.clone(), None));
        while let Some(Move { tile, dir }) = self.path.last() {
            let dir = tile
                .glyph
                .connectors()
                .expect("no connectors")
                .iter()
                .copied()
                .filter(|d| {
                    if let Some(dir) = dir {
                        dir != &d.negate()
                    } else {
                        d == &match tile.glyph {
                            Glyph::VPipe => Dir::Up,
                            Glyph::HPipe => Dir::Right,
                            Glyph::BendNE => Dir::Down,
                            Glyph::BendNW => Dir::Right,
                            Glyph::BendSE => Dir::Left,
                            Glyph::BendSW => Dir::Up,
                            _ => panic!("non pipe tile"),
                        }
                    }
                })
                .next()
                .expect("no initial dir");
            let next = self.neighbor(*tile, dir).unwrap();
            if next.x == self.start.x && next.y == self.start.y {
                break;
            }
            visited.insert(next);
            self.path.push(Move::new(next, Some(dir)));
        }
        for row in self.tiles.iter_mut() {
            for tile in row.iter_mut() {
                if !visited.contains(tile) {
                    tile.glyph = Glyph::Ground;
                }
            }
        }
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

    fn path_is_clockwise(&self) -> bool {
        self.path
            .iter()
            .map(|m| m.tile)
            .filter(|tile| matches!(tile.glyph, Glyph::HPipe | Glyph::BendNE | Glyph::BendNW))
            .any(|tile| self.ray_ground(tile, Dir::Up, Glyph::Ground))
    }

    fn ray_ground(&self, mut tile: Tile, dir: Dir, glyph: Glyph) -> bool {
        while let Some(t) = self.neighbor(tile, dir) {
            if !matches!(t.glyph, Glyph::Ground) {
                return false;
            }
            tile = t;
        }
        true
    }

    fn neighbors(&self, tile: Tile) -> Vec<(Dir, Tile)> {
        let neighbors = Dir::iter()
            .flat_map(|dir| self.neighbor(tile, dir).map(|t| (dir, t)))
            .collect::<Vec<_>>();
        neighbors
    }

    fn neighbor(&self, tile: Tile, dir: Dir) -> Option<Tile> {
        let Tile { glyph, x, y, .. } = tile;
        match dir {
            Dir::Up => y.checked_sub(1).map(|y| (x, y)),
            Dir::Down => y.checked_add(1).map(|y| (x, y)),
            Dir::Left => x.checked_sub(1).map(|x| (x, y)),
            Dir::Right => x.checked_add(1).map(|x| (x, y)),
        }
        .and_then(|(x, y)| self.get(x, y))
    }

    fn neighbor_mut(&mut self, tile: Tile, dir: Dir) -> Option<&mut Tile> {
        let Tile { glyph, x, y, .. } = tile;
        match dir {
            Dir::Up => y.checked_sub(1).map(|y| (x, y)),
            Dir::Down => y.checked_add(1).map(|y| (x, y)),
            Dir::Left => x.checked_sub(1).map(|x| (x, y)),
            Dir::Right => x.checked_add(1).map(|x| (x, y)),
        }
        .and_then(|(x, y)| self.get_mut(x, y))
    }

    fn get(&self, x: usize, y: usize) -> Option<Tile> {
        self.tiles.get(y).and_then(|r| r.get(x)).copied()
    }

    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Tile> {
        self.tiles.get_mut(y).and_then(|r| r.get_mut(x))
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
            .map(|row| row.iter().map(|t| t.to_string()).collect::<String>())
            .join("\n");
        write!(f, "{rendered}")
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Tile {
    glyph: Glyph,
    x: usize,
    y: usize,
    status: Status,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Status {
    Interior,
    Exterior,
    Unknown,
}

impl Tile {
    fn new(x: usize, y: usize, glyph: Glyph) -> Self {
        Self {
            x,
            y,
            glyph,
            status: Status::Unknown,
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch = match self.status {
            Status::Interior => 'I',
            Status::Exterior => 'O',
            Status::Unknown => self.glyph.render(true),
        };
        write!(f, "{ch}")
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
    fn dir_to(&self, other: &Glyph) -> Dir {
        info!("Dir from {self:?} to {other:?}");
        let g_conns = self.connectors();
        let o_conns = other.connectors();
        todo!()
    }
    fn connectors(&self) -> Option<[Dir; 2]> {
        let mut res = match self {
            Self::VPipe => Some([Dir::Up, Dir::Down]),
            Self::HPipe => Some([Dir::Left, Dir::Right]),
            Self::BendNE => Some([Dir::Left, Dir::Down]),
            Self::BendNW => Some([Dir::Right, Dir::Down]),
            Self::BendSE => Some([Dir::Left, Dir::Up]),
            Self::BendSW => Some([Dir::Right, Dir::Up]),
            _ => None,
        };
        if let Some(dirs) = res.as_mut() {
            dirs.sort();
        }
        res
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
