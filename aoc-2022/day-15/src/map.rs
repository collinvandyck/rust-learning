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
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "map")
    }
}
