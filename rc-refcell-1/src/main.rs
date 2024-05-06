#![allow(unused)]

use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
};

fn main() {
    let env = Env::new();
    env.set(0, 41);
    env.set(1, 42);
    assert_eq!(env.get(&1), Some(42));

    let child = env.child();
    child.set(1, 43);
    child.set(2, 44);
    assert_eq!(child.get(&0), Some(41));
    assert_eq!(child.get(&1), Some(43));
    assert_eq!(child.get(&2), Some(44));
    drop(child);

    assert_eq!(env.get(&0), Some(41));
    assert_eq!(env.get(&1), Some(42));
}

type Values = HashMap<i32, i32>;

#[derive(Default)]
struct Env {
    parent: Option<Inner>,
    inner: Inner,
}

impl Env {
    fn new() -> Self {
        Self::default()
    }

    fn child(&self) -> Self {
        Self {
            parent: Some(self.inner.clone()),
            inner: Inner::default(),
        }
    }

    fn set(&self, k: i32, v: i32) {
        self.inner.set(k, v);
    }

    fn get(&self, k: &i32) -> Option<i32> {
        self.inner.get(k).or_else(|| {
            if let Some(parent) = &self.parent {
                parent.get(k)
            } else {
                None
            }
        })
    }
}

#[derive(Clone, Default, derive_more::Deref, derive_more::DerefMut)]
struct Inner {
    vals: Rc<RefCell<Values>>,
}

impl Inner {
    fn new() -> Self {
        Self::default()
    }

    fn set(&self, k: i32, v: i32) {
        let mut m = RefCell::borrow_mut(&self.vals);
        m.insert(k, v);
    }

    fn get(&self, k: &i32) -> Option<i32> {
        let m = RefCell::borrow(&self.vals);
        m.get(k).cloned()
    }
}
