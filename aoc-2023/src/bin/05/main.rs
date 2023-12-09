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
use std::collections::HashMap;
use std::ops;

fn main() {
    let example = include_str!("example.txt");
    let input = include_str!("input.txt");
    println!("p1ex={}", lowest_location(example, SeedMode::Literal));
    //println!("p1in={}", lowest_location(input, SeedMode::Literal));
}

enum SeedMode {
    Literal,
    Range,
}

fn lowest_location(input: &str, seed_mode: SeedMode) -> Id {
    let almanac = parse(input);

    // disregard seed mode for now.
    let seeds = almanac
        .seeds
        .iter()
        .copied()
        .map(|id| TypedRange {
            resource: resource("seed"),
            range: id..id + 1,
        })
        .collect::<Vec<_>>();
    let dst_resource = resource("location");
    seeds
        .into_iter()
        .map(|range| almanac.lowest_for_range(range, &dst_resource))
        .min()
        .expect("no min found!")

    /*
    almanac
        .seeds
        .iter()
        .copied()
        .map(|id| almanac.lookup(id, "seed", "location"))
        .min()
        .unwrap()
        */
}

type Resource = String;

fn resource(s: impl AsRef<str>) -> Resource {
    s.as_ref().to_string()
}

type Id = u64;
type IdRange = ops::Range<Id>;

trait RangeExt
where
    Self: Sized,
{
    fn intersect(&self, other: &Self) -> Option<Self>;
    fn before_after(&self, other: &Self) -> (Option<Self>, Option<Self>);
}

struct IdType(Id, Resource);
#[derive(Debug, Clone, PartialEq, Eq)]
struct TypedRange {
    resource: Resource,
    range: IdRange,
}

fn typed_range(r: impl AsRef<str>, range: IdRange) -> TypedRange {
    TypedRange {
        resource: r.as_ref().to_string(),
        range,
    }
}

