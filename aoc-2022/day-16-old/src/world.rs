use crate::prelude::*;

use std::{collections::HashMap, rc::Rc};

#[derive(Debug, Clone)]
pub struct Map {
    valves: Vec<Rc<Valve>>,
    lookup: HashMap<String, Rc<Valve>>,
}

impl Map {
    pub fn new<I>(valves: I) -> Self
    where
        I: IntoIterator<Item = Valve>,
    {
        let valves = valves.into_iter().map(Rc::new).collect::<Vec<_>>();
        let lookup = valves.iter().map(|v| (v.name.clone(), v.clone())).collect();
        Self { valves, lookup }
    }
    pub fn get(&self, name: &str) -> Rc<Valve> {
        self.lookup.get(name).cloned().unwrap()
    }
    pub fn valves(&self) -> Vec<Rc<Valve>> {
        self.valves.clone()
    }
}
