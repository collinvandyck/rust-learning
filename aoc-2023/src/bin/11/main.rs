use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    ops::Deref,
};
use tracing::{debug, info};

fn main() {
    tracing_subscriber::fmt().init();
    let ex1 = include_str!("ex1.txt");
    let in1 = include_str!("in1.txt");
    info!("p1ex1: {}", sum_of_shortest_paths(ex1, 1));
    info!("p1in1: {}", sum_of_shortest_paths(in1, 1));
}

fn sum_of_shortest_paths(input: &str, expansion_amt: usize) -> usize {
    let mut map = Map::parse(input);
    map.expand(expansion_amt);
    map.galaxy_pairs()
        .into_iter()
        .map(|(t1, t2)| map.shortest_path(&t1, &t2))
        .sum()
}

#[derive(Debug, Clone)]
struct Map {
    tile_vec: Vec<Vec<Tile>>,
    galaxies: HashSet<Point>,
    exp_ys: HashMap<usize, usize>, // y -> amt
    exp_xs: HashMap<usize, usize>, // x -> amt
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn from(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl Map {
    fn parse(input: &str) -> Self {
        let galaxies = input
            .trim()
            .lines()
            .enumerate()
            .flat_map(|(y, row)| {
                row.chars()
                    .enumerate()
                    .filter(|(_x, ch)| ch == &'#')
                    .map(move |(x, _)| Point::from(x, y))
            })
            .collect();
        Self {
            galaxies,
            exp_ys: HashMap::default(),
            exp_xs: HashMap::default(),
            tile_vec: input
                .trim()
                .lines()
                .enumerate()
                .map(|(y, row)| {
                    row.chars()
                        .enumerate()
                        .map(|(x, ch)| Tile::new(x, y, Glyph::from(ch)))
                        .collect()
                })
                .collect(),
        }
    }

    fn shortest_path(&self, src: &Tile, dst: &Tile) -> usize {
        info!("Shortest path from {src} to {dst}");
        assert!(src.is_galaxy());
        assert!(dst.is_galaxy());
        let (ymin, ymax) = (src.y.min(dst.y), src.y.max(dst.y));
        let (xmin, xmax) = (src.x.min(dst.x), src.x.max(dst.x));
        let mut dist = (ymax - ymin) + (xmax - xmin);
        info!("Non expanded distance: {dist}");
        dist = dist
            + (ymin..=ymax)
                .filter_map(|y| match self.exp_ys.get(&y).copied() {
                    Some(amt) => Some(amt),
                    None => None,
                })
                .sum::<usize>();
        dist = dist
            + (xmin..=xmax)
                .filter_map(|x| match self.exp_xs.get(&x).copied() {
                    Some(amt) => Some(amt),
                    None => None,
                })
                .sum::<usize>();
        dist
    }

    fn expand(&mut self, amt: usize) {
        (0..self.num_rows())
            .into_iter()
            .filter(|y| self.row_iter(*y).all(|t| t.is_space()))
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|y| {
                debug!("Expanded y at {y} for {amt}");
                self.exp_ys.insert(y, amt);
            });
        (0..self.num_cols())
            .into_iter()
            .filter(|x| self.col_iter(*x).all(|t| t.is_space()))
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|x| {
                debug!("Expanded x at {x} for {amt}");
                self.exp_xs.insert(x, amt);
            });
    }

    fn galaxy_pairs(&self) -> Vec<(Tile, Tile)> {
        self.galaxy_iter()
            .combinations(2)
            .map(|v| (v[0], v[1]))
            .collect()
    }
    fn galaxy_iter(&self) -> impl Iterator<Item = Tile> + '_ {
        self.galaxies
            .iter()
            .map(|point| Tile::new(point.x, point.y, Glyph::Galaxy))
    }
    fn row_iter(&self, idx: usize) -> impl Iterator<Item = &Tile> {
        self.tile_vec
            .get(idx)
            .map(|s| s.as_slice())
            .unwrap_or(&[])
            .into_iter()
    }
    fn col_iter(&self, idx: usize) -> impl Iterator<Item = &Tile> {
        self.tile_vec.iter().filter_map(move |row| row.get(idx))
    }
    fn num_rows(&self) -> usize {
        self.tile_vec.len()
    }
    fn num_cols(&self) -> usize {
        self.tile_vec.get(0).map(|v| v.len()).unwrap_or_default()
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .tile_vec
            .iter()
            .map(|row| row.iter().map(|t| t.ch()).collect::<String>())
            .join("\n");
        write!(f, "{s}\n")
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Tile {
    glyph: Glyph,
    x: usize,
    y: usize,
}

impl Tile {
    fn new(x: usize, y: usize, glyph: Glyph) -> Self {
        Self { x, y, glyph }
    }
}

impl Deref for Tile {
    type Target = Glyph;
    fn deref(&self) -> &Self::Target {
        &self.glyph
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})[{}]", self.x, self.y, self.glyph.ch())
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, strum_macros::EnumIs)]
enum Glyph {
    Galaxy,
    Space,
}

impl Glyph {
    fn ch(&self) -> char {
        match self {
            Self::Galaxy => '#',
            Self::Space => '.',
        }
    }
}

impl From<char> for Glyph {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Galaxy,
            '.' => Self::Space,
            _ => panic!("unknown char: {value}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use tracing_test::traced_test;

    use super::*;

    #[test]
    #[traced_test]
    fn test_outputs() {
        let ex1 = include_str!("ex1.txt");
        assert_eq!(sum_of_shortest_paths(ex1, 1), 374);
        let in1 = include_str!("in1.txt");
        assert_eq!(sum_of_shortest_paths(in1, 1), 9648398);
    }

    //   2  5  8
    //   E  E  E
    // ...#......
    // .......#..
    // #.........
    // .......... E 3
    // ......#...
    // .#........
    // .........#
    // .......... E 7
    // .......#..
    // #...#.....

    #[test]
    #[traced_test]
    fn test_shortest_path_pt_1() {
        let input = include_str!("ex1.txt");
        let mut map = Map::parse(input);
        map.expand(1);
        for ((srcx, srcy), (dstx, dsty), expected) in [
            ((3, 0), (7, 8), 15), // 1 and 7
            ((0, 2), (9, 6), 17), // 3 and 6
            ((0, 9), (4, 9), 5),  // 8 and 9
        ] {
            let src = Tile::new(srcx, srcy, Glyph::Galaxy);
            let dst = Tile::new(dstx, dsty, Glyph::Galaxy);
            let dist = map.shortest_path(&src, &dst);
            assert_eq!(
                dist, expected,
                "expected {dist} but got {expected} src={src:?} dst={dst:?}"
            );
        }
    }

    #[test]
    fn test_map_pairs() {
        let input = include_str!("ex1.txt");
        let map = Map::parse(input);
        let pairs = map.galaxy_pairs().into_iter().count();
        assert_eq!(pairs, 36);
    }

    #[test]
    fn test_map_serde() {
        let input = include_str!("ex1.txt");
        let map = Map::parse(input);
        let map_str = map.to_string();
        assert_eq!(map_str, input);
    }
}
