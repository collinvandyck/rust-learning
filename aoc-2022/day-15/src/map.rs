use std::{collections::HashMap, fmt::Display};

use crate::prelude::*;

pub struct Map {
    sensors: Vec<Sensor>,
    lookup: HashMap<Point, Entity>,
    min: Point,
    max: Point,
}

#[derive(PartialEq, Eq)]
enum Entity {
    Sensor,
    Beacon,
}

impl Map {
    pub fn new(sensors: Vec<Sensor>, min: Point, max: Point) -> Self {
        let lookup = sensors
            .iter()
            .flat_map(|s| vec![(s.point, Entity::Sensor), (s.beacon, Entity::Beacon)])
            .collect();
        Self {
            sensors,
            lookup,
            min,
            max,
        }
    }
    // for the specified row (y), how many points are impossible for a beacon to be present?
    pub fn beacon_not_possible(&self, y: i32) -> i32 {
        let mut res = 0;
        // iterate over each column
        for x in self.min.0..=self.max.0 {
            let point = Point(x, y);
            // see if there's something literally in the way
            if let Some(e) = self.lookup.get(&point) {
                if *e == Entity::Sensor {
                    res += 1;
                }
                continue;
            }
            // see if any beacon is able to cover this spot
            let can_reach = self.sensors.iter().any(|s| s.can_reach(point));
            if can_reach {
                res += 1
            }
        }
        res
    }
    fn render(&self) -> String {
        let mut lines = vec![];
        for y in self.min.1..=self.max.1 {
            let mut buf = String::new();
            buf.push_str(format!("{y}\t").as_str());
            for x in self.min.0..=self.max.0 {
                let point = Point(x, y);
                buf.push(self.render_point(point));
            }
            lines.push(buf);
        }
        lines.join("\n")
    }
    fn render_point(&self, point: Point) -> char {
        if let Some(e) = self.lookup.get(&point) {
            match e {
                Entity::Sensor => 'S',
                Entity::Beacon => 'B',
            }
        } else {
            let reach = self.sensors.iter().any(|s| s.can_reach(point));
            if reach {
                '#'
            } else {
                '.'
            }
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.render())
    }
}
