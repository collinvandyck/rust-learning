use std::{collections::HashMap, time::Duration};

use crate::task;

/// Rules govern how things can and cannot be scheduled.
///
/// Each task type can have its own rule, and there is a default rule
/// that applies to task types that do not have a specific rule.
pub struct Rules {
    rules: HashMap<task::Type, Rule>,
    default: Rule,
}

impl Rules {
    #[must_use]
    pub fn builder() -> Builder {
        Builder::new()
    }

    pub fn get(&self, typ: &task::Type) -> &Rule {
        self.rules.get(typ).unwrap_or(&self.default)
    }
}

pub struct Rule {
    pub max_running: usize,
    pub run_every: Option<Duration>,
}

impl Default for Rules {
    fn default() -> Self {
        Self {
            rules: HashMap::default(),
            default: Rule {
                max_running: 1,
                run_every: None,
            },
        }
    }
}

pub struct Builder {
    rules: Rules,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            rules: Rules::default(),
        }
    }
    #[must_use]
    pub fn default(mut self, rule: Rule) -> Self {
        self.rules.default = rule;
        self
    }
    #[must_use]
    pub fn rule<T: Into<task::Type>>(mut self, typ: T, rule: Rule) -> Self {
        let typ = typ.into();
        if self.rules.rules.contains_key(&typ) {
            panic!("duplicate rule for type: {:?}", typ);
        }
        self.rules.rules.insert(typ.clone(), rule);
        self
    }
    #[must_use]
    pub fn build(self) -> Rules {
        self.rules
    }
}
