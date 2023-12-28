use core::panic;
use itertools::Itertools;
use std::fmt::Display;

fn main() {
    let ex1 = include_str!("ex1.txt");
    let in1 = include_str!("in1.txt");

    println!("p1ex1 = {}", tilt_load(ex1, Dir::North));
    println!("p1in1 = {}", tilt_load(in1, Dir::North));
    println!("p2ex1 = {}", cycle_load(ex1));
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
    const NUM_CYCLES: usize = 1000000000;
    while idx < NUM_CYCLES {
        if idx % 100000 == 0 {
            println!("idx: {idx} {:.2}", (idx as f64) / NUM_CYCLES as f64 * 100.0);
        }
        for d in 0..4 {
            map.tilt(dirs[d]);
        }
        idx += 1;
    }
    map.load()
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Map {
    rows: usize,
    cols: usize,
    tiles: Vec<Tile>,
    rounds: Vec<usize>,
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
        let rounds = tiles
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|t| t.1.is_round())
                    .map(move |t| (t.0, y))
            })
            .map(|(x, y)| y * cols + x)
            .collect();
        Self {
            rows,
            cols,
            tiles: tiles.concat(),
            rounds,
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
                    .map(|(y, _t)| rows - y)
                    .sum::<usize>()
            })
            .sum()
    }

    fn swap(&mut self, src: usize, dst: usize, rsrc: usize) -> bool {
        if &self.tiles[src] != &Tile::Round {
            return false;
        }
        if &self.tiles[dst] != &Tile::Space {
            return false;
        }
        self.tiles.swap(src, dst);
        self.rounds[rsrc] = dst;
        /*
        if !self.rounds.remove(&src) {
            panic!("Src {src} was not in rounds set: {:?}", self.rounds);
        }
        if !self.rounds.insert(dst) {
            panic!("Dst {dst} was already in the rounds set: {:?}", self.rounds);
        }
        */
        true
    }

    fn round_xys(&self) -> Vec<(usize, (usize, usize))> {
        self.rounds
            .iter()
            .enumerate()
            .map(|(idx, r)| (idx, (*r % self.cols, *r / self.cols)))
            .collect()
    }

    fn tilt(&mut self, dir: Dir) {
        let rows = self.rows;
        let cols = self.cols;
        let xy = |x: usize, y: usize| -> usize { y * cols + x };
        let mut rounds = self.round_xys();
        match dir {
            Dir::North => {
                rounds.sort_by(|r1, r2| r1.1 .1.cmp(&r2.1 .1));
                for (ridx, (x, mut y)) in rounds {
                    while y > 0 && self.swap(xy(x, y), xy(x, y - 1), ridx) {
                        y -= 1;
                    }
                }
            }
            Dir::South => {
                rounds.sort_by(|r1, r2| r1.1 .1.cmp(&r2.1 .1).reverse());
                for (ridx, (x, mut y)) in rounds {
                    while y < rows - 1 && self.swap(xy(x, y), xy(x, y + 1), ridx) {
                        y += 1;
                    }
                }
            }
            Dir::East => {
                rounds.sort_by(|r1, r2| r1.1 .0.cmp(&r2.1 .0).reverse());
                for (ridx, (mut x, y)) in rounds {
                    while x < cols - 1 && self.swap(xy(x, y), xy(x + 1, y), ridx) {
                        x += 1;
                    }
                }
            }
            Dir::West => {
                rounds.sort_by(|r1, r2| r1.1 .0.cmp(&r2.1 .0));
                for (ridx, (mut x, y)) in rounds {
                    while x > 0 && self.swap(xy(x, y), xy(x - 1, y), ridx) {
                        x -= 1;
                    }
                }
            }
        }
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
