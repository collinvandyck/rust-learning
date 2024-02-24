#![allow(dead_code, unused)]
use anyhow::bail;
use nom::{
    bytes::complete::{tag, take_while1, take_while_m_n},
    character::complete::space1,
    combinator::map_res,
    sequence::{delimited, tuple},
    IResult,
};
use std::num::ParseIntError;

fn main() {
    let ex1 = include_str!("ex1.txt");
    let plan = Plan::parse(ex1);
    println!("plan: {plan:?}");
}

#[derive(Debug)]
struct Plan(Vec<Step>);

impl Plan {
    fn parse(input: &str) -> Self {
        Self(
            input
                .trim()
                .lines()
                .map(|l| Step::parse(l).unwrap().1)
                .collect(),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Debug)]
struct Step {
    dir: Dir,
    amount: usize,
    color: Color,
}

impl Step {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, dir) = map_res(
            take_while1(|ch: char| -> bool { !ch.is_whitespace() }),
            Self::dir_from,
        )(input)?;
        let (input, _) = space1(input)?;
        let (input, amount) = map_res(
            take_while1(|ch: char| -> bool { ch.is_digit(10) }),
            Self::amount_from,
        )(input)?;
        let (input, _) = space1(input)?;
        let (input, color) = delimited(tag("("), Self::hex_color, tag(")"))(input)?;
        Ok((input, Self { dir, amount, color }))
    }

    fn hex_color(input: &str) -> IResult<&str, Color> {
        let (input, _) = tag("#")(input)?;
        let (input, (r, g, b)) =
            tuple((Self::hex_primary, Self::hex_primary, Self::hex_primary))(input)?;
        Ok((input, Color { r, g, b }))
    }

    fn hex_primary(input: &str) -> IResult<&str, u8> {
        map_res(
            take_while_m_n(2, 2, |c: char| -> bool { c.is_digit(16) }),
            Self::from_hex,
        )(input)
    }

    fn dir_from(input: &str) -> anyhow::Result<Dir> {
        Ok(match input {
            "R" => Dir::Right,
            "L" => Dir::Left,
            "U" => Dir::Up,
            "D" => Dir::Down,
            _ => bail!("unknown dir: {input}"),
        })
    }

    fn amount_from(input: &str) -> Result<usize, ParseIntError> {
        input.parse::<usize>()
    }

    fn from_hex(input: &str) -> Result<u8, ParseIntError> {
        u8::from_str_radix(input, 16)
    }
}
