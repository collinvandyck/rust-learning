use anyhow::Result;

use crate::dao::BlockingDao;

/// Enables the display of a table's contents
pub struct Table {
    dao: BlockingDao,
    name: String,
}

impl Table {
    pub fn new(dao: BlockingDao, name: String) -> Result<Self> {
        let table = Self { dao, name };
        Ok(table)
    }
}
