use ratatui::widgets::{List, ListItem, ListState};

pub struct Tables {
    pub names: Vec<String>,
    pub state: ListState,
}

impl Tables {
    pub fn new(names: Vec<String>) -> Self {
        Self {
            names,
            state: ListState::default(),
        }
    }

    pub fn list(&self) -> List {
        let items = self
            .names
            .iter()
            .map(|n| n.clone())
            .map(|n| ListItem::new(n))
            .collect::<Vec<_>>();
        List::new(items)
    }

    pub fn next(&mut self) {
        let i = self
            .state
            .selected()
            .map(|i| if i >= self.names.len() - 1 { 0 } else { i + 1 })
            .unwrap_or(0);
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = self
            .state
            .selected()
            .map(|i| if i == 0 { self.names.len() - 1 } else { i - 1 })
            .unwrap_or(0);
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}
