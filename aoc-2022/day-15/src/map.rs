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
        for row in self.min.1..=self.max.1 {
            let mut buf = String::new();
            for col in self.min.0..=self.max.0 {
                buf.push('.');
            }
            lines.push(buf);
        }
        lines.join("\n")
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.render())
    }
}
