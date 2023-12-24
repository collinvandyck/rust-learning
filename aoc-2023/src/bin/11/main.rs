use itertools::Itertools;
use std::{collections::HashMap, fmt::Display, ops::Deref};
use tracing::info;

fn main() {
    tracing_subscriber::fmt().init();
    let ex1 = include_str!("ex1.txt");
    let in1 = include_str!("in1.txt");
    info!("p1ex1: {}", sum_of_shortest_paths(ex1, 2));
    info!("p1in1: {}", sum_of_shortest_paths(in1, 2));
    info!("p2in1: {}", sum_of_shortest_paths(in1, 1000000));
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
    tiles: Vec<Vec<Tile>>,
    exp_y: HashMap<usize, usize>, // y -> amt
    exp_x: HashMap<usize, usize>, // x -> amt
}

impl Map {
    fn parse(input: &str) -> Self {
        Self {
            exp_y: HashMap::default(),
            exp_x: HashMap::default(),
            tiles: input
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
        let (ymin, ymax) = (src.y.min(dst.y), src.y.max(dst.y));
        let (xmin, xmax) = (src.x.min(dst.x), src.x.max(dst.x));
        let yds = (ymin..ymax)
            .filter_map(|y| self.exp_y.get(&y).copied())
            .collect::<Vec<_>>();
        let xds = (xmin..xmax)
            .filter_map(|x| self.exp_x.get(&x).copied())
            .collect::<Vec<_>>();
        let yd: usize = (ymax - ymin) + yds.iter().sum::<usize>() - yds.len();
        let xd: usize = (xmax - xmin) + xds.iter().sum::<usize>() - xds.len();
        return yd + xd;
    }
    fn expand(&mut self, amt: usize) {
        (0..self.num_rows())
            .into_iter()
            .filter(|y| self.row_iter(*y).all(|t| t.is_space()))
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|y| {
                self.exp_y.insert(y, amt);
            });
        (0..self.num_cols())
            .into_iter()
            .filter(|x| self.col_iter(*x).all(|t| t.is_space()))
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|x| {
                self.exp_x.insert(x, amt);
            });
    }
    fn galaxy_pairs(&self) -> Vec<(Tile, Tile)> {
        self.galaxy_iter()
            .combinations(2)
            .map(|v| (v[0], v[1]))
            .collect()
    }
    fn galaxy_iter(&self) -> impl Iterator<Item = Tile> + '_ {
        self.tiles
            .iter()
            .flat_map(|r| r.into_iter())
            .filter(|t| t.is_galaxy())
            .copied()
    }
    fn row_iter(&self, idx: usize) -> impl Iterator<Item = &Tile> {
        self.tiles
            .get(idx)
            .map(|s| s.as_slice())
            .unwrap_or(&[])
            .into_iter()
    }
    fn col_iter(&self, idx: usize) -> impl Iterator<Item = &Tile> {
        self.tiles.iter().filter_map(move |row| row.get(idx))
    }
    fn num_rows(&self) -> usize {
        self.tiles.len()
    }
    fn num_cols(&self) -> usize {
        self.tiles.get(0).map(|v| v.len()).unwrap_or_default()
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .tiles
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
    use super::*;
    use tracing_test::traced_test;

    #[test]
    #[traced_test]
    fn test_outputs_p1() {
        let ex1 = include_str!("ex1.txt");
        assert_eq!(sum_of_shortest_paths(ex1, 2), 374);
        let in1 = include_str!("in1.txt");
        assert_eq!(sum_of_shortest_paths(in1, 2), 9648398);
    }

    #[test]
    #[traced_test]
    fn test_outputs_p2() {
        let ex1 = include_str!("ex1.txt");
        assert_eq!(sum_of_shortest_paths(ex1, 10), 1030);
        assert_eq!(sum_of_shortest_paths(ex1, 100), 8410);
        let in1 = include_str!("in1.txt");
        assert_eq!(sum_of_shortest_paths(in1, 1000000), 618800410814);
    }

    #[test]
    #[traced_test]
    fn test_shortest_path_pt_1() {
        let input = include_str!("ex1.txt");
        let mut map = Map::parse(input);
        map.expand(2);
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
