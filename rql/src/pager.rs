use crate::prelude::*;

#[derive(Default)]
pub struct Pager<T> {
    pub items: Vec<T>,
    pub top: usize,
    pub viewport_rows: usize,
    pub pos: Option<usize>,
}

impl<T> Pager<T> {
    #[must_use]
    fn viewport_rows(mut self, rows: usize) -> Self {
        self.set_viewport_rows(rows);
        self
    }

    pub fn set_viewport_rows(&mut self, rows: usize) {
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

    pub fn select(&mut self, pos: usize) {
        if self.items.is_empty() {
            return;
        }
        if let Some(p) = &mut self.pos {
            *p = pos;
        }
    }

    pub fn unselect(&mut self) {
        self.pos = None;
    }

    pub fn next(&mut self) {
        let Some(pos) = self.pos.as_mut() else { return };
        if self.items.is_empty() {
            return;
        };
        if *pos >= self.items.len() - 1 {
            // start at the beginning
            self.top = 0;
            *pos = 0;
        } else {
            // bump forward
            *pos += 1;
            if *pos - self.top >= self.viewport_rows {
                self.top = *pos - self.viewport_rows + 1;
            }
        }
    }

    pub fn prev(&mut self) {
        let Some(pos) = self.pos.as_mut() else { return };
        if *pos == 0 {
            // start at the end
            *pos = self.items.len() - 1;
            if *pos >= self.viewport_rows {
                self.top = *pos - self.viewport_rows + 1;
            } else {
                self.top = 0;
            }
        } else {
            // bump backward
            *pos -= 1;
            if *pos < self.top {
                self.top = *pos;
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
        self.pos.unwrap_or(0) - self.top
    }

    fn top_pos_rel(&self) -> (usize, usize, usize) {
        (self.top, self.pos.unwrap_or(0), self.relative_pos())
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
        let pos = None;
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
        assert_eq!(p.top_pos_rel(), (1, 3, 2));
        p.next();
        assert_eq!(p.top_pos_rel(), (2, 4, 2));
        p.next();
        assert_eq!(p.top_pos_rel(), (0, 0, 0));

        // move backwards a bit.
        p.prev();
        // failing with (1,4,3)
        assert_eq!(p.top_pos_rel(), (2, 4, 2));
        p.prev();
        assert_eq!(p.top_pos_rel(), (2, 3, 1));
        p.prev();
        assert_eq!(p.top_pos_rel(), (2, 2, 0));
        p.prev();
        assert_eq!(p.top_pos_rel(), (1, 1, 0));
        p.prev();
        assert_eq!(p.top_pos_rel(), (0, 0, 0));
        p.prev();
        assert_eq!(p.top_pos_rel(), (2, 4, 2));
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
