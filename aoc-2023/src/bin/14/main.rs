#![allow(dead_code, unused)]

use std::{fmt::Display, mem, time::Instant};

use itertools::Itertools;
fn main() {
    let ex1 = include_str!("ex1.txt");
    let in1 = include_str!("in1.txt");

    println!("p1ex1 = {}", tilt_load(ex1, Dir::North));
    println!("p1in1 = {}", tilt_load(in1, Dir::North));

    let start = Instant::now();
    cycle(ex1, 1000000);
}

fn cycle(input: &str, amt: usize) {
    let mut map = Map::parse(input);
    let start = Instant::now();
    for _ in 0..amt {
        map.tilt(Dir::North);
    }
    let dur = start.elapsed();
    let per_sec = (amt as f64) / dur.as_secs_f64();
    println!("cycle dur: {dur:?} per/sec: {per_sec:.2}");
}

fn tilt_load(input: &str, dir: Dir) -> usize {
    let mut map = Map::parse(input);
    map.tilt(dir);
    map.load()
}

fn cycle_load(input: &str) -> usize {
    let mut map = Map::parse(input);
    let dirs = [Dir::North, Dir::West, Dir::South, Dir::East];
    let mut idx = 0;
    const NUM_ITERS: usize = 1000000000;
    while idx < NUM_ITERS {
        if idx % 100000 == 0 {
            println!("idx: {idx} {:.2}", (idx as f64) / NUM_ITERS as f64 * 100.0);
        }
        map.tilt(dirs[idx % dirs.len()]);
        idx += 1;
    }
    map.load()
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Map {
    rows: usize,
    cols: usize,
    tiles: Vec<Tile>,
}

impl Map {
    fn parse(input: &str) -> Self {
        let tiles = input
            .trim()
            .lines()
            .map(|row| row.trim().chars().map(Tile::from).collect_vec())
            .collect_vec();
        assert!(tiles.iter().map(|r| r.len()).all_equal());
        let rows = tiles.len();
        let cols = tiles.first().map(|r| r.len()).unwrap_or_default();
        Self {
            rows,
            cols,
            tiles: tiles.concat(),
        }
    }

    fn load(&self) -> usize {
        let rows = self.rows;
        let cols = self.cols;
        (0..cols)
            .map(|x| {
                (0..rows)
                    .map(move |y| (x, y))
                    .map(|(x, y)| (y, self.get_xy(x, y)))
                    .filter(|(_, t)| t == &Tile::Round)
                    .map(|(mut y, _t)| rows - y)
                    .sum::<usize>()
            })
            .sum()
    }

    fn tilt(&mut self, dir: Dir) {
        let rows = self.rows;
        let cols = self.cols;
        let len = rows * cols;
        let tiles = self.tiles.as_mut_slice();
        let xy = |x: usize, y: usize| -> usize { y * cols + x };
        let mut swap = |src: usize, dst: usize| -> bool {
            //println!("{tiles:?}");
            //println!("swapping {src},{dst} {:?},{:?}", tiles[src], tiles[dst]);
            if &tiles[src] != &Tile::Round {
                return false;
            }
            if &tiles[dst] != &Tile::Space {
                return false;
            }
            tiles.swap(src, dst);
            true
        };
        match dir {
            Dir::North => {
                for idx in (0..len).skip(cols) {
                    let mut y = idx / cols;
                    let x = idx % cols;
                    while y > 0 && swap(xy(x, y), xy(x, y - 1)) {
                        y -= 1;
                    }
                }
            }
            Dir::South => {
                for idx in (0..len).rev().skip(cols) {
                    let mut y = idx / cols;
                    let x = idx % cols;
                    while y < rows - 1 && swap(xy(x, y), xy(x, y + 1)) {
                        y += 1;
                    }
                }
            }
            Dir::East => {
                for idx in (0..rows)
                    .cycle()
                    .zip((0..cols - 1).cycle().take(len - rows))
                    .map(|(y, x)| y * (cols - 1) + x)
                {
                    let y = idx / cols;
                    let mut x = idx % cols;
                    while x < cols - 1 && swap(xy(x, y), xy(x + 1, y)) {
                        x += 1;
                    }
                }
            }
            Dir::West => {
                for idx in (0..rows)
                    .cycle()
                    .zip((1..cols).rev().cycle().take(len - rows))
                    .map(|(y, x)| y * cols + x)
                {
                    let y = idx / cols;
                    let mut x = idx % cols;
                    while x > 0 && swap(xy(x, y), xy(x - 1, y)) {
                        x -= 1;
                    }
                }
            }
        }
    }

    fn drop_rock(&mut self, src: (usize, usize), dst: (usize, usize)) -> bool {
        let src_off = self.cols * src.1 + src.0;
        let dst_off = self.cols * dst.1 + dst.0;
        let tiles = self.tiles.as_mut_slice();
        if &tiles[src_off] != &Tile::Round {
            return false;
        }
        if &tiles[dst_off] != &Tile::Space {
            return false;
        }
        let tiles = self.tiles.as_mut_slice();
        tiles.swap(src_off, dst_off);
        true
    }

    fn set_xy(&mut self, x: usize, y: usize, tile: Tile) {
        let dt = self
            .tiles
            .get_mut(self.cols * y + x)
            .expect("no tile found");
        *dt = tile
    }

    fn get_xy(&self, x: usize, y: usize) -> Tile {
        self.tiles
            .get(self.cols * y + x)
            .expect("no tile found")
            .clone()
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .tiles
            .chunks(self.cols)
            .map(|r| r.iter().map(|t| t.ch()).collect::<String>())
            .join("\n");
        writeln!(f, "{s}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum_macros::EnumIs)]
enum Tile {
    Space,
    Round,
    Cube,
}

impl Tile {
    fn from(ch: char) -> Self {
        match ch {
            '.' => Self::Space,
            'O' => Self::Round,
            '#' => Self::Cube,
            _ => panic!("bad ch: {ch}"),
        }
    }
    fn ch(&self) -> char {
        match self {
            Tile::Space => '.',
            Tile::Round => 'O',
            Tile::Cube => '#',
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    North,
    South,
    East,
    West,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_tilt() {
        let m1 = "
            O.#.
            O..#
            .OOO
            ";
        let mut map = Map::parse(m1);
        println!("{map}\n");
        map.tilt(Dir::North);
        println!("{map}\n");
        assert_eq!(
            map,
            Map::parse(
                "
            OO#.
            O.O#
            ...O
            "
            )
        );
        map.tilt(Dir::South);
        println!("{map}\n");
        assert_eq!(
            map,
            Map::parse(
                "
            ..#.
            O..#
            OOOO
            "
            )
        );
        map.tilt(Dir::East);
        println!("{map}\n");
        assert_eq!(
            map,
            Map::parse(
                "
            ..#.
            ..O#
            OOOO
            "
            )
        );
        map.tilt(Dir::West);
        println!("{map}\n");
        assert_eq!(
            map,
            Map::parse(
                "
            ..#.
            O..#
            OOOO
            "
            )
        );
    }

    #[test]
    fn test_cycle() {
        let ex1 = include_str!("ex1.txt");
        let mut map = Map::parse(ex1);
        for _ in 0..1000 {
            map.tilt(Dir::North);
        }
    }

    #[test]
    fn test_pt2() {
        let ex1 = include_str!("ex1.txt");
        assert_eq!(cycle_load(ex1), 64);
    }

    #[test]
    fn test_pt1() {
        let ex1 = include_str!("ex1.txt");
        let mut map = Map::parse(ex1);
        map.tilt(Dir::North);
        assert_eq!(map.load(), 136);
        let in1 = include_str!("in1.txt");
        let mut map = Map::parse(in1);
        map.tilt(Dir::North);
        assert_eq!(map.load(), 109596);
    }

    #[test]
    fn test_drop() {
        let ex1 = include_str!("ex1.txt");
        let ex1n = include_str!("ex1_north.txt");
        let mut map = Map::parse(ex1);
        println!("{map}");
        println!("{ex1n}");
        map.tilt(Dir::North);
        println!("{map}");
        assert_eq!(map.to_string(), ex1n);
    }

    #[test]
    fn test_parse() {
        let ex1 = include_str!("ex1.txt");
        let map = Map::parse(ex1);
        assert_eq!(map.cols, 10);
        assert_eq!(map.rows, 10);
        assert_eq!(map.tiles.len(), 100);
        assert_eq!(map.to_string(), ex1);
    }
}
