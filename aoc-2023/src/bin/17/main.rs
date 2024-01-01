#![allow(dead_code, unused)]

use std::collections::{hash_map::Entry, HashMap, HashSet, VecDeque};

use itertools::Itertools;
use strum::IntoEnumIterator;

fn main() {
    let ex1 = include_str!("ex1.txt");
    println!("p1ex1 = {}", min_loss(ex1));
}

fn min_loss(input: &str) -> usize {
    let map = Map::parse(input);
    let src = Point::new(0, 0);
    let dst = Point::new(map.cols - 1, map.rows - 1);
    let mut ml = MinLoss::new(&map, src, dst);
    ml.solve()
}

fn minimize_loss(input: &str) -> usize {
    let map = Map::parse(input);
    let src = Point::new(0, 0);
    let dst = Point::new(map.cols - 1, map.rows - 1);
    let mut path = Path::new(&map, src, dst);
    path.dijkstra()
}

struct MinLoss<'a> {
    map: &'a Map,
    src: Point,
    dst: Point,
    rays: Vec<Ray<'a>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Move {
    from: Point,
    dir: Dir,
    tile: Tile, // the dst tile
}

impl Move {
    fn new(from: Point, dir: Dir, tile: Tile) -> Self {
        Self { from, tile, dir }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Ray<'a> {
    map: &'a Map,
    cur: Point,
    dst: Point,
    dir: Option<Dir>,
    hst: Vec<Move>,
}

impl<'a> Ray<'a> {
    fn new(map: &'a Map, cur: Point, dst: Point) -> Self {
        let dir = None;
        let hst = vec![];
        Self {
            map,
            cur,
            dst,
            dir,
            hst,
        }
    }

    fn step(&mut self, mov: Move) {}

    fn next_moves(&self) -> Vec<Move> {
        Dir::iter()
            .filter(|dir| match self.dir {
                Some(sd) => dir != &sd.opposite(),
                None => true,
            })
            .filter(|dir| {
                self.hst
                    .iter()
                    .rev()
                    .take(2)
                    .filter(|mv| &mv.dir == dir)
                    .count()
                    < 2
            })
            .flat_map(|dir| self.cur.next(dir).map(|pt| (pt, dir)))
            .flat_map(|(next_pt, dir)| {
                self.map
                    .get(next_pt)
                    .map(|next_tile| Move::new(self.cur, dir, *next_tile))
            })
            .collect()
    }
}

impl<'a> MinLoss<'a> {
    fn new(map: &'a Map, src: Point, dst: Point) -> Self {
        let rays = vec![Ray::new(map, src, dst)];
        Self {
            map,
            src,
            dst,
            rays,
        }
    }
    fn solve(&mut self) -> usize {
        for idx in 0..self.len() {
            let ray = self.rays.get_mut(idx).unwrap();
            let moves = dbg!(ray.next_moves());
        }
        todo!()
    }

    fn len(&self) -> usize {
        self.rays.len()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Path<'a> {
    map: &'a Map,
    src: Point,
    dst: Point,
}

impl<'a> Path<'a> {
    fn new(map: &'a Map, src: Point, dst: Point) -> Self {
        Self { map, src, dst }
    }

