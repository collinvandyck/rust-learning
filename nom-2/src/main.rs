#![allow(dead_code, unused)]
use std::error::Error;

use nom::{
    branch::alt,
    bytes::{
        self,
        complete::{is_not, take_while1},
    },
    character,
    combinator::{self, map_res, opt},
    multi::{self, many0},
    IResult,
};

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq, Eq)]
enum Run<'a> {
    Digits(&'a str),
    Text(&'a str),
}

fn parse_digit(i: &str) -> IResult<&str, Run> {
    let parser = nom::bytes::complete::is_a("0123456789");
    combinator::map(parser, |i: &str| Run::Digits(i))(i)
}

#[test]
fn test_parse_digit() {
    assert_eq!(parse_digit("123abc").unwrap(), ("abc", Run::Digits("123")));
}

fn parse_non_digit(i: &str) -> IResult<&str, Run> {
    let parser = is_not("0123456789");
    combinator::map(parser, |i: &str| Run::Text(i))(i)
}

#[test]
fn test_parse_non_digit() {
    assert_eq!(
        parse_non_digit("foo123abc").unwrap(),
        ("123abc", Run::Text("foo"))
    );
}

fn parse_all(i: &str) -> IResult<&str, Vec<Run>> {
    many0(alt((parse_digit, parse_non_digit)))(i)
}

#[test]
fn test_parse_all() {
    assert_eq!(
        parse_all("foo123abc").unwrap(),
        (
            "",
            vec![Run::Text("foo"), Run::Digits("123"), Run::Text("abc"),]
        )
    );
}
