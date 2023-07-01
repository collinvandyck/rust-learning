#![allow(dead_code)]

use std::{fmt::Display, thread, vec};

fn main() {
    println!("Hello, world!");
    let city: City = "Atlanta".into();
    println!("{}", city);
    let cities: Vec<City> = vec!["Atlanta".into(), "Miami".into()];
    let th = start_sort_thread(cities, Statistic::Max);
    let cities = th.join().expect("sort failed");
    println!("{:?}", cities);
}

fn start_sort_thread(mut cities: Vec<City>, stat: Statistic) -> thread::JoinHandle<Vec<City>> {
    let key_fn = move |city: &City| -> i64 { city.get_statistic(&stat) };
    thread::spawn(move || {
        cities.sort_by_key(key_fn);
        cities
    })
}

enum Statistic {
    Max,
    Min,
    Avg,
}

#[derive(Debug)]
struct City {
    name: String,
}

impl From<&str> for City {
    fn from(value: &str) -> Self {
        Self {
            name: value.to_string(),
        }
    }
}

impl Display for City {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl City {
    fn get_statistic(&self, _stat: &Statistic) -> i64 {
        0
    }
}
