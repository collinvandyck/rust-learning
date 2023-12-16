#![allow(unused, dead_code)]

use itertools::Itertools;
use std::{collections::HashSet, fmt::Debug};

fn main() {
    let ex1 = include_str!("ex1.txt");
    let ex2 = include_str!("ex2.txt");
    let in1 = include_str!("in1.txt");
    println!("p1ex={}", farthest_distance(ex1));
    //println!("p2in={}", farthest_distance(in1));
    //println!("p2ex1={}", area_enclosed(ex1));
}

fn farthest_distance(input: &str) -> usize {
    let map = parse(input);
    map.loop_pts.len() / 2
}

fn area_enclosed(input: &str) -> usize {
    let map = parse(input);
    map.area()
}

struct Map {
    start: Pt,
    tiles: Vec<Vec<Tile>>,
    loop_pts: HashSet<Pt>,
}

impl Map {
    fn new(tiles: Vec<Vec<Tile>>) -> Self {
        let start = Self::find_tile(&tiles, Tile::Start)
            .first()
            .copied()
            .unwrap();
        let mut map = Self {
            tiles,
            start,
            loop_pts: Default::default(),
        };
        map.swap_start();
        map.walk();
        map
    }

    // pick the first tile found on the first row and walk the loop
    fn area(&self) -> usize {
        println!("{self}");
        todo!()
    }

    fn walk(&mut self) {
        let mut visited: HashSet<Pt> = HashSet::default();
        let start = self.get(self.start.x, self.start.y).expect("no start");
        let mut cur = start.clone();
        let mut last_dir: Option<Dir> = None;
        while !visited.contains(&start) {
            let dir = cur
                .tile
                .dirs()
                .unwrap()
                .into_iter()
                .filter(|dir| match &last_dir {
                    Some(last) if dir != &last.opposite() => true,
                    Some(last) => false,
                    None => true,
                })
                .next()
                .unwrap();
            println!("Dir is {dir:?}");
            return;
        }
    }

    fn find_loop(&mut self) {
        let mut visited: HashSet<Pt> = HashSet::default();
        let cur = self.get(self.start.x, self.start.y).expect("no start");
        let mut queue = vec![cur];
        while let Some(pt) = queue.pop() {
            if let Some(dirs) = pt.tile.dirs() {
                for pt in dirs.into_iter().flat_map(|d| self.neighbor(pt, d)) {
                    if !visited.contains(&pt) {
                        queue.push(pt);
                    }
                }
            }
            visited.insert(pt);
        }
        for (y, row) in self.tiles.iter_mut().enumerate() {
            for (x, t) in row.iter_mut().enumerate() {
                let pt = Pt::new(x, y, *t);
                if !visited.contains(&pt) {
                    *t = Tile::Ground;
                }
            }
        }
        self.loop_pts = visited;
    }

    fn swap_start(&mut self) {
        use Dir::*;
        let u = self.neighbor(self.start, Up).map(|p| p.tile);
        let d = self.neighbor(self.start, Down).map(|p| p.tile);
        let l = self.neighbor(self.start, Left).map(|p| p.tile);
        let r = self.neighbor(self.start, Right).map(|p| p.tile);
        let tile = match (u, d, l, r) {
            (Some(u), Some(d), _, _) if u.has(Down) && d.has(Up) => Tile::VPipe,
            (_, _, Some(l), Some(r)) if l.has(Right) && r.has(Left) => Tile::HPipe,
            (_, Some(d), _, Some(r)) if d.has(Up) && r.has(Left) => Tile::BendSE,
            (_, Some(d), Some(l), _) if d.has(Up) && l.has(Right) => Tile::BendSW,
            (Some(u), _, _, Some(r)) if u.has(Down) && r.has(Left) => Tile::BendNE,
            (Some(u), _, Some(l), _) if u.has(Down) && l.has(Right) => Tile::BendNW,
            _ => panic!("no start could be found"),
        };
        self.set(self.start.x, self.start.y, tile);
    }

    fn neighbor(&self, pt: Pt, dir: Dir) -> Option<Pt> {
        match dir {
            Dir::Up => pt.y.checked_sub(1).map(|y| (pt.x, y)),
            Dir::Down => pt.y.checked_add(1).map(|y| (pt.x, y)),
            Dir::Left => pt.x.checked_sub(1).map(|x| (x, pt.y)),
            Dir::Right => pt.x.checked_add(1).map(|x| (x, pt.y)),
        }
        .map(|(x, y)| self.get(x, y))
        .flatten()
    }

