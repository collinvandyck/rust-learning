use std::fmt::Display;

use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Sensor {
    pub point: Point,
    pub beacon: Point,
}

impl Sensor {
    fn distance(&self) -> i32 {
        self.point.distance(self.beacon)
    }
}

impl Display for Sensor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Sensor(point:{}\tbeacon:{}\tdistance:{})",
            self.point,
            self.beacon,
            self.distance()
        )
    }
}
