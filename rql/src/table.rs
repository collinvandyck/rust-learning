use std::collections::HashMap;

use crate::prelude::*;

/// Enables the display of a table's contents
pub struct DbTable {
    dao: BlockingDao,
    pub schema: TableSchema,
    records: Records,
    pub state: TableState,
    pub count: u64,
    max_lens: HashMap<TableColumn, usize>,
    top: usize, // the row (0-indexed) of the top of the viewport
}

impl DbTable {
    pub fn new(dao: BlockingDao, name: String) -> Result<Self> {
        info!(name, "Building db table");
        let count = dao.count(&name)?;
        let schema = dao.table_schema(&name)?;
        let records = Records::default();
        let state = TableState::default();
        let max_lens = HashMap::default();
        let top = 0;
        let mut table = Self {
            dao,
            schema,
            records,
            state,
            count,
            max_lens,
            top,
        };
        table.fetch()?;
        Ok(table)
    }

    pub fn max_len(&self, col: &TableColumn, dfvalue: usize) -> usize {
        *self.max_lens.get(col).unwrap_or(&dfvalue)
    }

    fn fetch(&mut self) -> Result<()> {
        self.records = self
            .dao
            .records(&self.schema, GetRecords::new(&self.schema.name))?;
        let cols = &self.schema.cols;
        for record in self.records.iter() {
            for (field_idx, field) in record.fields.iter().enumerate() {
                let col = &cols[field_idx];
                let val = &field.val;
                let len = val.len();
                let insert = match self.max_lens.get(col) {
                    Some(l) if &len < l => false,
                    _ => true,
                };
                if insert {
                    self.max_lens.insert(col.clone(), len);
                }
            }
        }
        Ok(())
    }

    pub fn records<'a>(&'a self, count: usize) -> (&[Record], TableState) {
        let Some(selected) = self.state.selected() else {
            return (&self.records, TableState::default());
        };

        // we want to return the records from top to the max window
        let num_records = self.records.len();
        let start_idx = self.top;
        let end_idx = (count + start_idx + 1).min(num_records);
        debug!(count, num_records, start_idx, end_idx, "Records");
        let records = &self.records[start_idx..end_idx];
        let mut state = self.state.clone();
        state.select(Some(selected - self.top));
        (records, state)
    }

    pub fn name<'a>(&'a self) -> &'a str {
        return &self.schema.name;
    }

    pub fn next(&mut self, rows: usize) {
        self.incr(1, rows);
    }

    pub fn previous(&mut self, rows: usize) {
        self.incr(-1, rows);
    }

    pub fn select_first(&mut self) {
        self.state.select(if self.records.is_empty() {
            None
        } else {
            Some(0)
        });
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }

    fn incr(&mut self, amt: i64, rows: usize) {
        if self.records.is_empty() {
            self.top = 0;
            self.state.select(Some(0));
            return;
        }
        if self.state.selected().is_none() {
            self.top = 0;
            self.state.select(Some(0));
            return;
        }
        let num_records = self.records.len();
        let selected = self.state.selected().unwrap_or_default();
        let selected: i64 = selected.try_into().unwrap();
        if amt < 0 {
            // we are moving up
            let selected = selected + amt;
            if selected < 0 {
                // we wrap around to the end
                let selected = num_records - 1;
                self.top = if selected > rows { selected - rows } else { 0 };
                self.state.select(Some(selected));
            } else {
                // we can bump up
                let selected: usize = selected.try_into().unwrap();
                if self.top >= selected {
                    self.top = selected;
                }
                self.state.select(Some(selected));
            }
        } else {
            // we are moving down
            let selected = selected + amt;
            if selected >= num_records.try_into().unwrap() {
                // we wrap around to the beginning
                self.top = 0;
                self.state.select(Some(0));
            } else {
                // we can bump down
                let selected: usize = selected.try_into().unwrap();
                // if the top is outside of our window we need to bump it.
                if selected - self.top > rows {
                    self.top = selected - rows;
                }
                self.state.select(Some(selected));
            }
        }
    }
}
