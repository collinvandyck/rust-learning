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
        if v > 0 {
            self.pos = Some(0);
        }
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
            if *pos - self.top >= self.viewport_rows + 1 {
                self.top = *pos - (self.viewport_rows + 1);
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

    fn relative_pos(&self) -> usize {
        match self.pos {
            Some(pos) if pos >= self.top => pos - self.top,
            _ => 0,
        }
    }

    pub fn top_pos_rel(&self) -> (usize, usize, usize) {
        (self.top, self.pos.unwrap_or(0), self.relative_pos())
    }

    /*
    pub fn state<'a>(&'a self) -> (&'a [T], TableState) {
        if self.items.is_empty() {
            return (&self.items, TableState::default());
        }
        let Some(pos) = self.pos else {
            return (&self.items, TableState::default());
        };
        let (top, pos, rel) = self.top_pos_rel();
        let end_idx = (self.top + self.viewport_rows).min(self.items.len());
        let items = &self.items[self.top..end_idx];
        let mut state = TableState::default();
        state.select(Some(rel));
        (items, state)
    }
    */
}

mod tests {
    use super::*;
    #[test]
    fn test_pager() {
        let mut p = Pager::default().count(0).viewport_rows(5);
        assert_eq!(p.top_pos_rel(), (0, 0, 0));
        p.next();
        assert_eq!(p.top_pos_rel(), (0, 0, 0));

        let mut p = Pager::default().count(5).viewport_rows(3);

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
}
