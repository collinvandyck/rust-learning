#![allow(unused, dead_code)]

fn main() {}

struct Map(Vec<Vec<Tile>>);

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

fn parse(input: &str) -> Map {
    Map(input.lines().map(parse_row).collect())
}

fn parse_row(input: &str) -> Vec<Tile> {
    input
        .chars()
        .map(|ch| match ch {
            '.' => Tile::Ground,
            '|' => Tile::VPipe,
            '-' => Tile::HPipe,
            'L' => Tile::BendNE,
            'J' => Tile::BendNW,
            '7' => Tile::BendSW,
            'F' => Tile::BendSE,
            'S' => Tile::Start,
            _ => panic!("unknown tile: {ch}"),
        })
        .collect()
}

#[test]
fn test_parse() {
    let example = include_str!("example.txt");
    let input = include_str!("input.txt");
    parse(example);
    parse(input);
}
