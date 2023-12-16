#![allow(unused, dead_code)]

fn main() {}

struct Map {
    start: Pt,
    tiles: Vec<Vec<Tile>>,
}

impl Map {
    fn new(tiles: Vec<Vec<Tile>>) -> Self {
        let start = Self::find_tile(&tiles, Tile::Start)
            .first()
            .copied()
            .unwrap();
        Self { tiles, start }
    }
    fn swap_start(&mut self) {
        let pt = self.find(Tile::Start).first().unwrap();
        todo!()
    }
    fn get(&self, pt: Pt) -> Option<&Tile> {
        self.tiles.get(pt.1).and_then(|r| r.get(pt.0))
    }
    fn find(&self, tile: Tile) -> Vec<Pt> {
        Self::find_tile(&self.tiles, tile)
    }
    fn find_tile(tiles: &[Vec<Tile>], tile: Tile) -> Vec<Pt> {
        tiles
            .iter()
            .enumerate()
            .map(|(row, tiles)| {
                tiles
                    .iter()
                    .enumerate()
                    .filter(|(col, t)| t == &&tile)
                    .map(|(col, _)| (row, col))
                    .collect::<Vec<_>>()
            })
            .flatten()
            .map(|(row, col)| Pt(row, col))
            .collect::<Vec<_>>()
    }
    fn rows(&self) -> usize {
        self.tiles.len()
    }
    fn cols(&self) -> usize {
        self.tiles.get(0).map(|l| l.len()).unwrap_or_default()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pt(usize, usize); // x,y

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    VPipe,
    HPipe,
    BendNE,
    BendNW,
    BendSW,
    BendSE,
    Ground,
    Start,
}

impl From<char> for Tile {
    fn from(ch: char) -> Self {
        match ch {
            '.' => Tile::Ground,
            '|' => Tile::VPipe,
            '-' => Tile::HPipe,
            'L' => Tile::BendNE,
            'J' => Tile::BendNW,
            '7' => Tile::BendSW,
            'F' => Tile::BendSE,
            'S' => Tile::Start,
            _ => panic!("unknown tile: {ch}"),
        }
    }
}

fn parse(input: &str) -> Map {
    Map::new(input.lines().map(parse_row).collect())
}

fn parse_row(input: &str) -> Vec<Tile> {
    input.chars().map(Tile::from).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_map() {
        let example = include_str!("example.txt");
        let map = parse(example);
        let starts = map.find(Tile::Start);
        assert_eq!(starts, vec![Pt(2, 0)]);
    }

    #[test]
    fn test_parse() {
        let example = include_str!("example.txt");
        let input = include_str!("input.txt");
        let map = parse(example);
        assert_eq!(map.rows(), 5);
        assert_eq!(map.cols(), 5);
        parse(input);
    }
}
