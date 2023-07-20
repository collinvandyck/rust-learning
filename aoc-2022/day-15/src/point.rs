#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Point(i32, i32);

impl Point {
    fn x(&self) -> i32 {
        self.0
    }
    fn y(&self) -> i32 {
        self.1
    }
    fn distance(self, rhs: Self) -> i32 {
        let x = (self.x() - rhs.x()).abs();
        let y = (self.y() - rhs.y()).abs();
        x + y
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_sub() {
        assert_eq!(0, Point(0, 0).distance(Point(0, 0)));
        assert_eq!(42, Point(0, 0).distance(Point(42, 0)));
        assert_eq!(42, Point(0, 0).distance(Point(0, 42)));
        assert_eq!(42, Point(2, 20).distance(Point(4, -20)));
        assert_eq!(15, Point(-5, -10).distance(Point(-10, -20)));
    }
}
