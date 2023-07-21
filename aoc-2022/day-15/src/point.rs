use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
pub struct Point(pub i32, pub i32);

impl Point {
    fn x(self) -> i32 {
        self.0
    }
    fn y(self) -> i32 {
        self.1
    }
    pub fn distance(self, rhs: Self) -> i32 {
        let x = (self.x() - rhs.x()).abs();
        let y = (self.y() - rhs.y()).abs();
        x + y
    }
    pub fn min_max<P>(points: P) -> Option<(Point, Point)>
    where
        P: IntoIterator<Item = Point>,
    {
        let start: Option<(Point, Point)> = None;
        points.into_iter().fold(start, |mut acc, point| {
            let (mut min, mut max) = acc.get_or_insert((point, point));
            min.0 = min.0.min(point.0);
            min.1 = min.1.min(point.1);
            max.0 = max.0.max(point.0);
            max.1 = max.1.max(point.1);
            Some((min, max))
        })
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

    fn sorted(mut v: Vec<Point>) -> Vec<Point> {
        v.sort();
        v
    }

    #[test]
    fn test_min_max() {
        assert_eq!(
            Point::min_max([Point(0, 0), Point(1, 0)]),
            Some((Point(0, 0), Point(1, 0))),
        );
        assert_eq!(
            Point::min_max([Point(1, 0), Point(0, 0)]),
            Some((Point(0, 0), Point(1, 0))),
        );
        assert_eq!(
            Point::min_max([Point(0, 0), Point(3, 3), Point(-3, -3)]),
            Some((Point(-3, -3), Point(3, 3))),
        );
        assert_eq!(
            Point::min_max([Point(3, 3), Point(-3, -3), Point(0, 0)]),
            Some((Point(-3, -3), Point(3, 3))),
        );
        assert_eq!(
            Point::min_max([Point(3, 8), Point(-8, -3), Point(0, -5)]),
            Some((Point(-8, -5), Point(3, 8))),
        );
    }

    #[test]
    fn test_points_ordering() {
        assert_eq!(
            sorted(vec![Point(0, 0), Point(1, 0)]),
            vec![Point(0, 0), Point(1, 0)],
        );
        assert_eq!(
            sorted(vec![Point(1, 0), Point(0, 0)]),
            vec![Point(0, 0), Point(1, 0)],
        );
        assert_eq!(
            sorted(vec![Point(0, 0), Point(3, 3), Point(-3, -3)]),
            vec![Point(-3, -3), Point(0, 0), Point(3, 3)],
        );
        assert_eq!(
            sorted(vec![Point(3, 3), Point(-3, -3), Point(0, 0)]),
            vec![Point(-3, -3), Point(0, 0), Point(3, 3)],
        );
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
