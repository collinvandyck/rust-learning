#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Point(i32, i32);

impl Point {
    fn default() -> Self {
        Point(0, 0)
    }
}

#[derive(Debug)]
pub struct Formation(Vec<Point>);

impl Formation {
    pub fn parse(line: String) -> Self {
        let points = line
            .split(" -> ")
            .map(|s| {
                let mut nums = s.split(',').map(|s| s.parse::<i32>().unwrap());
                let x = nums.next().unwrap();
                let y = nums.next().unwrap();
                Point(x, y)
            })
            .collect::<Vec<_>>();
        Self(points)
    }
}

#[derive(Debug)]
pub struct Cave {
    min: Point,
    max: Point,
}

impl Cave {
    pub fn new(formations: Vec<Formation>) -> Cave {
        let mut min = Point::default();
        let mut max = Point::default();
        formations
            .iter()
            .flat_map(|f| &f.0)
            .enumerate()
            .for_each(|(idx, point)| {
                if idx == 0 {
                    min = *point;
                    max = *point;
                } else {
                    min.0 = i32::min(min.0, point.0);
                    min.1 = i32::min(min.1, point.1);
                    max.0 = i32::max(max.0, point.0);
                    max.1 = i32::max(max.1, point.1);
                }
            });
        Cave { min, max }
    }
}
