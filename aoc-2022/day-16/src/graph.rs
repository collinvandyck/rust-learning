use crate::prelude::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Network {
    valves: HashMap<Name, Valve>,
}

impl Network {
    pub fn new(iter: impl Iterator<Item = Valve>) -> Self {
        let mut valves = HashMap::new();
        for valve in iter {
            valves.insert(valve.name, valve);
        }
        Self { valves }
    }
}

pub type Path = Vec<(Name, Name)>;
