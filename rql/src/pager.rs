use crate::prelude::*;

struct Pager<T> {
    items: Vec<T>,
    top: usize,
    viewport_rows: usize,
    pos: usize,
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

    fn next(&mut self) {
        if self.items.is_empty() {
            return;
        };
        if self.pos >= self.items.len() - 1 {
            // start at the beginning
            self.top = 0;
            self.pos = 0;
        } else {
            // bump forward
            self.pos += 1;
            if self.pos - self.top > self.viewport_rows {
                self.top = self.pos - self.viewport_rows;
            }
        }
    }

    fn prev(&mut self) {
        if self.items.is_empty() {
            return;
        };
        if self.pos == 0 {
            // start at the end
            self.pos = self.items.len() - 1;
            if self.pos >= self.viewport_rows {
                self.top = self.pos - self.viewport_rows;
            } else {
                self.top = 0;
            }
        } else {
            // bump backward
            self.pos -= 1;
            if self.pos < self.top {
                self.top = self.pos;
            }
        }
    }

    /// Returns the relative position to the total stream.
    ///
    /// So, if we have:
    /// - 10 element vec
    /// - viewport_rows: 3
    /// - top: 2
    /// - pos: 4
    ///
    /// The records we send back will be [2..4]
    /// Our pos returned will need to be 2 (last minus start)
    fn relative_pos(&self) -> usize {
        self.pos - self.top
    }

    fn top_pos_rel(&self) -> (usize, usize, usize) {
        (self.top, self.pos, self.relative_pos())
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
        let pos = 0;
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
        let nums: Vec<_> = (0..5).collect();
        let mut p = Pager::from(nums).viewport_rows(3);

        // verify that top keeps up as we move forward
        assert_eq!(p.top_pos_rel(), (0, 0, 0));
        p.next();
        assert_eq!(p.top_pos_rel(), (0, 1, 1));
        p.next();
        assert_eq!(p.top_pos_rel(), (0, 2, 2));
        p.next();
        // failing here with (0, 3, 3)
        assert_eq!(p.top_pos_rel(), (1, 3, 2));
        p.next();
        assert_eq!(p.top_pos_rel(), (2, 4, 2));
        p.next();
        assert_eq!(p.top_pos_rel(), (0, 0, 0));

        // move backwards a bit.
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
