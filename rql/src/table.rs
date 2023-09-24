use std::collections::HashMap;

use crate::{pager::Pager, prelude::*};

/// Enables the display of a table's contents
pub struct DbTable {
    dao: BlockingDao,
    pub schema: TableSchema,
    max_lens: HashMap<TableColumn, usize>,
    pub pager: Pager,
    pub count: u64,
}

impl DbTable {
    pub fn new(dao: BlockingDao, name: String) -> Result<Self> {
        info!(name, "Building db table");
        let count = dao.count(&name)?;
        let schema = dao.table_schema(&name)?;
        let max_lens = HashMap::default();
        let mut pager = Pager::default().count(count);
        let mut table = Self {
            dao,
            schema,
            max_lens,
            pager,
            count,
        };
        Ok(table)
    }

    pub fn set_viewport_rows(&mut self, rows: usize) {
        self.pager.set_viewport_rows(rows);
    }

    pub fn max_len(&self, col: &TableColumn, dfvalue: usize) -> usize {
        *self.max_lens.get(col).unwrap_or(&dfvalue)
    }

    /*
    fn fetch(&mut self) -> Result<()> {
        self.pager.items = self
            .dao
            .records(&self.schema, GetRecords::new(&self.schema.name))?;
        let cols = &self.schema.cols;
        for record in self.pager.items.iter() {
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
    */

    pub fn records<'a>(&'a self) -> (Vec<Record>, TableState) {
        return (vec![], TableState::default());
    }

    pub fn name<'a>(&'a self) -> &'a str {
        return &self.schema.name;
    }

    pub fn next(&mut self) {
        self.pager.next();
    }

    pub fn previous(&mut self) {
        self.pager.prev();
    }

    pub fn select_first(&mut self) {
        self.pager.select(0);
    }

    pub fn unselect(&mut self) {
        self.pager.unselect();
    }
}
