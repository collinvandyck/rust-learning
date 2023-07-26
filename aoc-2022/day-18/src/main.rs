use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use clap::Parser;

fn main() {
    let args = Args::parse();
    for line in BufReader::new(File::open(&args.filename).unwrap()).lines() {
        let point = Point::parse(line.unwrap());
        println!("{point:?}");
    }
}

#[derive(Parser)]
struct Args {
    #[arg(short, default_value = "example.txt")]
    filename: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point(u32, u32, u32);
impl Point {
    fn parse(line: String) -> Self {
        let mut iter = line.split(',');
        let x = iter.next().unwrap().parse::<u32>().unwrap();
        let y = iter.next().unwrap().parse::<u32>().unwrap();
        let z = iter.next().unwrap().parse::<u32>().unwrap();
        Self(x, y, z)
    }

    /// Each cube is comprised of 6 squares.
    ///
    /// (1,1,1) has the following squares
    ///
    /// (0,1,1) x (1,0,1) - front
    /// (0,1,0) x (1,0,0) - back
    /// (0,1,0) x (1,1,1) - top
    /// (0,0,0) x (1,0,1) - bottom
    /// (0,0,0) x (0,1,1) - left
    /// (1,0,0) x (1,1,1) - right
    fn squares(&self) -> [Square; 6] {
        [
            // front
            Square(
                Point(self.0 - 1, self.1, self.2),
                Point(self.0, self.1 - 1, self.2),
            ),
            // back
            Square(
                Point(self.0 - 1, self.1, self.2 - 1),
                Point(self.0, self.1 - 1, self.2 - 1),
            ),
            // top
            Square(
                Point(self.0 - 1, self.1, self.2 - 1),
                Point(self.0, self.1, self.2),
            ),
            // bottom
            Square(
                Point(self.0 - 1, self.1 - 1, self.2 - 1),
                Point(self.0, self.1 - 1, self.2),
            ),
            // left
            Square(
                Point(self.0 - 1, self.1 - 1, self.2 - 1),
                Point(self.0 - 1, self.1, self.2),
            ),
            // right
            Square(
                Point(self.0, self.1 - 1, self.2 - 1),
                Point(self.0, self.1, self.2),
            ),
        ]
    }
}

#[test]
fn test_point_squares() {
    let cube = Point(1, 1, 1);
    let squares = cube.squares();
    assert_eq!(
        squares,
        [
            // front
            Square(Point(0, 1, 1), Point(1, 0, 1)),
            // back
            Square(Point(0, 1, 0), Point(1, 0, 0)),
            // top
            Square(Point(0, 1, 0), Point(1, 1, 1)),
            // bottom
            Square(Point(0, 0, 0), Point(1, 0, 1)),
            // left
            Square(Point(0, 0, 0), Point(0, 1, 1)),
            // right
            Square(Point(1, 0, 0), Point(1, 1, 1)),
        ]
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Square(Point, Point);
