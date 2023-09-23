use crate::prelude::*;

struct Pager<T> {
    items: Vec<T>,
    top: usize,
    viewport_rows: usize,
    pos: Option<usize>,
    table_state: TableState,
}

impl<T> Pager<T> {
    #[must_use]
    fn viewport_rows(mut self, rows: usize) -> Self {
        self.set_viewport_rows(rows);
        self
    }

    fn set_viewport_rows(&mut self, rows: usize) {
        self.viewport_rows = rows;
    }

    #[must_use]
    fn items<I: IntoIterator<Item = T>>(mut self, items: I) -> Self {
        self.set_items(items);
        self
    }

    fn set_items<I: IntoIterator<Item = T>>(&mut self, items: I) {
        let items = items.into_iter().collect::<Vec<_>>();
        self.items = items;
    }

    fn next(&mut self) {}

    fn prev(&mut self) {}

    fn snapshot(&self) -> Snapshot<T> {
        Snapshot {
            pager: self,
            table_state: self.table_state.clone(),
        }
    }
}

struct Snapshot<'a, T> {
    pager: &'a Pager<T>,
    table_state: TableState,
}

impl<T, I> From<I> for Pager<T>
where
    I: IntoIterator<Item = T>,
{
    fn from(items: I) -> Self {
        let top = 0;
        let viewport_rows = 0;
        let items: Vec<T> = items.into_iter().collect();
        let pos = if items.is_empty() { None } else { Some(0) };
        let table_state = TableState::default();
        Self {
            items,
            top,
            viewport_rows,
            pos,
            table_state,
        }
    }
}

fn table_state(val: Option<usize>) -> TableState {
    let mut state = TableState::default();
    state.select(val);
    state
}
mod tests {
    use super::*;
    #[test]
    fn test_pager() {
        let nums: Vec<_> = (0..10).collect();
        let mut p = Pager::from(nums).viewport_rows(5);
        let s = p.snapshot();
        assert_eq!(s.pager.pos, Some(0));
        assert_eq!(s.pager.viewport_rows, 5);
        assert_eq!(s.pager.top, 0);
        assert_eq!(s.table_state, table_state(None));

        p.next();
    }

    #[test]
    fn test_pager_from() {
        let mut p: Pager<i32> = [1, 2, 3, 4, 5].into();
        assert_eq!(p.top, 0);
        assert_eq!(p.viewport_rows, 0);
        p.set_viewport_rows(100);
        assert_eq!(p.viewport_rows, 100);
    }
}
