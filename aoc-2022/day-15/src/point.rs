use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct Point(pub i32, pub i32);

impl Point {
    fn x(&self) -> i32 {
        self.0
    }
    fn y(&self) -> i32 {
        self.1
    }
    pub fn distance(self, rhs: Self) -> i32 {
        let x = (self.x() - rhs.x()).abs();
        let y = (self.y() - rhs.y()).abs();
        x + y
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::*;

    #[test]
    fn test_points_ordering() {
        let v1 = vec![Point(0, 0), Point(1, 0)];
        let mut v2 = v1.clone();
        v2.sort();
        assert_eq!(vec![Point(0, 0), Point(1, 0)], v2)
    }

    #[test]
    fn test_sub() {
        assert_eq!(0, Point(0, 0).distance(Point(0, 0)));
        assert_eq!(42, Point(0, 0).distance(Point(42, 0)));
        assert_eq!(42, Point(0, 0).distance(Point(0, 42)));
        assert_eq!(42, Point(2, 20).distance(Point(4, -20)));
        assert_eq!(15, Point(-5, -10).distance(Point(-10, -20)));
    }
}
