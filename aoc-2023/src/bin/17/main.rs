#![allow(dead_code, unused)]

use itertools::Itertools;

fn main() {
    let ex1 = include_str!("ex1.txt");

    println!("p1ex1 = {}", minimize_loss(ex1));
}

fn minimize_loss(input: &str) -> usize {
    let map = Map::parse(input);
    let src = Point::new(0, 0);
    let dst = Point::new(map.cols - 1, map.rows - 1);
    let mut path = Path::new(&map, src);
    path.dijkstra(dst)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Path<'a> {
    map: &'a Map,
    pt: Point,
    hist: Vec<PointDir>,
}

impl<'a> Path<'a> {
    fn new(map: &'a Map, pt: Point) -> Self {
        Self {
            map,
            pt,
            hist: vec![],
        }
    }

    fn dijkstra(&mut self, dst: Point) -> usize {
        todo!()
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Tile {
    pt: Point,
    cost: usize,
}

impl Tile {
    fn new(pt: Point, cost: usize) -> Self {
        Self { pt, cost }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum_macros::EnumIs)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
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
