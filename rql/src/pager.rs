use crate::prelude::*;

struct Pager<T> {
    items: Vec<T>,
    top: usize,
    viewport_rows: usize,
    pos: Option<usize>,
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
        Self {
            items,
            top,
            viewport_rows,
            pos,
        }
    }
}

mod tests {
    use super::*;
    #[test]
    fn test_pager() {
        let nums: Vec<_> = (0..10).collect();
        let p = Pager::from(nums).viewport_rows(5);
        assert_eq!(p.pos, Some(0));
        assert_eq!(p.viewport_rows, 5);
        assert_eq!(p.top, 0);
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
