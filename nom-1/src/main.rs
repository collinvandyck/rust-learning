// https://developerlife.com/2023/02/20/guide-to-nom-parsing/
//
use nom::{bytes::complete::take_while_m_n, combinator::map_res, sequence::tuple, IResult};

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Color { red, green, blue }
    }
}

mod helpers {
    use super::*;
    use nom::{bytes::complete::take_while_m_n, combinator::map_res, IResult};

    pub fn parse_str_to_hex_num(input: &str) -> Result<u8, std::num::ParseIntError> {
        u8::from_str_radix(input, 16)
    }

    pub fn match_is_hex_digit(c: char) -> bool {
        c.is_ascii_hexdigit()
    }

    pub fn parse_hex_seg(input: &str) -> IResult<&str, u8> {
        map_res(
            take_while_m_n(2, 2, match_is_hex_digit),
            parse_str_to_hex_num,
        )(input)
    }
}

mod parsers {
    use super::*;
    use nom::{
        bytes::complete::take_while_m_n,
        combinator::map_res,
        error::{FromExternalError, ParseError},
        Parser,
    };
    use std::num::ParseIntError;

    pub fn gen_hex_seg_parser_fn<'input, E>() -> impl Parser<&'input str, u8, E>
    where
        E: FromExternalError<&'input str, ParseIntError> + ParseError<&'input str>,
    {
        map_res(
            take_while_m_n(2, 2, helpers::match_is_hex_digit),
            helpers::parse_str_to_hex_num,
        )
    }
}

fn hex_color_no_alpha(input: &str) -> IResult<&str, Color> {
    let it = (
        helpers::parse_hex_seg,
        parsers::gen_hex_seg_parser_fn(),
        map_res(
            take_while_m_n(2, 2, helpers::match_is_hex_digit),
            helpers::parse_str_to_hex_num,
        ),
    );
    let (input, _) = nom::bytes::complete::tag("#")(input)?;
    let (input, (red, green, blue)) = tuple(it)(input)?;
    Ok((input, Color { red, green, blue }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_color() {
        let mut input = String::new();
        input.push_str("#2F14DF");
        input.push('👍');
        let result = dbg!(hex_color_no_alpha(&input));
        let Ok((remainder, color)) = result else {
            panic!()
        };
        assert_eq!(remainder, "👍");
        assert_eq!(color, Color::new(47, 20, 223));
    }

    #[test]
    fn parse_invalid_color() {
        let result = dbg!(hex_color_no_alpha("👍#2F14DF"));
        assert!(result.is_err());
    }
}
