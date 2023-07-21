use std::fmt::Display;

use crate::prelude::*;

pub struct Map {
    sensors: Vec<Sensor>,
    min: Point,
    max: Point,
}

impl Map {
    pub fn new(sensors: Vec<Sensor>, min: Point, max: Point) -> Self {
        Self { sensors, min, max }
    }
    fn render(&self) -> String {
        let mut lines = vec![];
        for y in self.min.1..=self.max.1 {
            let mut buf = String::new();
            for x in self.min.0..=self.max.0 {
                let point = Point(x, y);
                buf.push(self.render_point(point));
            }
            lines.push(buf);
        }
        lines.join("\n")
    }
    fn render_point(&self, point: Point) -> char {
        let reach = self.sensors.iter().any(|s| {
            let res = s.can_reach(point);
            if res {
                println!(
                    "{point} is reachable from {s} (this dist: {})",
                    s.distance_to(point)
                );
            } else {
                println!(
                    "{point} is NOT reachable from {s} (this dist: {})",
                    s.distance_to(point)
                );
            }
            res
        });
        if reach {
            '#'
        } else {
            '.'
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.render())
    }
}
