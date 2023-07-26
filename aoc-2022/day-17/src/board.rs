use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    ops::{AddAssign, Deref, DerefMut},
};

use crate::prelude::*;

// the floor is at level y = 0. positions above
// the board are at y > 0.
pub struct Board {
    width: i32, // always 7
    rocks: HashSet<Point>,
    entities: Vec<Entity>,
    shapes: Shapes, // iterator
    gusts: Gusts,   // iterator
}

impl Board {
    pub fn new(shapes: Shapes, gusts: Gusts) -> Self {
        let width = 7;
        let entities = vec![];
        Self {
            width,
            entities,
            shapes,
            gusts,
        }
    }

    /// the main run loop.
    pub fn run(&mut self, num_rocks: usize) {
        for count in 0..=num_rocks {
            if count % 1000 == 0 {
                println!("{count}");
            }
            let shape = self.shapes.next().unwrap();
            let entity = self.shape_to_entity(shape); // figure out where to put the entity
            self.drop(entity);
        }
        if num_rocks < 5000 {
            println!("{self}");
        }
        println!("Height: {}", self.height());
    }

    /// alternates between gusting and dropping until the entity comes to rest,
    /// at which point the method will exit.
    fn drop(&mut self, mut entity: Entity) {
        // adjust for gust
        loop {
            if let Some(gust) = self.gusts.next() {
                let adjustment = match gust {
                    Gust::Left => Point(-1, 0),
                    Gust::Right => Point(1, 0),
                };
                let candidate_points = entity.points.add(adjustment);
                if self.is_space_available(&candidate_points) {
                    entity.points = candidate_points;
                }
            }
            // adjust for drop
            let adjustment = Point(0, -1);
            let candidate_points = entity.points.add(adjustment);
            if self.is_space_available(&candidate_points) {
                entity.points = candidate_points;
            } else {
                break;
            }
        }
        self.entities.push(entity);
    }

    fn is_space_available(&self, points: &Points) -> bool {
        // first check to make sure the points are in bounds
        if !points.in_bounds(self.width) {
            return false;
        }
        // next check to make sure that there is space on the board. if there
        // are any points that overlap, we must return false.
        self.entities.iter().flat_map(|e| e.points.iter()).all(|p| {
            // p cannot be in any of the points
            !points.iter().any(|can| p.0 == can.0 && p.1 == can.1)
        })
    }

    fn shape_to_entity(&mut self, shape: Shape) -> Entity {
        let height = shape.height();
        let highest_y = self.highest_rock_y();
        let mut points = Points(shape.starting_coords());
        points.iter_mut().for_each(|p| {
            p.0 += 3;
            p.1 = (height - p.1) + highest_y + 3;
        });
        Entity { shape, points }
    }

    fn render(&self) -> String {
        let mut points = self.sorted_points().into_iter().peekable();
        let mut lines = vec![];
        for y in (1..=self.highest_rock_y()).rev() {
            let mut line = String::new();
            line.push('|');
            for x in 1..=self.width {
                if points.peek().filter(|p| p.0 == x && p.1 == y).is_some() {
                    line.push('#');
                    points.next();
                } else {
                    line.push('.');
                }
            }
            line.push('|');
            lines.push(line);
        }
        lines.push(format!("+{}+", "-".repeat(self.width as usize)));
        lines.join("\n")
    }

    fn sorted_points(&self) -> Vec<Point> {
        let mut points = self
            .entities
            .iter()
            .flat_map(|e| e.points.iter())
            .copied()
            .collect::<Vec<_>>();
        points.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        points
    }

    /// returns the highest rock y position. The floor is represented at y=0.
    fn highest_rock_y(&self) -> i32 {
        self.entities
            .iter()
            .flat_map(|e| e.points.0.iter())
            .map(|p| p.1)
            .max()
            .unwrap_or(0)
    }
    pub fn height(&self) -> i32 {
        self.highest_rock_y()
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.render())
    }
}

type Shapes = Box<dyn Iterator<Item = Shape>>;
type Gusts = Box<dyn Iterator<Item = Gust>>;

#[derive(Debug)]
struct Entity {
    shape: Shape,
    points: Points,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Point(pub i32, pub i32);

impl Point {
    fn in_bounds(&self, width: i32) -> bool {
        self.0 > 0 && self.0 <= width && self.1 > 0
    }
}

#[test]
fn test_point_in_bounds() {
    let width = 7_i32;
    assert!(Point(1, 1).in_bounds(width));
    assert!(Point(2, 2).in_bounds(width));
    assert!(Point(3, 7).in_bounds(width));
    assert!(Point(7, 9).in_bounds(width));
    assert!(!Point(8, 9).in_bounds(width));
    assert!(!Point(5, 0).in_bounds(width));
    assert!(!Point(1, 0).in_bounds(width));
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Points(Vec<Point>);

impl Points {
    pub fn in_bounds(&self, width: i32) -> bool {
        self.0.iter().all(|p| p.in_bounds(width))
    }
    pub fn add(&self, point: Point) -> Points {
        Points(
            self.0
                .iter()
                .map(|p| Point(p.0 + point.0, p.1 + point.1))
                .collect::<Vec<_>>(),
        )
    }
}

impl Deref for Points {
    type Target = Vec<Point>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Points {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
