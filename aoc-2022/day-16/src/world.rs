use std::rc::Rc;

pub struct Map {
    valves: Vec<Rc<Valve>>,
}

pub struct Valve {
    name: String,
    rate: i32,
    nexts: Vec<String>,
}
