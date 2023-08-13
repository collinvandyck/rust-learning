use std::collections::HashMap;

use crate::task;

/// State is used to keep track of the currently running tasks.
pub(crate) struct State {
    running: HashMap<task::Type, bool>,
}

impl State {
    pub(crate) fn new() -> Self {
        Self {
            running: HashMap::default(),
        }
    }
    /// Returns the number of currently executing tasks
    pub(crate) fn num_running(&self) -> usize {
        self.running.len()
    }
    /// Return true if we are allowed to run this task type.
    pub(crate) fn try_run(&mut self, typ: &task::Type) -> bool {
        if self.running.contains_key(typ) {
            return false;
        }
        self.running.insert(typ.clone(), true);
        true
    }
    pub(crate) fn remove(&mut self, typ: &task::Type) {
        self.running.remove(typ);
    }
}
