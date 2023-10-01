use crate::prelude::*;

#[derive(Default)]
pub struct Pager {
    pub top: usize,
    pub viewport_rows: usize,
    pub pos: Option<usize>,
    pub count: usize,
}

impl Pager {
    pub fn count(mut self, v: u64) -> Self {
        self.count = v.try_into().unwrap();
        self
    }

    #[must_use]
    fn viewport_rows(mut self, rows: usize) -> Self {
        self.set_viewport_rows(rows);
        self
    }

    pub fn set_viewport_rows(&mut self, rows: usize) {
        self.viewport_rows = rows;
    }

    fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn select(&mut self, pos: usize) {
        if self.is_empty() {
            return;
        }
        if let Some(p) = &mut self.pos {
            *p = pos;
        } else {
            self.pos = Some(pos);
        }
    }

    pub fn unselect(&mut self) {
        self.pos = None;
    }

    pub fn next(&mut self) {
        if self.is_empty() {
            return;
        }
        let Some(pos) = self.pos.as_mut() else { return };
        if *pos >= self.count - 1 {
            // start at the beginning
            self.top = 0;
            *pos = 0;
        } else {
            // bump forward
            *pos += 1;
            if *pos >= self.viewport_rows + self.top {
                self.top = (*pos)
                    .checked_sub(self.viewport_rows - 1)
                    .unwrap_or_default();
            }
        }
    }

    pub fn prev(&mut self) {
        if self.is_empty() {
            return;
        }
        let Some(pos) = self.pos.as_mut() else { return };
        if *pos == 0 {
            // start at the end
            *pos = self.count - 1;
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

    fn relative_pos(&self) -> Option<usize> {
        match self.pos {
            Some(pos) if pos >= self.top => Some(pos - self.top),
            _ => None,
        }
    }

    pub fn top_pos_rel(&self) -> (usize, Option<usize>, Option<usize>) {
        (self.top, self.pos, self.relative_pos())
    }
}

mod tests {
    use super::*;
    #[test]
    fn test_pager() {
        let mut p = Pager::default().count(0).viewport_rows(5);
        p.select(0);
        assert_eq!(p.top_pos_rel(), (0, None, None));
        p.next();
        assert_eq!(p.top_pos_rel(), (0, None, None));

        let mut p = Pager::default().count(5).viewport_rows(3);
        p.select(0);

        // verify that top keeps up as we move forward
        assert_eq!(p.top_pos_rel(), (0, Some(0), Some(0)));
        p.next();
        assert_eq!(p.top_pos_rel(), (0, Some(1), Some(1)));
        p.next();
        assert_eq!(p.top_pos_rel(), (0, Some(2), Some(2)));
        p.next();
        assert_eq!(p.top_pos_rel(), (1, Some(3), Some(2)));
        p.next();
        assert_eq!(p.top_pos_rel(), (2, Some(4), Some(2)));
        p.next();
        assert_eq!(p.top_pos_rel(), (0, Some(0), Some(0)));

        // move backwards a bit.
        p.prev();
        assert_eq!(p.top_pos_rel(), (2, Some(4), Some(2)));
        p.prev();
        assert_eq!(p.top_pos_rel(), (2, Some(3), Some(1)));
        p.prev();
        assert_eq!(p.top_pos_rel(), (2, Some(2), Some(0)));
        p.prev();
        assert_eq!(p.top_pos_rel(), (1, Some(1), Some(0)));
        p.prev();
        assert_eq!(p.top_pos_rel(), (0, Some(0), Some(0)));
        p.prev();
        assert_eq!(p.top_pos_rel(), (2, Some(4), Some(2)));
    }
}
