use std::rc::Rc;

pub struct Map {
    valves: Vec<Rc<Valve>>,
}

#[derive(Debug)]
pub struct Valve {
    name: String,
    rate: i32,
    nexts: Vec<Rc<Valve>>,
}

impl Valve {
    pub fn new(name: String, rate: i32) -> Self {
        Self {
            name,
            rate,
            nexts: Vec::new(),
        }
    }
    pub fn add_next(&mut self, other: Rc<Valve>) {
        self.nexts.push(other);
    }
}
