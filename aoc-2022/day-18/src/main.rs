use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use clap::Parser;

fn main() {
    let args = Args::parse();
    let lines = BufReader::new(File::open(args.filename).unwrap())
        .lines()
        .flatten();
    let lines = lines.collect::<Vec<_>>();
    part_one(&lines);
    part_two(&lines);
}

fn part_one(lines: &[String]) {
    let mut lookup: HashMap<Square, usize> = HashMap::new();
    for line in lines.iter() {
        let point = Point::parse(line.to_string());
        for square in point.squares() {
            let entry = lookup.entry(square).or_insert(0);
            *entry += 1;
        }
    }
    let count = lookup.iter().filter(|&(_a, b)| *b == 1).count();
    println!("Part 1 count: {count}");
}

// part 2 needs to also count the surface area but only include the
// area for sides/squares that are exposed to the air. cubes of air
// that are wholly contained by other sides in some way should not
// be counted.
//
// one approach might involve gathering all of the squares that are
// not overlapped by other cube squares (i.e. the output of part one)
// and then determine if that square can reach the air. this is difficult
// because a square could be part of a larger structure, like a tunnel
// with many bends but which ultimately is exposed to the air. this doesn't
// seem that feasible... another representation of the data seems like
// it might be more approachable.
//
// Another approach might be to first detect any blank spaces in the
// structure by taking slices of the structure, and determining if
// there are any gaps. For each of these gaps, then run a pathing
// algorithm to determine if you can escape. for each cub that
// the pathing fails to reach the outside, that cube would be subtracted
// from the total surface area.
//
// I would probably start by first determining the min and max x, y, and z.
// From there start at the max z
fn part_two(lines: &[String]) {
    let mut lookup: HashMap<Square, usize> = HashMap::new();
    for line in lines.iter() {
        let point = Point::parse(line.to_string());
        for square in point.squares() {
            let entry = lookup.entry(square).or_insert(0);
            *entry += 1;
        }
    }
    let count = lookup.iter().filter(|&(_a, b)| *b == 1).count();
    println!("Part 2 count: {count}");
}

#[derive(Parser)]
struct Args {
    #[arg(short, default_value = "example.txt")]
    filename: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(i32, i32, i32);
impl Point {
    fn parse(line: String) -> Self {
        let mut iter = line.split(',');
        let x = iter.next().unwrap().parse::<i32>().unwrap();
        let y = iter.next().unwrap().parse::<i32>().unwrap();
        let z = iter.next().unwrap().parse::<i32>().unwrap();
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Square(Point, Point);
