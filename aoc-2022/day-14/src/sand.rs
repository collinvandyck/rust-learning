use std::{
    fmt::{Debug, Display},
    vec,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Point(i32, i32);

impl Point {
    fn new(row: i32, col: i32) -> Self {
        Self(row, col)
    }
    fn default() -> Self {
        Point(0, 0)
    }
}

#[derive(Debug)]
pub struct Formation(Vec<Point>);

impl Formation {
    pub fn parse(line: String) -> Self {
        let points = line
            .split(" -> ")
            .map(|s| {
                let mut nums = s.split(',').map(|s| s.parse::<i32>().unwrap());
                let x = nums.next().unwrap();
                let y = nums.next().unwrap();
                Point(x, y)
            })
            .collect::<Vec<_>>();
        Self(points)
    }
}

#[derive(Debug)]
struct Tile {
    point: Point,
    entity: Entity,
}

#[derive(Debug, Clone, Copy)]
enum Entity {
    Nothing,
    Source,
    Rock,
}

impl Entity {
    fn char(&self) -> char {
        use Entity::*;
        match self {
            Nothing => '.',
            Source => '+',
            Rock => '#',
        }
    }
}

impl Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch = self.char();
        write!(f, "{ch}")
    }
}

#[derive(Debug)]
pub struct Cave {
    tiles: Vec<Vec<Tile>>,
    min: Point,
    max: Point,
}

impl Cave {
    pub fn new(formations: Vec<Formation>) -> Cave {
        let mut min = Point::new(i32::MAX, 0);
        let mut max = Point::new(i32::MIN, i32::MIN);
        formations.iter().flat_map(|f| &f.0).for_each(|point| {
            min.0 = i32::min(min.0, point.0);
            max.0 = i32::max(max.0, point.0);
            max.1 = i32::max(max.1, point.1);
        });
        let mut tiles = vec![];
        for row_idx in min.1..=max.1 {
            let mut row = vec![];
            for col_idx in min.0..=max.0 {
                let tile = Tile {
                    point: Point(row_idx, col_idx),
                    entity: Entity::Nothing,
                };
                row.push(tile);
            }
            tiles.push(row);
        }
        let mut res = Cave { min, max, tiles };
        res.set(Point::new(500, 0), Entity::Source);
        res
    }
    fn set(&mut self, point: Point, e: Entity) {
        let (row, col) = self.to_world(&point);
        self.tiles
            .get_mut(row)
            .and_then(|r| r.get_mut(col))
            .iter_mut()
            .for_each(|r| r.entity = e);
    }
    fn to_world(&self, point: &Point) -> (usize, usize) {
        let row = point.1 - self.min.1;
        let col = point.0 - self.min.0;
        (row as usize, col as usize)
    }
    fn rows(&self) -> usize {
        self.tiles.len()
    }
    fn cols(&self) -> usize {
        self.tiles.first().map_or(0, |r| r.len())
    }
    fn render(&self) -> String {
        let row_pd = self.tiles.len() / 10;
        self.tiles
            .iter()
            .enumerate()
            .map(|(ri, row)| {
                let row_pd = row_pd - (ri / 10);
                let pad = " ".repeat(row_pd);
                let mut res = format!("{ri}{pad}");
                let row = row.iter().map(|t| t.entity.char()).collect::<String>();
                res.push_str(row.as_str());
                res
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rendered = self.render();
        write!(f, "{rendered}")
    }
}
