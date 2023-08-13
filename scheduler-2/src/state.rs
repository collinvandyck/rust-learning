use std::collections::HashMap;

use crate::{rules::Rules, task};

/// State is used to keep track of the currently running tasks.
///
/// It is meant to be used by a single task that is controlling what gets run or what does not, and
/// because of that it is not threadsafe by design.
pub(crate) struct State<'a> {
    running: HashMap<task::Type, bool>,
    rules: &'a Rules,
}

impl<'a> State<'a> {
    pub(crate) fn new(rules: &'a Rules) -> Self {
        Self {
            running: HashMap::default(),
            rules,
        }
    }
    /// Returns the number of currently executing tasks
    pub(crate) fn running(&self) -> usize {
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
