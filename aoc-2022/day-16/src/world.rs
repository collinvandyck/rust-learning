use std::{collections::HashMap, rc::Rc};

#[derive(Debug)]
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
}

#[derive(Debug, Clone)]
pub struct Valve {
    pub name: String,
    pub rate: i32,
    tunnels: Vec<String>,
}

impl Valve {
    pub fn new(name: String, rate: i32, tunnels: Vec<String>) -> Self {
        Self {
            name,
            rate,
            tunnels,
        }
    }
}
