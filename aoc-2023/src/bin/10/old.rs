#![allow(unused, dead_code)]

use itertools::Itertools;
use std::{collections::HashSet, fmt::Debug, hash::Hash};

fn main() {
    let ex1 = include_str!("ex1.txt");
    let ex2 = include_str!("ex2.txt");
    let in1 = include_str!("in1.txt");
    println!("p1ex1={}", farthest_distance(ex1));
    println!("p1i1n={}", farthest_distance(in1));
    println!("p2e1x={}", area_enclosed(ex1));
    // println!("p2e2x={}", area_enclosed(ex2));
    //println!("p2in={}", area_enclosed(in1));
}

fn farthest_distance(input: &str) -> usize {
    let map = parse(input);
    map.path.len() / 2
}

fn area_enclosed(input: &str) -> usize {
    let mut map = parse(input);
    map.area()
}

struct Map {
    start: Pt,
    pts: Vec<Vec<Pt>>,
    path: Vec<Pt>,
    marks: HashSet<Pt>,
}

impl Map {
    fn new(pts: Vec<Vec<Pt>>) -> Self {
        let start = Self::find_tile(&pts, Tile::Start).first().copied().unwrap();
        let mut map = Self {
            pts,
            start,
            path: Default::default(),
            marks: HashSet::default(),
        };
        map.swap_start();
        map.walk();
        map
    }

    fn mark(&mut self, pt: Pt) {
        self.marks.insert(pt);
    }

    // start with a path tile on the top and start walking the path, recording interior spaces.
    fn area(&mut self) -> usize {
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
        let mut pts: Vec<Pt> = self.path[idx..]
            .iter()
            .chain(self.path[0..idx].iter())
            .copied()
            .collect::<Vec<_>>();
        let rev = pts[1].x <= pts[0].x;
        if rev {
            println!("Reversing iterator.");
            pts = self.path[0..=idx]
                .iter()
                .rev()
                .chain(self.path[idx + 1..self.path.len()].iter().rev())
                .copied()
                .collect();
        }
        // from here on, we can assume that as we're moving along the points of the path that we
        // will be using a rule that the "interior" is anything on the "right" side of the path.
        // The right side of the path is the right side as determined by the vector of the last
        // tile to the current one.
        let mut iter = pts.iter_mut();
        let mut last: Option<Pt> = None;
        let mut interiors: HashSet<Pt> = HashSet::default();
        for (idx, pt) in iter.enumerate() {
            println!("{idx}: {pt:?}");
            match (pt.tile, last.as_ref()) {
                (Tile::HPipe, None) => {
                    let tiles = self.ground_tiles(*pt, Dir::Down);
                    if !tiles.is_empty() {
                        self.mark(*pt);
                        println!("  hpipe (none) ground tiles: {tiles:?}");
                    }
                    interiors.extend(tiles);
                }
                (Tile::HPipe, Some(last)) => {
                    let interior_dir = Tile::HPipe.interior_dir_for_last(&last.tile);
                    let tiles = self.ground_tiles(*pt, interior_dir);
                    if !tiles.is_empty() {
                        self.mark(*pt);
                        println!("{idx}: {pt:?}");
                        println!(
                            "  interior_dir={interior_dir:?} for last_tile: {:?}",
                            last.tile
                        );
                        println!("  hpipe ground tiles: {tiles:?}");
                    }
                    interiors.extend(tiles);
                }
                (Tile::VPipe, Some(last)) => {
                    let interior_dir = Tile::VPipe.interior_dir_for_last(&last.tile);
                    let tiles = self.ground_tiles(*pt, interior_dir);
                    if !tiles.is_empty() {
                        self.mark(*pt);
                        println!("{idx}: {pt:?}");
                        println!("  vpipe ground tiles: {tiles:?}");
                    }
                    interiors.extend(tiles);
                }
                (Tile::BendSW | Tile::BendSE | Tile::BendNE | Tile::BendNW, last) => {}
                (Tile::VPipe, None) => panic!("invalid vpipe with no last"),
                (tile, last) => panic!("invalid tile: {tile:?} last: {last:?}"),
            }
            if let Some(last) = last {
                let dir = pt.dir_to(&last);
                self.set(pt.x, pt.y, Tile::Dir(dir));
            }
            last.replace(*pt);
        }
        interiors.iter().for_each(|pt| {
            self.set(pt.x, pt.y, Tile::Interior);
        });
        println!("{self}");
        interiors.len()
    }

    fn ground_tiles(&self, mut pt: Pt, dir: Dir) -> Vec<Pt> {
        let mut res = vec![];
        while let Some(dst) = self.neighbor(pt, dir) {
            if matches!(dst.tile, Tile::Ground) {
                res.push(dst);
                pt = dst;
            } else {
                break;
            }
        }
        res
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
fn test_ground_tiles() {
    let input = include_str!("ex1.txt");
    let map = parse(input);
    println!("{map}");
    assert!(false, "boom");
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
            .map(|pts| {
                pts.iter()
                    .map(|pt| {
                        if self.marks.contains(&pt) {
                            format!("*")
                        } else if (self.start.x, self.start.y) == (pt.x, pt.y) {
                            format!("S")
                        } else {
                            format!("{}", pt.tile)
                        }
                    })
                    .join("")
            })
            .join("\n");
        write!(f, "{s}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    fn dir_to(&self, other: &Pt) -> Dir {
        if other.x > self.x {
            Dir::Right
        } else if other.x < self.x {
            Dir::Left
        } else if other.y > self.y {
            Dir::Down
        } else {
            Dir::Up
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
    Dir(Dir),
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
        match (self, dir) {
            (Tile::VPipe | Tile::BendSW | Tile::BendSE, Dir::Up) => true,
            (Tile::VPipe | Tile::BendNW | Tile::BendNE, Dir::Down) => true,
            (Tile::HPipe | Tile::BendSE | Tile::BendNE, Dir::Left) => true,
            (Tile::HPipe | Tile::BendSW | Tile::BendNW, Dir::Right) => true,
            _ => false,
        }
    }
    // which direction is the interior given the last?
    fn interior_dir_for_last(&self, last: &Tile) -> Dir {
        match self {
            Tile::VPipe if last.has(Dir::Down) => Dir::Left,
            Tile::VPipe => Dir::Right,
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
            Tile::Dir(Dir::Up) => "^",
            Tile::Dir(Dir::Down) => "v",
            Tile::Dir(Dir::Left) => "<",
            Tile::Dir(Dir::Right) => ">",
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