struct Almanac {
    seeds: Vec<Id>,
    nexts: HashMap<Resource, Resource>,
    ranges: Vec<ResourceRanges>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ResourceRanges {
    src: ResourceRange,
    dst: ResourceRange,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ResourceRange {
    resource: Resource,
    range: IdRange,
}

impl Almanac {
    fn lookup(&self, mut src_id: Id, src_typ: impl AsRef<str>, fd_typ: impl AsRef<str>) -> Id {
        let fd_typ = fd_typ.as_ref().to_string();
        let mut src_typ = src_typ.as_ref().to_string();
        loop {
            let (dst_typ, dst_id) = self
                .ranges
                .iter()
                .filter(|m| m.src.resource == src_typ)
                .flat_map(|m| m.dst_id_for_src(src_id, &src_typ))
                .next()
                .unwrap_or_else(|| {
                    let dst_typ = self
                        .ranges
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

    fn lowest_for_range(&self, range: TypedRange, dst_res: &Resource) -> Id {
        println!("lowest for range={range:?} dst_res={dst_res}");
        let mut lowest: Option<Id> = None;
        let mut queue = vec![range];
        while let Some(range) = queue.pop() {
            println!("\nPopped range {range:?}");
            self.ranges
                .iter()
                .flat_map(|rrs| rrs.dest_ranges_for(&range))
                .for_each(|tr| {
                    println!("  <== {tr:?}");
                    if &tr.resource == dst_res {
                        lowest = Some(match lowest {
                            None => tr.range.start,
                            Some(lowest) if lowest < tr.range.start => lowest,
                            Some(value) => tr.range.start,
                        });
                    } else {
                        queue.push(tr);
                    }
                });
        }
        lowest.expect("no lowest found!")
    }
}

// a collection of dst and src ranges
impl ResourceRanges {
    // for a given source id and type, what is the destination that it maps to. if there is no
    // mapping that fits, None will be returned.
    fn dst_id_for_src(&self, src_id: Id, src_typ: &Resource) -> Option<(Resource, Id)> {
        if &self.src.resource == src_typ {
            if self.src.range.contains(&src_id) {
                let distance = src_id - self.src.range.start;
                let dst_id = self.dst.range.start + distance;
                return Some((self.dst.resource.clone(), dst_id));
            }
        }
        None
    }

    // for the given typed range, return the overlap with the src ranges as ranged types for the
    // destination. If the source type is wrong, or the ranges do not overlap, None will be
    // returned. The return value is the source range along with the dest range.
    fn intersection(&self, t: &TypedRange) -> Option<TypedRange> {
        if self.src.resource == t.resource {
            let dst = self.dst.resource.clone();
            if let Some(range) = self.src.range.intersect(&t.range) {
                let offset = range.start - self.src.range.start;
                let distance = range.end - range.start;
                let start = self.dst.range.start + offset;
                let end = start + distance;
                let range = start..(start + distance);
                return Some(TypedRange {
                    resource: self.dst.resource.clone(),
                    range,
                });
            }
        }
        None
    }

    // like intersection but also maps unmatched regions to bookended typed ranges
    // must return empty vec for no matches
    fn dest_ranges_for(&self, t: &TypedRange) -> Vec<TypedRange> {
        let mut res = vec![];
        if self.src.resource == t.resource {
            println!("  dest ranges for {t:?}");
            let dst = self.dst.resource.clone();
            println!(
                "  ?range({}) {:?} intersect {:?}",
                self.src.resource, self.src.range, t.range
            );
            if let Some(range) = self.src.range.intersect(&t.range) {
                let (before, after) = self.src.range.before_after(&range);
                let offset = range.start - self.src.range.start;
                let distance = range.end - range.start;
                let start = self.dst.range.start + offset;
                let end = start + distance;
                let range = start..(start + distance);
                let next = TypedRange {
                    resource: self.dst.resource.clone(),
                    range,
                };
                if let Some(range) = before {
                    let next = TypedRange {
                        resource: self.dst.resource.clone(),
                        range,
                    };
                    res.push(next);
                }
                res.push(next);

                if let Some(range) = after {
                    let next = TypedRange {
                        resource: self.dst.resource.clone(),
                        range,
                    };
                    res.push(next);
                }

                // TODO: push the bookends
            } else {
                // TODO: no intersection so just send the same ranges as self
                // but with a dest type
            }
        }
        res
    }
}

impl RangeExt for ops::Range<Id> {
    fn intersect(&self, other: &Self) -> Option<Self> {
        println!("    intersect {self:?} x {other:?}");
        let start = self.start.max(other.start);
        let end = self.end.min(other.end);
        if end > start {
            let res = Some(start..end);
            println!("    {res:?}");
            res
        } else {
            println!("    None");
            None
        }
    }
    fn before_after(&self, other: &Self) -> (Option<Self>, Option<Self>) {
        let before = if self.start < other.start {
            Some(self.start..other.start)
        } else {
            None
        };
        let after = if self.end > other.end {
            Some(other.end..self.end)
        } else {
            None
        };
        (before, after)
    }
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
    let nexts = mappings
        .iter()
        .cloned()
        .map(|m| (m.src.resource, m.dst.resource))
        .collect::<HashMap<_, _>>();
    Ok((
        input,
        Almanac {
            seeds,
            nexts,
            ranges: mappings,
        },
    ))
}

fn parse_mappings(input: &str) -> IResult<&str, Vec<ResourceRanges>> {
    let (input, maps) = separated_list0(complete::line_ending, parse_mapping)(input)?;
    let maps = maps.into_iter().flatten().collect::<Vec<_>>();
    Ok((input, maps))
}

fn parse_mapping(input: &str) -> IResult<&str, Vec<ResourceRanges>> {
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
        .map(|(dst, src, amt)| ResourceRanges {
            src: ResourceRange {
                resource: src_resource.clone(),
                range: src..(src + amt),
            },
            dst: ResourceRange {
                resource: dst_resource.clone(),
                range: dst..(dst + amt),
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
    fn test_basics() {
        let ex = include_str!("example.txt");
        let almanac = parse(&ex);
        assert_eq!(almanac.seeds, vec![79, 14, 55, 13]);
        assert_eq!(almanac.ranges.len(), 18);
        assert_eq!(
            almanac.ranges.get(0),
            Some(&ResourceRanges {
                src: crate::ResourceRange {
                    resource: String::from("seed"),
                    range: (98..100),
                },
                dst: crate::ResourceRange {
                    resource: String::from("soil"),
                    range: (50..52),
                },
            })
        );

        // range tests for the first range
        let range = almanac.ranges.get(0).unwrap();
        for (src_range, dst_range) in [
            ((99..100), Some((51..52))),
            ((98..99), Some((50..51))),
            ((98..98), None),
            ((1..2), None),
            ((100..101), None),
        ] {
            assert_eq!(
                range.intersection(&TypedRange {
                    resource: resource("seed"),
                    range: src_range.clone(),
                }),
                dst_range.clone().map(|range| TypedRange {
                    resource: resource("soil"),
                    range
                }),
                "expected {src_range:?} to map to {dst_range:?}"
            );
        }

        assert_eq!(
            almanac.ranges.get(1),
            Some(&ResourceRanges {
                src: crate::ResourceRange {
                    resource: String::from("seed"),
                    range: (50..98),
                },
                dst: crate::ResourceRange {
                    resource: String::from("soil"),
                    range: (52..100),
                },
            })
        );
    }

    #[test]
    fn test_ranges() {
        let r1: IdRange = (1..5);
        let r2: IdRange = (2..8);
        assert_eq!(r1.intersect(&r2), Some(2..5));

        let r1: IdRange = (1..5);
        let r2: IdRange = (7..10);
        assert_eq!(r1.intersect(&r2), None);

        let r1: IdRange = (1..5);
        let r2: IdRange = (5..10);
        assert_eq!(r1.intersect(&r2), None);

        let r1: IdRange = (50..98);
        let r2: IdRange = (79..80);
        assert_eq!(r1.intersect(&r2), Some(79..80));
    }

    #[test]
    fn test_ranges_before_after() {
        let r1: IdRange = (1..10);
        assert_eq!(r1.before_after(&(5..15)), (Some(1..5), None));
        assert_eq!(r1.before_after(&(5..6)), (Some(1..5), Some(6..10)));
        assert_eq!(r1.before_after(&(0..5)), (None, Some(5..10)));
    }
}
