use crate::{Color, Cubes, Game};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    combinator::{self, map_res},
    multi::separated_list0,
    sequence::separated_pair,
    IResult,
};
use std::{collections::HashMap, str::FromStr};

fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

fn parse_u64(input: &str) -> IResult<&str, u64> {
    let parser = take_while1(is_digit);
    map_res(parser, |f: &str| f.parse::<u64>())(input)
}

fn parse_color(input: &str) -> IResult<&str, Color> {
    let parser = alt((tag("blue"), tag("green"), tag("red")));
    let mut parser = map_res(parser, Color::from_str);
    parser(input)
}

fn parse_num_color(input: &str) -> IResult<&str, (Color, u64)> {
    let parser = separated_pair(parse_u64, tag(" "), parse_color);
    let mut parser = combinator::map(parser, |(ct, cr)| (cr, ct));
    parser(input)
}

fn parse_turn(input: &str) -> IResult<&str, Cubes> {
    let parser = separated_list0(tag(", "), parse_num_color);
    let mut parser = combinator::map(parser, |f| Cubes(f.into_iter().collect::<HashMap<_, _>>()));
    parser(input)
}

fn parse_turns(input: &str) -> IResult<&str, Vec<Cubes>> {
    separated_list0(tag("; "), parse_turn)(input)
}

pub(crate) fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(input)?;
    let (input, id) = parse_u64(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, turns) = parse_turns(input)?;
    Ok((input, Game { id, turns }))
}
