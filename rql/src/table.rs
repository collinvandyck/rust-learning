use crate::prelude::*;
use std::{collections::HashMap, fmt::Debug};

/// Enables the display of a table's contents
pub struct DbTable {
    dao: BlockingDao,
    pub schema: TableSchema,
    max_lens: HashMap<TableColumn, usize>,
    pub pager: Pager,
    pub count: u64,
    pub indexed: IndexedRecords,
    search: Search,
}

#[derive(Default)]
pub struct IndexedRecord(usize, pub Record);

impl Debug for IndexedRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("IndexedRecord").field(&self.0).finish()
    }
}

impl IndexedRecord {
    fn index(&self) -> usize {
        self.0
    }
}

#[derive(Default)]
pub struct IndexedRecords(Vec<IndexedRecord>);

impl IndexedRecords {
    fn first_index(&self) -> Option<usize> {
        self.0.first().map(|r| r.index())
    }

    fn last_index(&self) -> Option<usize> {
        self.0.last().map(|r| r.index())
    }

    fn index(&self) -> Option<(usize, usize)> {
        if let Some(first) = self.first_index() {
            if let Some(last) = self.last_index() {
                return Some((first, last));
            }
        }
        None
    }

    fn contains(&self, first: usize, last: usize) -> bool {
        self.index()
            .map(|(f, l)| f <= first && l >= last - 1)
            .unwrap_or_default()
    }

    fn range(&self, first: usize, last: usize) -> Vec<Record> {
        trace!(first, last, "IndexedRecords::range");
        assert!(first <= last);
        self.0
            .iter()
            .filter(|r| {
                trace!(?r, "IndexedRecords::range::filter");
                let idx = r.index();
                idx >= first && idx < last
            })
            .map(|r| r.1.clone())
            .collect()
    }
}

impl DbTable {
    pub fn new(dao: BlockingDao, name: String, search: Search) -> Result<Self> {
        info!(name, "Building db table");
        let count = dao.count(&name)?;
        let schema = dao.table_schema(&name)?;
        let max_lens = dao.max_lens(&schema)?;
        let max_lens: HashMap<TableColumn, usize> = schema
            .cols
            .iter()
            .zip(max_lens.iter())
            .map(|f| {
                let (col, len) = f;
                (col.clone(), *len)
            })
            .collect();
        let mut pager = Pager::default().count(count);
        let indexed = IndexedRecords::default();
        let mut table = Self {
            dao,
            schema,
            max_lens,
            pager,
            count,
            indexed,
            search,
        };
        Ok(table)
    }

    pub fn set_viewport_rows(&mut self, rows: usize) {
        self.pager.set_viewport_rows(rows);
    }

    pub fn max_len(&self, col: &TableColumn, dfvalue: usize) -> usize {
        *self.max_lens.get(col).unwrap_or(&dfvalue)
    }

    pub fn records(&mut self) -> Result<(Vec<Record>, TableState)> {
        let view_rows = self.pager.viewport_rows;
        let (start, pos, rel) = self.pager.top_pos_rel();
        let end = (start + view_rows).min(self.pager.count);
        let index = self.indexed.index();
        trace!(start, end, pos, rel, ?index, "Records");

        // fetch a new window if necessary
        let contains = self.indexed.contains(start, end);
        if !contains {
            let offset = if start >= view_rows {
                start - view_rows
            } else {
                0
            };
            let limit = view_rows * 3;
            let spec = GetRecords::new(&self.schema.name)
                .offset(offset)
                .limit(limit);
            let records = self.dao.records(&self.schema, spec)?;
            let irs = (offset..offset + limit)
                .zip(records.into_iter())
                .map(|(idx, record)| IndexedRecord(idx, record))
                .collect::<Vec<_>>();
            trace!(?irs, "irs");
            self.indexed = IndexedRecords(irs);
            let index = self.indexed.index();
            trace!(
                limit,
                offset,
                start,
                end,
                pos,
                rel,
                ?index,
                "Fetched {} records",
                self.indexed.0.len(),
            );
        }
        let records = self.indexed.range(start, end);
        trace!("Indexed Records: {}", records.len());
        let mut state = TableState::default();
        state.select(rel);
        Ok((records, state))
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
