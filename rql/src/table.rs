use anyhow::Result;

use crate::dao::{BlockingDao, TableSchema};

/// Enables the display of a table's contents
pub struct Table {
    dao: BlockingDao,
    name: String,
    schema: TableSchema,
}

impl Table {
    pub fn new(dao: BlockingDao, name: String) -> Result<Self> {
        let schema = dao.table_schema(&name)?;
        let table = Self { dao, name, schema };
        Ok(table)
    }
}
