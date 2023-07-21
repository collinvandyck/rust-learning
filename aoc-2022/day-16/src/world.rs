use std::{collections::HashMap, hash::Hash, rc::Rc};

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Valve {
    pub name: String,
    pub rate: i32,
    pub tunnels: Vec<String>,
}

impl Hash for Valve {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write(self.name.as_bytes())
    }
}

impl Valve {
    pub fn new<T>(name: T, rate: i32, tunnels: Vec<String>) -> Self
    where
        T: Into<String>,
    {
        Self {
            name: name.into(),
            rate,
            tunnels,
        }
    }
}
