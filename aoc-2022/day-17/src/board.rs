use std::{
    fmt::Display,
    ops::{AddAssign, Deref, DerefMut},
};

use crate::prelude::*;

impl Board {
    pub fn new() -> Self {
        let width = 7;
        let entities = vec![];
        let falling = None;
        Self {
            width,
            entities,
            falling,
        }
    }

    pub fn run(&mut self, mut shapes: Shapes, mut gusts: Gusts) {
        let shape = shapes.next().unwrap();
        let entity = self.shape_to_entity(shape); // figure out where to put the entity
        self.drop(entity);
        println!("{self}");
    }

    fn drop(&mut self, entity: Entity) {
        self.entities.push(entity);
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
        if let Some(entity) = &self.falling {
            entity.points.0.iter().for_each(|p| {
                points.push(p.clone());
            })
        }
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
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.render())
    }
}

// the floor is at level y = 0. positions above
// the board are at y > 0.
#[derive(Debug)]
pub struct Board {
    width: i32, // always 7
    entities: Vec<Entity>,
    falling: Option<Entity>,
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

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Points(Vec<Point>);

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
