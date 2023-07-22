use std::{collections::HashSet, fmt::Display, hash::Hash, rc::Rc};

/// Valves represents the current state of all valves. Which
/// valves are open and which are closed, and which valve is
/// the current valve.
///
/// It is meant to be quickly compared to other valve states
/// to avoid duplication of work that cannot result in a
/// higher score.
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Valves {
    current: Rc<Valve>,
    open: HashSet<Rc<Valve>>,
    clos: HashSet<Rc<Valve>>,
}

impl Valves {
    pub fn new(current: Rc<Valve>, valves: Vec<Rc<Valve>>) -> Self {
        let mut open = HashSet::new();
        let mut clos = HashSet::new();
        for valve in valves {
            if valve.rate == 0 {
                // we set valves to open that have a rate of
                // 0 so we won't try to open them later.
                open.insert(valve);
            } else {
                clos.insert(valve);
            }
        }
        Self {
            current,
            open,
            clos,
        }
    }
    pub fn all_open(&self) -> bool {
        self.clos.is_empty()
    }
    pub fn sum_open_rates(&self) -> i64 {
        self.open.iter().map(|v| v.rate as i64).sum()
    }
    pub fn move_to(&mut self, valve: Rc<Valve>) {
        self.current = valve;
    }
    pub fn open_current(&mut self) {
        assert!(self.open.insert(self.current.clone()));
        assert!(self.clos.remove(&self.current));
    }
    pub fn can_open_current(&self) -> bool {
        self.clos.contains(&self.current)
    }
    pub fn tunnels(&self) -> impl Iterator<Item = &String> {
        self.current.tunnels.iter()
    }
}

impl Display for Valves {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(open:{} closed:{} cur:{})",
            self.open.len(),
            self.clos.len(),
            self.current.name
        )
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
        state.write(self.name.as_bytes());
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

#[cfg(test)]
mod test {
    use super::*;

    #[derive(PartialEq, Eq, Debug)]
    struct State {
        open: HashSet<Rc<Valve>>,
        clos: HashSet<Rc<Valve>>,
    }

    impl State {}

    #[test]
    fn test_saved_state_super_struct() {
        let v1 = Rc::new(Valve::new("AA", 5, vec!["CC".to_string()]));
        let v2 = Rc::new(Valve::new("BB", 5, vec![]));
        let v3 = Rc::new(Valve::new("CC", 5, vec![]));

        let open: HashSet<Rc<Valve>> = [&v1, &v2].into_iter().map(|s| s.clone()).collect();
        let closed: HashSet<Rc<Valve>> = [&v3].into_iter().map(|s| s.clone()).collect();
        let mut s1 = State { open, clos: closed };

        let open: HashSet<Rc<Valve>> = [&v1, &v2].into_iter().map(|s| s.clone()).collect();
        let closed: HashSet<Rc<Valve>> = [&v3].into_iter().map(|s| s.clone()).collect();
        let mut s2 = State { open, clos: closed };

        assert_eq!(s1, s2);

        s1.open.insert(v3.clone());
        assert_ne!(s1, s2);
        s2.open.insert(v3.clone());
        assert_eq!(s1, s2);
        assert_eq!(s1, s2);

        s1.clos.clear();
        s1.open.clear();
        s2.clos.clear();
        s2.open.clear();
        assert_eq!(s1, s2);

        s1.open.insert(v1.clone());
        s2.clos.insert(v1.clone());
        assert_ne!(s1, s2);
        s1.clos.insert(v1.clone());
        assert_ne!(s1, s2);
        s2.open.insert(v1.clone());
        assert_eq!(s1, s2);
    }

    #[test]
    fn test_hashset_with_rc() {
        let mut hs: HashSet<Rc<Valve>> = HashSet::new();
        let v1 = Rc::new(Valve::new("AA", 5, vec!["CC".to_string()]));
        let v2 = Rc::new(Valve::new("BB", 5, vec![]));

        // insertions
        assert!(!hs.contains(&v1));
        hs.insert(v1.clone());
        assert!(hs.contains(&v1));
        assert!(!hs.contains(&v2));
        hs.insert(v1.clone());
        assert!(hs.contains(&v1));
        assert!(!hs.contains(&v2));
        assert_eq!(1, hs.len());
        hs.insert(v2.clone());
        assert!(hs.contains(&v1));
        assert!(hs.contains(&v2));
        assert_eq!(2, hs.len());

        // deletions
        hs.remove(&v2);
        assert!(hs.contains(&v1));
        assert!(!hs.contains(&v2));
        assert_eq!(1, hs.len());
        hs.remove(&v1);
        assert!(!hs.contains(&v1));
        assert!(!hs.contains(&v2));
        assert_eq!(0, hs.len());
    }
}