    // this is not going to work because of the need to modify the graph based on the recent
    // traversal due to the restrictions on direction as well as the restrictions on runs in the
    // same direction >= 3.
    //
    // a better idea might be to have separate depth first searches in progress where each search
    // can be canceled based on the cost associated with a subpath in a separate tree which might
    // be shorter.
    fn dijkstra(&mut self) -> usize {
        let mut queue: HashSet<Point> = HashSet::default();
        let mut dist: HashMap<Point, usize> = HashMap::default();
        let mut prev: HashMap<Point, Point> = HashMap::default();
        self.map.points().for_each(|pt| {
            dist.insert(pt, usize::MAX);
            queue.insert(pt);
        });
        dist.insert(self.src, 0);
        loop {
            if dist.is_empty() {
                panic!("no path found to dst");
            }
            // find the min dist and copy the value
            let (point, cost) = dist
                .iter()
                .filter(|pt| queue.contains(pt.0))
                .min_by_key(|(pt, cost)| *cost)
                .unwrap();
            if point == &self.dst {
                break;
            }
            let point = *point;

            // remove the point from the queue
            queue.remove(&point);

            // update dist and prev maps based on min cost
            for neighbor in self.unvisited(&point, &queue) {
                let pdist: &usize = dist.get(&point).expect("no pdist");
                let edge = self.map.get(neighbor).map(|t| t.cost).expect("no node");
                let alt = pdist + edge;
                if &alt < dist.get(&neighbor).expect("no neighbor dist") {
                    dist.insert(neighbor, alt);
                    prev.insert(neighbor, point);
                }
            }
        }
        let mut path: VecDeque<&Point> = VecDeque::default();
        let mut cur = &self.dst;
        path.push_front(cur);
        while let Some(prev) = prev.get(cur) {
            path.push_front(prev);
            cur = prev;
        }
        path.into_iter()
            .flat_map(|pt| self.map.get(*pt))
            .map(|t| t.cost)
            .sum()
    }

    // TOOD: we need to put the brakes on successive runs
    fn unvisited(
        &self,
        pt: &'a Point,
        unvisited: &'a HashSet<Point>,
    ) -> impl Iterator<Item = Point> + 'a {
        Dir::iter()
            .flat_map(|dir| pt.next(dir))
            .filter(|pt| unvisited.contains(pt))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Map {
    tiles: Vec<Tile>,
    rows: usize,
    cols: usize,
}

impl Map {
    fn parse(input: &str) -> Self {
        let tiles = input
            .trim()
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.trim()
                    .chars()
                    .map(|ch| ch.to_string().parse::<usize>().unwrap())
                    .enumerate()
                    .map(move |(x, cost)| Tile::new(Point::new(x, y), cost))
                    .collect_vec()
            })
            .collect_vec();
        let rows = tiles.len();
        let cols = tiles.first().map(|f| f.len()).expect("no rows");
        let tiles = tiles.into_iter().flatten().collect();
        Self { tiles, rows, cols }
    }
    fn points(&self) -> impl Iterator<Item = Point> + '_ {
        self.tiles.iter().map(|t| t.pt)
    }
    fn idx(&self, pt: Point) -> usize {
        pt.y * self.rows + pt.x
    }
    fn get(&self, pt: Point) -> Option<&Tile> {
        let idx = self.idx(pt);
        self.tiles.get(idx)
    }
    fn get_mut(&mut self, pt: Point) -> Option<&mut Tile> {
        let idx = self.idx(pt);
        self.tiles.get_mut(idx)
    }
    fn set_ch(&mut self, pt: Point, ch: char) {
        if let Some(tile) = self.get_mut(pt) {
            tile.ch = Some(ch);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Tile {
    pt: Point,
    cost: usize,
    ch: Option<char>,
}

impl Tile {
    fn new(pt: Point, cost: usize) -> Self {
        let ch = None;
        Self { pt, cost, ch }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct PointDir {
    pt: Point,
    dir: Dir,
}

impl PointDir {
    fn new(pt: Point, dir: Dir) -> Self {
        Self { pt, dir }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    fn next(&self, dir: Dir) -> Option<Self> {
        match dir {
            Dir::Up => self.y.checked_sub(1).map(|y| (self.x, y)),
            Dir::Down => self.y.checked_add(1).map(|y| (self.x, y)),
            Dir::Left => self.x.checked_sub(1).map(|x| (x, self.y)),
            Dir::Right => self.x.checked_add(1).map(|x| (x, self.y)),
        }
        .map(|(x, y)| Point::new(x, y))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum_macros::EnumIs, strum_macros::EnumIter)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let ex1 = include_str!("ex1.txt");
        let map = Map::parse(ex1);
        assert_eq!(map.rows, 13);
        assert_eq!(map.cols, 13);
        assert_eq!(map.tiles.len(), 13 * 13);
    }
}
