use crate::prelude::*;

struct Pager<T> {
    items: Vec<T>,
    top: usize,
    rows: usize,
}

impl<T> Pager<T> {
    fn size(&mut self, rows: usize) {
        self.rows = rows;
    }
}

impl<T, I> From<I> for Pager<T>
where
    I: IntoIterator<Item = T>,
{
    fn from(items: I) -> Self {
        let top = 0;
        let rows = 0;
        let items: Vec<T> = items.into_iter().collect();
        Self { items, top, rows }
    }
}

mod tests {
    use super::*;
    #[test]
    fn test_pager_from() {
        let mut p: Pager<i32> = [1, 2, 3, 4, 5].into();
        assert_eq!(p.top, 0);
        assert_eq!(p.rows, 0);
        p.size(100);
        assert_eq!(p.rows, 100);
    }
}
