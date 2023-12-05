use nom::{
    bytes::complete::{tag, take_while1},
    character::{
        self,
        complete::{line_ending, space0, space1},
    },
    combinator::map_res,
    multi::separated_list0,
    sequence::tuple,
    IResult,
};

fn main() {
    let example = include_str!("example.txt");
    let input = include_str!("input.txt");
    println!("p1ex={}", scratchcard_points(example));
    println!("p1in={}", scratchcard_points(input));
    println!("p2ex={}", scratchcard_burnout(example));
    println!("p2in={}", scratchcard_burnout(input));
}

fn scratchcard_points(input: &str) -> u64 {
    parse(input).iter().map(|c| c.points()).sum()
}

fn scratchcard_burnout(input: &str) -> u64 {
    let mut cards = parse(input);
    let cards = cards.as_mut_slice();
    for i in 0..cards.len() {
        let matching = cards[i].matching();
        for j in (i + 1)..=(i + matching) {
            cards[j].count += cards[i].count;
        }
    }
    cards.iter().map(|c| c.count).sum()
}

type Number = u64;
#[derive(Debug, Clone)]
struct Card {
    count: u64,
    winning: Vec<Number>,
    ours: Vec<Number>,
}

impl Card {
    fn matching(&self) -> usize {
        self.winning
            .iter()
            .filter(|n| self.ours.contains(n))
            .count()
    }
    fn points(&self) -> u64 {
        let count: u64 = self
            .winning
            .iter()
            .filter(|n| self.ours.contains(*n))
            .count()
            .try_into()
            .unwrap();
        match count {
            0 => 0,
            n => (1..n).fold(1_u64, |a, _| a * 2),
        }
    }
}

fn parse(input: &str) -> Vec<Card> {
    let (input, cards) = parse_cards(input).unwrap();
    assert_eq!(input, "");
    cards
}

fn parse_cards(input: &str) -> IResult<&str, Vec<Card>> {
    separated_list0(line_ending, parse_card)(input.trim())
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    let (input, _) = tag("Card")(input)?;
    let (input, _) = space1(input)?;
    let (input, _id) = parse_u64(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = space1(input)?;
    let (input, (winning, _, ours)) = tuple((parse_nums, tag("|"), parse_nums))(input)?;
    let count = 1;
    let card = Card {
        count,
        winning,
        ours,
    };
    Ok((input, card))
}

fn parse_nums(input: &str) -> IResult<&str, Vec<Number>> {
    let (input, _) = space0(input)?;
    let ws = character::complete::space1;
    let list = separated_list0(ws, parse_u64);
    let (input, (list, _)) = tuple((list, space0))(input)?;
    Ok((input, list))
}

fn parse_u64(input: &str) -> IResult<&str, u64> {
    let parser = take_while1(|c: char| c.is_digit(10));
    map_res(parser, |f: &str| f.parse::<u64>())(input)
}
