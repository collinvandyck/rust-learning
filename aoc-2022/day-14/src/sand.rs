#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Point(i32, i32);

#[derive(Debug)]
pub struct Formation(Vec<Point>);

impl Formation {
    pub fn parse(line: String) -> Self {
        let points = line
            .split(" -> ")
            .map(|s| {
                let nums = s
                    .split(",")
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect::<Vec<_>>();
                let x = nums.get(0).unwrap();
                let y = nums.get(1).unwrap();
                Point(*x, *y)
            })
            .collect::<Vec<_>>();
        Self(points)
    }
}
