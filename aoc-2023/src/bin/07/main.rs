use once_cell::sync::Lazy;
use std::{cmp::Ordering, collections::HashMap};

fn main() {
    let example = include_str!("example.txt");
    let input = include_str!("input.txt");
    println!("p1ex={}", total_winnings(example, Mode::Normal));
    println!("p1in={}", total_winnings(input, Mode::Normal));
    println!("p2ex={}", total_winnings(example, Mode::Jokers));
    println!("p2in={}", total_winnings(input, Mode::Jokers));
}

fn total_winnings(input: &str, mode: Mode) -> u64 {
    let mut bids = parse(input, mode);
    bids.sort();
    bids.iter()
        .zip(1_u64..)
        .map(|(bid, factor)| bid.1 * factor)
        .sum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Mode {
    Normal,
    Jokers,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Card(char);

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
    typ: Type,
    mode: Mode,
}

impl Hand {
    #[cfg(test)]
    fn from(cards: &str, typ: Type, mode: Mode) -> Hand {
        let cards = cards.chars().map(Card).collect();
        Hand { cards, typ, mode }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Bid(Hand, u64);

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Type {
    HighCard(Card),
    OnePair(Card),
    TwoPair(Card, Card),
    ThreeOfKind(Card),
    FullHouse(Card, Card),
    FourOfKind(Card),
    FiveOfKind(Card),
}

impl Type {
    // i had to do this because i decided to include Card in my Type enum
    // because i thought it would be necessary, but that fucks up the derivation of
    // Ord, and PartialOrd
    fn ord(&self) -> usize {
        match self {
            Type::HighCard(_) => 1,
            Type::OnePair(_) => 2,
            Type::TwoPair(_, _) => 3,
            Type::ThreeOfKind(_) => 4,
            Type::FullHouse(_, _) => 5,
            Type::FourOfKind(_) => 6,
            Type::FiveOfKind(_) => 7,
        }
    }
    fn upgrade(self, jokers: usize) -> Type {
        if jokers == 0 {
            return self;
        }
        match self {
            Type::HighCard(c) => match jokers {
                1 => Self::OnePair(c),
                2 => Self::ThreeOfKind(c),
                3 => Self::FourOfKind(c),
                _ => Self::FiveOfKind(c),
            },
            Type::OnePair(c) => match jokers {
                1 => Self::ThreeOfKind(c),
                2 => Self::FourOfKind(c),
                _ => Self::FiveOfKind(c),
            },
            Type::TwoPair(c1, c2) => match jokers {
                1 => Type::FullHouse(c1, c2),
                2 => Type::FourOfKind(c1),
                _ => Type::FiveOfKind(c1),
            },
            Type::ThreeOfKind(c) => match jokers {
                1 => Type::FourOfKind(c),
                _ => Type::FiveOfKind(c),
            },
            Type::FullHouse(c1, _c2) => match jokers {
                1 => Type::FourOfKind(c1),
                _ => Type::FiveOfKind(c1),
            },
            Type::FourOfKind(c) => Type::FiveOfKind(c),
            Type::FiveOfKind(c) => Type::FiveOfKind(c),
        }
    }
}

impl Card {
    fn cmp(&self, other: &Card, mode: Mode) -> Ordering {
        match mode {
            Mode::Normal => CARD_VALUES.get(&self.0).cmp(&CARD_VALUES.get(&other.0)),
            Mode::Jokers => JOKER_VALUES.get(&self.0).cmp(&JOKER_VALUES.get(&other.0)),
        }
    }
}

type CardMap = Lazy<HashMap<char, i32>>;
static CARD_VALUES: CardMap = Lazy::new(|| "23456789TJQKA".chars().zip(1..).collect());
static JOKER_VALUES: CardMap = Lazy::new(|| "J23456789TQKA".chars().zip(1..).collect());

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.typ.ord().cmp(&other.typ.ord()).then_with(|| {
            let mode = self.mode;
            self.cards
                .iter()
                .zip(other.cards.iter())
                .map(|(c1, c2)| c1.cmp(c2, mode))
                .reduce(|a, b| a.then(b))
                .unwrap_or(Ordering::Equal)
        })
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Bid {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for Bid {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<&[Card]> for Type {
    fn from(cards: &[Card]) -> Self {
        let mut hm = HashMap::new();
        for card in cards {
            *hm.entry(card).or_insert(0) += 1;
        }
        let mut counts: Vec<_> = hm.into_iter().map(|(ca, ct)| (*ca, ct)).collect();
        counts.sort_by(|(c1_card, c1_count), (c2_card, c2_count)| {
            c2_count
                .cmp(c1_count)
                .then(c2_card.cmp(c1_card, Mode::Normal)) // TODO: fix this
        });
        use Type::*;
        match counts.as_slice() {
            &[(c, 5), ..] => FiveOfKind(c),
            &[(c, 4), ..] => FourOfKind(c),
            &[(c1, 3), (c2, 2), ..] => FullHouse(c1, c2),
            &[(c, 3), ..] => ThreeOfKind(c),
            &[(c1, 2), (c2, 2), ..] => TwoPair(c1, c2),
            &[(c, 2), ..] => OnePair(c),
            &[(c, _), ..] => HighCard(c),
            _ => unreachable!(),
        }
    }
}

fn parse(input: &str, mode: Mode) -> Vec<Bid> {
    input.lines().map(|l| parse_bid(l, mode)).collect()
}

fn parse_bid(input: &str, mode: Mode) -> Bid {
    let mut parts = input.splitn(2, " ");
    let hand = parse_hand(parts.next().unwrap(), mode);
    let amt = parts.next().unwrap().parse::<u64>().unwrap();
    Bid(hand, amt)
}

fn parse_hand(input: &str, mode: Mode) -> Hand {
    let cards: Vec<_> = input.chars().map(Card).collect();
    let typ = Type::from(cards.as_slice());
    let typ = match mode {
        Mode::Normal => typ,
        Mode::Jokers => typ.upgrade(cards.iter().filter(|c| c.0 == 'J').count()),
    };
    Hand { cards, typ, mode }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_and_sort() {
        use Type::*;

        let example = include_str!("example.txt");
        let mut bids = parse(example, Mode::Normal);
        let mode = Mode::Normal;
        assert_eq!(
            bids,
            vec![
                Bid(Hand::from("32T3K", OnePair(Card('3')), mode), 765),
                Bid(Hand::from("T55J5", ThreeOfKind(Card('5')), mode), 684),
                Bid(Hand::from("KK677", TwoPair(Card('K'), Card('7')), mode), 28),
                Bid(
                    Hand::from("KTJJT", TwoPair(Card('J'), Card('T')), mode),
                    220
                ),
                Bid(Hand::from("QQQJA", ThreeOfKind(Card('Q')), mode), 483),
            ]
        );

        // test sorted bids
        bids.sort();
        assert_eq!(
            bids,
            vec![
                Bid(Hand::from("32T3K", OnePair(Card('3')), mode), 765),
                Bid(
                    Hand::from("KTJJT", TwoPair(Card('J'), Card('T')), mode),
                    220
                ),
                Bid(Hand::from("KK677", TwoPair(Card('K'), Card('7')), mode), 28),
                Bid(Hand::from("T55J5", ThreeOfKind(Card('5')), mode), 684),
                Bid(Hand::from("QQQJA", ThreeOfKind(Card('Q')), mode), 483),
            ]
        );
    }

    #[test]
    fn test_hand_cmp_one() {
        let h1 = parse_hand("33332", Mode::Normal);
        let h2 = parse_hand("2AAAA", Mode::Normal);
        assert_eq!(h1.cards, cards("33332"));
        assert_eq!(h1.typ, Type::FourOfKind(Card('3')));
        assert_eq!(h2.cards, cards("2AAAA"));
        assert_eq!(h2.typ, Type::FourOfKind(Card('A')));
        assert!(h1 > h2);
    }

    fn cards(chs: &str) -> Vec<Card> {
        chs.chars().map(Card).collect()
    }
}
