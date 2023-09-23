use crate::prelude::*;

struct Pager<T> {
    items: Vec<T>,
    top: usize,
    size: usize,
}

impl<T> Pager<T> {
    fn size(&mut self, size: usize) {
        self.size = size;
    }
}

impl<T, I> From<I> for Pager<T>
where
    I: IntoIterator<Item = T>,
{
    fn from(items: I) -> Self {
        let top = 0;
        let size = 0;
        let items: Vec<T> = items.into_iter().collect();
        Self { items, top, size }
    }
}

mod tests {
    use super::*;
    #[test]
    fn test_pager_from() {
        let mut p: Pager<i32> = [1, 2, 3, 4, 5].into();
        assert_eq!(p.top, 0);
        assert_eq!(p.size, 0);
        p.size(100);
        assert_eq!(p.size, 100);
    }
}
