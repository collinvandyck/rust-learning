use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    vec,
};

/// (x, y)
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Point(i32, i32);

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self(x, y)
    }
    fn default() -> Self {
        Point(0, 0)
    }
}

#[derive(Debug)]
pub struct Formation(Vec<Point>);

// points are in X,Y form
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sand {
    Waiting,        // time to start a new drip
    Falling(Point), // we are falling at this point
    Abyss,          // we've fallen off.
    Done,           // nothing else to be done
}

#[derive(Debug)]
pub struct Cave {
    tiles: HashMap<Point, Tile>,
    min: Point,
    max: Point,
    source: Point,
    sand: Sand,
    pub grains: i32,
    in_the_abyss: bool,
    use_floor: bool,
}

impl Cave {
    pub fn tick(&mut self) -> Sand {
        self.sand = self.advance();
        self.sand
    }
    pub fn use_floor(&mut self, v: bool) {
        self.use_floor = v;
    }
    fn advance(&mut self) -> Sand {
        let mut sand = self.sand;
        loop {
            sand = match sand {
                Sand::Falling(point) => {
                    // todo: figure out if we can short circuit from the result of self.gravity
                    match self.gravity(point) {
                        falling @ Sand::Falling(_) => return falling,
                        d => d,
                    }
                }
                Sand::Waiting => {
                    if let Some(tile) = self.get(self.source) {
                        if tile.entity != Entity::Nothing {
                            return Sand::Done;
                        }
                    }
                    self.set(self.source, Entity::Sand);
                    self.grains += 1;
                    Sand::Falling(self.source)
                }
                Sand::Done => return Sand::Done,
                Sand::Abyss => {
                    self.grains -= 1; // don't count that grain
                    self.in_the_abyss = true;
                    Sand::Done
                }
            }
        }
    }
    fn gravity(&mut self, prev: Point) -> Sand {
        let down = Point(prev.0, prev.1 + 1);
        let down_left = Point(prev.0 - 1, prev.1 + 1);
        let down_right = Point(prev.0 + 1, prev.1 + 1);
        for to in [down, down_left, down_right] {
            if let Some(s) = self.try_move(prev, to) {
                return s;
            }
        }
        Sand::Waiting
    }
    // returns Some if the sand could be moved
    fn try_move(&mut self, prev: Point, to: Point) -> Option<Sand> {
        match self.get(to) {
            Some(tile) => match tile.entity {
                Entity::Nothing => {
                    self.remove(prev);
                    self.set(to, Entity::Sand);
                    Some(Sand::Falling(to))
                }
                Entity::Rock | Entity::Sand => None,
                Entity::Source => None,
            },
            None => Some(Sand::Abyss),
        }
    }
    pub fn new(formations: &[Formation]) -> Cave {
        let mut min = Point::new(i32::MAX, 0);
        let mut max = Point::new(i32::MIN, i32::MIN);
        formations.iter().flat_map(|f| f.hydrate()).for_each(|p| {
            min.0 = i32::min(min.0, p.0);
            max.0 = i32::max(max.0, p.0);
            max.1 = i32::max(max.1, p.1);
        });
        let source = Point::new(500, 0);
        let sand = Sand::Waiting;
        let grains = 0;
        let in_the_abyss = false;
        let tiles = HashMap::new();
        let use_floor = false;
        let mut res = Cave {
            tiles,
            min,
            max,
            source,
            sand,
            grains,
            in_the_abyss,
            use_floor,
        };
        formations
            .iter()
            .flat_map(|f| f.hydrate())
            .for_each(|p| res.set(p, Entity::Rock));
        res
    }
    fn abyss_y(&self) -> i32 {
        if self.use_floor {
            self.max.1 + 3
        } else {
            self.max.1 + 1
        }
    }
    fn get(&self, point: Point) -> Option<Tile> {
        match self.tiles.get(&point) {
            Some(tile) => return Some(*tile),
            None => {
                if point.1 >= self.abyss_y() {
                    // we have fallen off the map
                    None
                } else if self.use_floor && point.1 == self.max.1 + 2 {
                    Some(Tile {
                        point,
                        entity: Entity::Rock,
                    })
                } else {
                    Some(Tile {
                        point,
                        entity: Entity::Nothing,
                    })
                }
            }
        }
    }
    fn remove(&mut self, point: Point) {
        self.tiles.remove(&point);
    }
    fn set(&mut self, point: Point, entity: Entity) {
        self.tiles.insert(point, Tile { point, entity });
        // update the min/max x bounds
        self.min.0 = self.min.0.min(point.0);
        self.max.0 = self.max.0.max(point.0);
    }
    fn rows(&self) -> usize {
        (self.max.1 - self.min.1 + 1).try_into().unwrap()
    }
    fn cols(&self) -> usize {
        (self.max.0 - self.min.0 + 1).try_into().unwrap()
    }
    fn render(&self) -> String {
        let mut buf = String::new();

        let r1 = format!("{}", self.min.0).chars().collect::<Vec<_>>();
        let r2 = format!("{}", self.source.0).chars().collect::<Vec<_>>();
        let r3 = format!("{}", self.max.0).chars().collect::<Vec<_>>();

        for idx in 0..3 {
            let pad = "\t";
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
        let mut y_max = self.max.1;
        if self.use_floor {
            y_max += 2;
        }
        for y in self.min.1..=y_max {
            let pad = "\t";
            let mut res = format!("{}{pad}", y);
            for x in self.min.0..=self.max.0 {
                let point = Point(x, y);
                let tile = self.get(point).unwrap();
                if tile.entity == Entity::Nothing && tile.point == self.source {
                    res.push('+');
                } else {
                    res.push(tile.entity.char());
                }
            }
            buf.push_str(&res);
            buf.push('\n');
        }
        buf
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rendered = self.render();
        write!(f, "{rendered}")
    }
}
