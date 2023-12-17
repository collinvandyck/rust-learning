#![allow(unused, dead_code)]

use itertools::Itertools;
use std::{collections::HashSet, fmt::Debug};

fn main() {
    let ex1 = include_str!("ex1.txt");
    let ex2 = include_str!("ex2.txt");
    let in1 = include_str!("in1.txt");
    println!("p1ex={}", farthest_distance(ex1));
    println!("p1in={}", farthest_distance(in1));
    println!("p2ex={}", area_enclosed(ex1));
    //println!("p2in={}", area_enclosed(in1));
}

fn farthest_distance(input: &str) -> usize {
    let map = parse(input);
    map.path.len() / 2
}

fn area_enclosed(input: &str) -> usize {
    let map = parse(input);
    map.area()
}

struct Map {
    start: Pt,
    pts: Vec<Vec<Pt>>,
    path: Vec<Pt>,
}

impl Map {
    fn new(pts: Vec<Vec<Pt>>) -> Self {
        let start = Self::find_tile(&pts, Tile::Start).first().copied().unwrap();
        let mut map = Self {
            pts,
            start,
            path: Default::default(),
        };
        map.swap_start();
        map.walk();
        map
    }

    // start with a path tile on the top and start walking the path, recording interior spaces.
    fn area(&self) -> usize {
        println!("{self}");
        let start: &Pt = self
            .pts
            .iter()
            .flat_map(|r| r.into_iter())
            .filter(|pt| matches!(pt.tile, Tile::HPipe | Tile::BendNE | Tile::BendNW))
            .next()
            .unwrap();
        let idx: usize = self
            .path
            .iter()
            .enumerate()
            .find(|(i, pt)| pt == &start)
            .map(|(i, pt)| i)
            .unwrap();
        let mut pts: Vec<&Pt> = self.path[idx..]
            .iter()
            .chain(self.path[0..idx].iter())
            .collect::<Vec<_>>();
        if !pts[1].tile.has(Dir::Left) && !pts[1].tile.has(Dir::Up) {
            println!("Reversing iterator.");
            pts = self.path[0..=idx]
                .iter()
                .rev()
                .chain(self.path[idx + 1..self.path.len()].iter().rev())
                .collect();
        }
        // from here on, we can assume that as we're moving along the points of the path that we
        // will be using a rule that the "interior" is anything on the "right" side of the path.
        // The right side of the path is the right side as determined by the vector of the last
        // tile to the current one.
        let mut iter = pts.iter().copied().copied();
        let mut last: Option<Pt> = None;
        let mut interiors: HashSet<Pt> = HashSet::default();
        use Tile::*;
        for pt in iter {
            println!("Pt: {pt:?}");
            match (pt.tile, last.as_ref()) {
                (HPipe, None) => {
                    let tiles = self.ground_tiles(pt, Dir::Down);
                    interiors.extend(tiles);
                }
                (HPipe, Some(last)) => {
                    let interior_dir = HPipe.interior_dir_for_last(&last.tile);
                    let tiles = self.ground_tiles(pt, interior_dir);
                    interiors.extend(tiles);
                }
                (VPipe, Some(last)) => {
                    let interior_dir = VPipe.interior_dir_for_last(&last.tile);
                    let tiles = self.ground_tiles(pt, interior_dir);
                    interiors.extend(tiles);
                }
                (BendSW | BendSE | BendNE | BendNW, last) => {}
                (VPipe, None) => panic!("invalid vpipe with no last"),
                (tile, last) => panic!("invalid tile: {tile:?} last: {last:?}"),
            }
            last.replace(pt);
        }
        interiors.len()
    }

    fn ground_tiles(&self, pt: Pt, dir: Dir) -> Vec<Pt> {
        self.neighbor(pt, dir);
        vec![]
    }

