use crate::{Color, Cubes, Game};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    combinator, multi,
    sequence::{self},
    IResult,
};
use std::{collections::HashMap, str::FromStr};

fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

fn parse_u64(input: &str) -> IResult<&str, u64> {
    combinator::map_res(take_while1(is_digit), |f: &str| f.parse::<u64>())(input)
}

fn parse_color(input: &str) -> IResult<&str, Color> {
    let parser = alt((tag("blue"), tag("green"), tag("red")));
    combinator::map_res(parser, |f: &str| Color::from_str(f))(input)
}

fn parse_num_color(input: &str) -> IResult<&str, (Color, u64)> {
    let parser = sequence::separated_pair(parse_u64, tag(" "), parse_color);
    combinator::map(parser, |(count, color)| (color, count))(input)
}

fn parse_turn(input: &str) -> IResult<&str, Cubes> {
    let parser = multi::separated_list0(tag(", "), parse_num_color);
    combinator::map(parser, |f| Cubes(f.into_iter().collect::<HashMap<_, _>>()))(input)
}

fn parse_turns(input: &str) -> IResult<&str, Vec<Cubes>> {
    multi::separated_list0(tag("; "), parse_turn)(input)
}

pub fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, _) = nom::bytes::complete::tag("Game ")(input)?;
    let (input, id) = parse_u64(input)?;
    let (input, _) = nom::bytes::complete::tag(": ")(input)?;
    let (input, turns) = parse_turns(input)?;
    Ok((input, Game { id, turns }))
}
