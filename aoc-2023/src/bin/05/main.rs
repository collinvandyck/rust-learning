#![allow(unused, dead_code)]
use nom::character::complete::{line_ending, space1};
use nom::multi::{many1, separated_list1};
use nom::sequence::tuple;
use nom::{
    bytes::{
        self,
        complete::{tag, take_while1},
    },
    character::{self, complete},
    combinator::map,
    combinator::map_res,
    multi::separated_list0,
    IResult,
};
use std::ops;

fn main() {
    let example = include_str!("example.txt");
    let input = include_str!("input.txt");
    println!("p1ex={}", lowest_location(example, SeedMode::Literal));
    println!("p1in={}", lowest_location(input, SeedMode::Literal));
}

enum SeedMode {
    Literal,
    Range,
}

fn lowest_location(input: &str, seed_mode: SeedMode) -> Id {
    let almanac = parse(input);
    almanac
        .seeds
        .iter()
        .copied()
        .map(|id| almanac.lookup(id, "seed", "location"))
        .min()
        .unwrap()
}

type Resource = String;
type Id = u64;

struct IdType(Id, Resource);

struct Almanac {
    seeds: Vec<Id>,
    mappings: Vec<Mapping>,
}

impl Almanac {
    fn lookup(&self, mut src_id: Id, src_typ: impl AsRef<str>, fd_typ: impl AsRef<str>) -> Id {
        let fd_typ = fd_typ.as_ref().to_string();
        let mut src_typ = src_typ.as_ref().to_string();
        loop {
            let (dst_typ, dst_id) = self
                .mappings
                .iter()
                .filter(|m| m.src.resource == src_typ)
                .flat_map(|m| m.dst_id_for_src(src_id, &src_typ))
                .next()
                .unwrap_or_else(|| {
                    let dst_typ = self
                        .mappings
                        .iter()
                        .find(|m| m.src.resource == src_typ)
                        .map(|m| m.dst.resource.clone())
                        .unwrap();
                    (dst_typ, src_id)
                });
            if dst_typ == fd_typ {
                // we're done
                return dst_id;
            }
            src_typ = dst_typ;
            src_id = dst_id;
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Mapping {
    src: ResourceRange,
    dst: ResourceRange,
}

impl Mapping {
    // for a given source id and type, what is the destination that it maps to. if there is no
    // mapping that fits, None will be returned.
    fn dst_id_for_src(&self, src_id: Id, src_typ: &Resource) -> Option<(Resource, Id)> {
        if &self.src.resource == src_typ {
            if self.src.ids.contains(&src_id) {
                let distance = src_id - self.src.ids.start;
                let dst_id = self.dst.ids.start + distance;
                return Some((self.dst.resource.clone(), dst_id));
            }
        }
        None
    }
}

type IdRange = ops::Range<Id>;

#[derive(Debug, PartialEq, Eq)]
struct ResourceRange {
    resource: Resource,
    ids: IdRange,
}

// ----------- parsing ---------------------------------------------------------------

fn parse(input: &str) -> Almanac {
    let (input, almanac) = parse_almanac(input).unwrap();
    assert_eq!(input, "");
    almanac
}

fn parse_almanac(input: &str) -> IResult<&str, Almanac> {
    let (input, _) = tag("seeds: ")(input)?;
    let (input, seeds) = parse_ids(input)?;
    let (input, _) = line_ending(input)?;
    let (input, _) = line_ending(input)?;
    let (input, mappings) = parse_mappings(input)?;
    Ok((input, Almanac { seeds, mappings }))
}

fn parse_mappings(input: &str) -> IResult<&str, Vec<Mapping>> {
    let (input, maps) = separated_list0(complete::line_ending, parse_mapping)(input)?;
    let maps = maps.into_iter().flatten().collect::<Vec<_>>();
    Ok((input, maps))
}

fn parse_mapping(input: &str) -> IResult<&str, Vec<Mapping>> {
    let (input, (src_resource, _, dst_resource, _, _, _)) = tuple((
        parse_resource,
        tag("-to-"),
        parse_resource,
        space1,
        tag("map:"),
        line_ending,
    ))(input)?;
    let (input, ranges) = parse_ranges(input)?;
    let (input, _) = nom::combinator::opt(line_ending)(input)?;
    let ranges = ranges
        .into_iter()
        .map(|(dst, src, amt)| Mapping {
            src: ResourceRange {
                resource: src_resource.clone(),
                ids: src..(src + amt),
            },
            dst: ResourceRange {
                resource: dst_resource.clone(),
                ids: dst..(dst + amt),
            },
        })
        .collect::<Vec<_>>();
    Ok((input, ranges))
}

fn parse_ranges(input: &str) -> IResult<&str, Vec<(Id, Id, Id)>> {
    separated_list1(line_ending, parse_range)(input)
}

fn parse_range(input: &str) -> IResult<&str, (Id, Id, Id)> {
    let mut parser = tuple((parse_id, space1, parse_id, space1, parse_id));
    let (input, (dst, _, src, _, amt)) = parser(input)?;
    Ok((input, (dst, src, amt)))
}

fn parse_resource(input: &str) -> IResult<&str, Resource> {
    let parser = take_while1(|c: char| c.is_alphabetic());
    map(parser, |f: &str| f.to_string())(input)
}

fn parse_ids(input: &str) -> IResult<&str, Vec<Id>> {
    separated_list0(complete::space1, parse_id)(input)
}

fn parse_id(input: &str) -> IResult<&str, Id> {
    let parser = take_while1(|c: char| c.is_digit(10));
    map_res(parser, |f: &str| f.parse::<u64>())(input)
}

// ----------- tests -----------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_example() {
        let ex = include_str!("example.txt");
        let almanac = parse(&ex);
        assert_eq!(almanac.seeds, vec![79, 14, 55, 13]);
        assert_eq!(almanac.mappings.len(), 18);
        assert_eq!(
            almanac.mappings.get(0),
            Some(&Mapping {
                src: crate::ResourceRange {
                    resource: String::from("seed"),
                    ids: (98..100),
                },
                dst: crate::ResourceRange {
                    resource: String::from("soil"),
                    ids: (50..52),
                },
            })
        );
        assert_eq!(
            almanac.mappings.get(1),
            Some(&Mapping {
                src: crate::ResourceRange {
                    resource: String::from("seed"),
                    ids: (50..98),
                },
                dst: crate::ResourceRange {
                    resource: String::from("soil"),
                    ids: (52..100),
                },
            })
        );
    }

    #[test]
    fn test_results() {
        let example = include_str!("example.txt");
        let input = include_str!("input.txt");
        assert_eq!(lowest_location(example, SeedMode::Literal), 35);
        assert_eq!(lowest_location(input, SeedMode::Literal), 240320250);
    }
}