    fn set(&mut self, x: usize, y: usize, tile: Tile) {
        if let Some(row) = self.tiles.get_mut(y) {
            if let Some(v) = row.get_mut(x) {
                *v = tile;
            }
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<Pt> {
        self.tiles
            .get(y)
            .map(|row| row.get(x))
            .flatten()
            .copied()
            .map(|tile| Pt::new(x, y, tile))
    }

    fn find_tile(tiles: &[Vec<Tile>], tile: Tile) -> Vec<Pt> {
        tiles
            .iter()
            .enumerate()
            .map(|(row, tiles)| {
                tiles
                    .iter()
                    .enumerate()
                    .filter(|(_col, t)| t == &&tile)
                    .map(|(col, t)| (col, row, *t))
                    .collect::<Vec<_>>()
            })
            .flatten()
            .map(|(x, y, tile)| Pt::new(x, y, tile))
            .collect::<Vec<_>>()
    }
}

fn lr_connects(left: &Pt, right: &Pt) -> bool {
    let distance = right.x.saturating_sub(left.x);
    let left_ok = left.tile.has(Dir::Right);
    let right_ok = right.tile.has(Dir::Left);
    distance == 1 && left_ok && right_ok
}

#[test]
fn test_lr_connects() {
    assert!(lr_connects(
        &Pt::new(0, 4, Tile::BendNE),
        &Pt::new(1, 4, Tile::HPipe)
    ))
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .tiles
            .iter()
            .map(|row| row.iter().map(|t| format!("{t}")).join(""))
            .join("\n");
        write!(f, "{s}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn opposite(&self) -> Dir {
        match self {
            Dir::Up => Dir::Down,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Right,
            Dir::Right => Dir::Left,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Pt {
    x: usize,
    y: usize,
    tile: Tile,
    dist: usize,
}

impl Pt {
    fn new(x: usize, y: usize, tile: Tile) -> Self {
        Self {
            x,
            y,
            tile,
            dist: 0,
        }
    }
}

impl Debug for Pt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})::{:?}", self.x, self.y, self.tile)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    VPipe,
    HPipe,
    BendNE,
    BendNW,
    BendSW,
    BendSE,
    Ground,
    Start,
}

impl Tile {
    fn dirs(&self) -> Option<[Dir; 2]> {
        match self {
            Tile::VPipe => Some([Dir::Up, Dir::Down]),
            Tile::HPipe => Some([Dir::Left, Dir::Right]),
            Tile::BendNE => Some([Dir::Up, Dir::Right]),
            Tile::BendNW => Some([Dir::Up, Dir::Left]),
            Tile::BendSE => Some([Dir::Down, Dir::Right]),
            Tile::BendSW => Some([Dir::Down, Dir::Left]),
            _ => None,
        }
    }
    fn has(&self, dir: Dir) -> bool {
        use Tile::*;
        match (self, dir) {
            (VPipe | BendNE | BendNW, Dir::Up) => true,
            (VPipe | BendSE | BendSW, Dir::Down) => true,
            (HPipe | BendNW | BendSW, Dir::Left) => true,
            (HPipe | BendNE | BendSE, Dir::Right) => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Tile::VPipe => "║",
            Tile::HPipe => "═",
            Tile::BendNE => "╚",
            Tile::BendNW => "╝",
            Tile::BendSW => "╗",
            Tile::BendSE => "╔",
            Tile::Ground => ".",
            Tile::Start => "S",
        };
        /*
        let s = match self {
            Tile::VPipe => "|",
            Tile::HPipe => "-",
            Tile::BendNE => "L",
            Tile::BendNW => "J",
            Tile::BendSW => "7",
            Tile::BendSE => "F",
            Tile::Ground => ".",
            Tile::Start => "S",
        };
        */
        write!(f, "{s}")
    }
}

impl From<char> for Tile {
    fn from(ch: char) -> Self {
        match ch {
            '.' => Tile::Ground,
            '|' => Tile::VPipe,
            '-' => Tile::HPipe,
            'L' => Tile::BendNE,
            'J' => Tile::BendNW,
            '7' => Tile::BendSW,
            'F' => Tile::BendSE,
            'S' => Tile::Start,
            _ => panic!("unknown tile: {ch}"),
        }
    }
}

fn parse(input: &str) -> Map {
    Map::new(input.lines().map(parse_row).collect())
}

fn parse_row(input: &str) -> Vec<Tile> {
    input.chars().map(Tile::from).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_map() {
        let example = include_str!("ex1.txt");
        let map = parse(example);
        assert_eq!(map.start, Pt::new(0, 2, Tile::Start));
    }
}
