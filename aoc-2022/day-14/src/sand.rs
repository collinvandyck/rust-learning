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
    pub fn parse(line: &str) -> Self {
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
    fn hydrate(&self) -> Vec<Point> {
        self.0
            .windows(2)
            .enumerate()
            .flat_map(|(idx, pts)| {
                let (Point(x1, y1), Point(x2, y2)) = (pts[0], pts[1]);
                assert!(x1 == x2 || y1 == y2);
                let mut v = vec![];
                let inclusive = idx < self.0.len() - 1;
                if x1 != x2 {
                    for x in i32::min(x1, x2)..=i32::max(x1, x2) {
                        if x != i32::max(x1, x2) || inclusive {
                            v.push(Point::new(x, y1));
                        }
                    }
                } else {
                    for y in i32::min(y1, y2)..=i32::max(y1, y2) {
                        if y != i32::max(y1, y2) || inclusive {
                            v.push(Point::new(x1, y));
                        }
                    }
                }
                v
            })
            .collect()
    }
}

#[derive(Debug, Clone, Copy)]
struct Tile {
    point: Point,
    entity: Entity,
}

#[derive(Debug, Clone, Copy)]
enum Entity {
    Nothing,
    Source,
    Rock,
    Sand,
}

impl Entity {
    fn char(self) -> char {
        match self {
            Entity::Nothing => '.',
            Entity::Source => '+',
            Entity::Rock => '#',
            Entity::Sand => 'o',
        }
    }
}

impl Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch = self.char();
        write!(f, "{ch}")
    }
}

#[derive(Debug, Clone, Copy)]
enum Sand {
    Waiting,        // time to start a new drip
    Falling(Point), // we are falling at this point
    AtRest(Point),  // where we landed
    Abyss,          // we've fallen off.
    Done,           // nothing else to be done
}

#[derive(Debug)]
pub struct Cave {
    tiles: Vec<Vec<Tile>>,
    min: Point,
    max: Point,
    source: Point,
    sand: Sand,
    grains: i32,
}

impl Cave {
    pub fn tick(&mut self) {
        self.sand = self.advance();
    }
    fn advance(&mut self) -> Sand {
        let mut sand = self.sand;
        loop {
            sand = match sand {
                Sand::Waiting => {
                    //
                    self.grains += 1;
                    Sand::Falling(self.source)
                }
                Sand::Falling(point) => return self.gravity(point),
                Sand::Done => return Sand::Done,
                Sand::AtRest(point) => Sand::Waiting,
                Sand::Abyss => Sand::Waiting,
            }
        }
    }
    fn gravity(&mut self, point: Point) -> Sand {
        let down = Point(point.0, point.1 + 1);
        match self.get(down) {
            Some(Tile {
                entity: Entity::Nothing,
                ..
            }) => {
                if point != self.source {
                    self.set(point, Entity::Nothing);
                }
                self.set(down, Entity::Sand);
                Sand::Falling(down)
            }
            Some(Tile {
                entity: Entity::Sand | Entity::Rock,
                ..
            }) => Sand::Waiting,
            Some(Tile {
                entity: Entity::Source,
                ..
            }) => panic!("should not fall onto the source"),
            None => return Sand::Abyss, //we have fallen off
        }
    }
    pub fn new(formations: &[Formation]) -> Cave {
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
        let source = Point::new(500, 0);
        let sand = Sand::Waiting;
        let grains = 0;
        let mut res = Cave {
            tiles,
            min,
            max,
            source,
            sand,
            grains,
        };
        res.set(res.source, Entity::Source);
        formations
            .iter()
            .flat_map(|f| f.hydrate())
            .for_each(|f| res.set(f, Entity::Rock));
        res
    }
    fn get(&self, point: Point) -> Option<Tile> {
        let (row, col) = self.to_world(point);
        self.tiles.get(row).and_then(|r| r.get(col)).map(|t| *t)
    }
    fn set(&mut self, point: Point, e: Entity) {
        let (row, col) = self.to_world(point);
        self.tiles
            .get_mut(row)
            .and_then(|r| r.get_mut(col))
            .iter_mut()
            .for_each(|r| r.entity = e);
    }
    // converts from "camera" to world coords
    #[allow(clippy::cast_sign_loss)]
    fn to_world(&self, point: Point) -> (usize, usize) {
        let row = point.1 - self.min.1;
        let col = point.0 - self.min.0;
        (row as usize, col as usize)
    }
    fn rows(&self) -> usize {
        self.tiles.len()
    }
    fn cols(&self) -> usize {
        self.tiles.first().map_or(0, Vec::len)
    }
    fn render(&self) -> String {
        let mut buf = String::new();
        let row_pd = self.tiles.len() / 10;

        let r1 = format!("{}", self.min.0).chars().collect::<Vec<_>>();
        let r2 = format!("{}", self.source.0).chars().collect::<Vec<_>>();
        let r3 = format!("{}", self.max.0).chars().collect::<Vec<_>>();

        for idx in 0..3 {
            let pad = " ".repeat(row_pd + 1);
            buf.push_str(&pad);
            buf.push(*r1.get(idx).unwrap());
            #[allow(clippy::cast_sign_loss)]
            let pad = " ".repeat((500 - self.min.0 - 1) as usize);
            buf.push_str(&pad);
            buf.push(*r2.get(idx).unwrap());
            #[allow(clippy::cast_sign_loss)]
            let pad = " ".repeat((self.max.0 - 500 - 1) as usize);
            buf.push_str(&pad);
            buf.push(*r3.get(idx).unwrap());
            buf.push('\n');
        }

        // draw the grid of the map with numbered rows.
        let board = self
            .tiles
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
            .join("\n");
        buf.push_str(&board);
        buf
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rendered = self.render();
        write!(f, "{rendered}")
    }
}
