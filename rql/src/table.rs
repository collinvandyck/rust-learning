use anyhow::Result;
use ratatui::widgets::{ListState, TableState};
use tracing::info;

use crate::dao::{BlockingDao, GetRecords, Record, Records, TableSchema};

/// Enables the display of a table's contents
pub struct DbTable {
    dao: BlockingDao,
    pub schema: TableSchema,
    records: Records,
    pub state: TableState,
    pub count: u64,
}

impl DbTable {
    pub fn new(dao: BlockingDao, name: String) -> Result<Self> {
        info!(name, "Building db table");
        let count = dao.count(&name)?;
        let schema = dao.table_schema(&name)?;
        let records = dao.records(&schema, GetRecords::new(&name))?;
        let state = TableState::default();
        let table = Self {
            dao,
            schema,
            records,
            state,
            count,
        };
        Ok(table)
    }

    pub fn records<'a>(&'a self) -> &[Record] {
        return &self.records;
    }

    pub fn name<'a>(&'a self) -> &'a str {
        return &self.schema.name;
    }

    pub fn next(&mut self) {
        let i = self
            .state
            .selected()
            .map(|i| {
                if self.records.is_empty() {
                    return 0;
                }
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
                if self.records.is_empty() {
                    return 0;
                }
                if i == 0 {
                    self.records.len() - 1
                } else {
                    i - 1
                }
            })
            .unwrap_or(0);
        self.state.select(Some(i));
    }

    pub fn select_first(&mut self) {
        if self.records.is_empty() {
            return;
        }
        self.state.select(Some(0));
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