    // find the complete path and record it
    fn walk(&mut self) {
        let mut lu: HashSet<Pt> = HashSet::default();
        let mut path: Vec<Pt> = vec![];
        let start = self.get(self.start.x, self.start.y).expect("no start");
        let mut cur = start.clone();
        let mut last_dir: Option<Dir> = None;
        loop {
            path.push(cur);
            lu.insert(cur);
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
            last_dir.replace(dir);
            cur = self.neighbor(cur, dir).unwrap();
            if cur == start {
                break;
            }
        }
        for row in self.pts.iter_mut() {
            for pt in row.iter_mut() {
                if !lu.contains(&pt) {
                    pt.tile = Tile::Ground;
                }
            }
        }
        self.path = path;
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
            (_, Some(d), _, Some(r)) if d.has(Up) && r.has(Left) => Tile::BendNW,
            (_, Some(d), Some(l), _) if d.has(Up) && l.has(Right) => Tile::BendNE,
            (Some(u), _, _, Some(r)) if u.has(Down) && r.has(Left) => Tile::BendSW,
            (Some(u), _, Some(l), _) if u.has(Down) && l.has(Right) => Tile::BendSE,
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
        if let Some(row) = self.pts.get_mut(y) {
            if let Some(v) = row.get_mut(x) {
                v.tile = tile;
            }
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<Pt> {
        self.pts.get(y).map(|row| row.get(x)).flatten().copied()
    }

    fn find_tile(pts: &[Vec<Pt>], tile: Tile) -> Vec<Pt> {
        pts.iter()
            .enumerate()
            .map(|(row, pts)| pts.iter().filter(|pt| pt.tile == tile).collect::<Vec<_>>())
            .flatten()
            .copied()
            .collect::<Vec<_>>()
    }
}

#[test]
fn test_iterator_rev() {
    let ns: &[i32; 3] = &[1, 2, 3];
    let xs: Vec<_> = ns
        .iter()
        .rev()
        .cycle()
        .skip(ns.len())
        .take(ns.len())
        .collect();
    let es: Vec<&i32> = vec![];
    assert_eq!(xs, es);
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
        &Pt::new(0, 4, Tile::BendSW),
        &Pt::new(1, 4, Tile::HPipe)
    ))
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .pts
            .iter()
            .map(|pts| pts.iter().map(|pt| format!("{}", pt.tile)).join(""))
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
    BendSW,
    BendSE,
    BendNE,
    BendNW,
    Ground,
    Start,
    Interior,
}

impl Tile {
    fn dirs(&self) -> Option<[Dir; 2]> {
        match self {
            Tile::VPipe => Some([Dir::Up, Dir::Down]),
            Tile::HPipe => Some([Dir::Left, Dir::Right]),
            Tile::BendSW => Some([Dir::Up, Dir::Right]),
            Tile::BendSE => Some([Dir::Up, Dir::Left]),
            Tile::BendNW => Some([Dir::Down, Dir::Right]),
            Tile::BendNE => Some([Dir::Down, Dir::Left]),
            _ => None,
        }
    }
    fn has(&self, dir: Dir) -> bool {
        use Tile::*;
        match (self, dir) {
            (VPipe | BendSW | BendSE, Dir::Up) => true,
            (VPipe | BendNW | BendNE, Dir::Down) => true,
            (HPipe | BendSE | BendNE, Dir::Left) => true,
            (HPipe | BendSW | BendNW, Dir::Right) => true,
            _ => false,
        }
    }
    // which direction is the interior given the last?
    fn interior_dir_for_last(&self, last: &Tile) -> Dir {
        match self {
            Tile::VPipe if last.has(Dir::Up) => Dir::Right,
            Tile::VPipe => Dir::Left,
            Tile::HPipe if last.has(Dir::Right) => Dir::Down,
            Tile::HPipe => Dir::Up,
            _ => panic!("invalid interior dir for last self:{self:?}"),
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Tile::VPipe => "║",
            Tile::HPipe => "═",
            Tile::BendSW => "╚",
            Tile::BendSE => "╝",
            Tile::BendNE => "╗",
            Tile::BendNW => "╔",
            Tile::Ground => ".",
            Tile::Start => "S",
            Tile::Interior => "I",
        };
        write!(f, "{s}")
    }
}

impl From<char> for Tile {
    fn from(ch: char) -> Self {
        match ch {
            '.' => Tile::Ground,
            '|' => Tile::VPipe,
            '-' => Tile::HPipe,
            'L' => Tile::BendSW,
            'J' => Tile::BendSE,
            '7' => Tile::BendNE,
            'F' => Tile::BendNW,
            'S' => Tile::Start,
            _ => panic!("unknown tile: {ch}"),
        }
    }
}

fn parse(input: &str) -> Map {
    Map::new(
        input
            .lines()
            .into_iter()
            .enumerate()
            .map(|(y, row)| {
                row.chars()
                    .enumerate()
                    .map(|(x, ch)| Pt::new(x, y, Tile::from(ch)))
                    .collect()
            })
            .collect(),
    )
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
