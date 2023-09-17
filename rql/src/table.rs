use anyhow::Result;
use ratatui::widgets::{ListState, TableState};

use crate::dao::{BlockingDao, Record, Records, TableSchema};

/// Enables the display of a table's contents
pub struct DbTable {
    dao: BlockingDao,
    pub name: String,
    pub schema: TableSchema,
    pub records: Records,
    pub state: TableState,
}

impl DbTable {
    pub fn new(dao: BlockingDao, name: String) -> Result<Self> {
        let schema = dao.table_schema(&name)?;
        let records = dao.records(&name, &schema)?;
        let state = TableState::default();
        let table = Self {
            dao,
            name,
            schema,
            records,
            state,
        };
        Ok(table)
    }

    pub fn next(&mut self) {
        let i = self
            .state
            .selected()
            .map(|i| {
                if i >= self.records.len() - 1 {
                    0
                } else {
                    i + 1
                }
            })
            .unwrap_or(0);
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = self
            .state
            .selected()
            .map(|i| {
                if i == 0 {
                    self.records.len() - 1
                } else {
                    i - 1
                }
            })
            .unwrap_or(0);
        self.state.select(Some(i));
    }

    pub fn selected(&self) -> Option<&Record> {
        self.state
            .selected()
            .map(|i| self.records.get(i).map(|s| s.clone()))
            .flatten()
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}
