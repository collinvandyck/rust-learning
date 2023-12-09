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

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

impl Type {
    fn upgrade(self, jokers: usize) -> Type {
        if jokers == 0 {
            return self;
        }
        match self {
            Type::HighCard => Type::OnePair.upgrade(jokers - 1),
            Type::OnePair => Type::ThreeOfKind.upgrade(jokers - 1),
            Type::TwoPair => Type::FullHouse.upgrade(jokers - 1),
            Type::ThreeOfKind | Type::FullHouse => Type::FourOfKind.upgrade(jokers - 1),
            Type::FourOfKind | Type::FiveOfKind => Type::FiveOfKind,
        }
    }
    fn from(cards: &[Card], mode: Mode) -> Self {
        let mut hm = HashMap::new();
        cards
            .into_iter()
            .filter(|c| mode != Mode::Jokers || c.0 != 'J')
            .for_each(|card| {
                *hm.entry(card).or_insert(0) += 1;
            });
        let mut counts = hm.values().collect::<Vec<_>>();
        counts.sort();
        counts.reverse();
        use Type::*;
        match counts.as_slice() {
            &[5, ..] => FiveOfKind,
            &[4, ..] => FourOfKind,
            &[3, 2, ..] => FullHouse,
            &[3, ..] => ThreeOfKind,
            &[2, 2, ..] => TwoPair,
            &[2, ..] => OnePair,
            _ => HighCard,
        }
    }
}

static VALUES: Lazy<HashMap<Mode, HashMap<char, i32>>> = Lazy::new(|| {
    HashMap::from([
        (Mode::Normal, "23456789TJQKA".chars().zip(1..).collect()),
        (Mode::Jokers, "J23456789TQKA".chars().zip(1..).collect()),
    ])
});

impl Card {
    fn cmp(&self, other: &Card, mode: Mode) -> Ordering {
        let values = VALUES.get(&mode).expect("no value for mode");
        values.get(&self.0).cmp(&values.get(&other.0))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.typ.cmp(&other.typ).then_with(|| {
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
    let typ = Type::from(cards.as_slice(), mode);
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
    fn test_results() {
        let example = include_str!("example.txt");
        let input = include_str!("input.txt");
        assert_eq!(total_winnings(example, Mode::Normal), 6440);
        assert_eq!(total_winnings(input, Mode::Normal), 248836197);
        assert_eq!(total_winnings(example, Mode::Jokers), 5905);
        assert_eq!(total_winnings(input, Mode::Jokers), 251195607);
    }

    #[test]
    fn test_parse_and_sort() {
        use Type::*;

        let example = include_str!("example.txt");
        let mut bids = parse(example, Mode::Normal);
        let mode = Mode::Normal;
        assert_eq!(
            bids,
            vec![
                Bid(Hand::from("32T3K", OnePair, mode), 765),
                Bid(Hand::from("T55J5", ThreeOfKind, mode), 684),
                Bid(Hand::from("KK677", TwoPair, mode), 28),
                Bid(Hand::from("KTJJT", TwoPair, mode), 220),
                Bid(Hand::from("QQQJA", ThreeOfKind, mode), 483),
            ]
        );

        // test sorted bids
        bids.sort();
        assert_eq!(
            bids,
            vec![
                Bid(Hand::from("32T3K", OnePair, mode), 765),
                Bid(Hand::from("KTJJT", TwoPair, mode), 220),
                Bid(Hand::from("KK677", TwoPair, mode), 28),
                Bid(Hand::from("T55J5", ThreeOfKind, mode), 684),
                Bid(Hand::from("QQQJA", ThreeOfKind, mode), 483),
            ]
        );
    }

    #[test]
    fn test_parse_and_sort_p2() {
        use Type::*;

        let mode = Mode::Jokers;
        let example = include_str!("example.txt");
        let mut bids = parse(example, mode);
        assert_eq!(
            bids,
            vec![
                Bid(Hand::from("32T3K", OnePair, mode), 765),
                Bid(Hand::from("T55J5", FourOfKind, mode), 684),
                Bid(Hand::from("KK677", TwoPair, mode), 28),
                Bid(Hand::from("KTJJT", FourOfKind, mode), 220),
                Bid(Hand::from("QQQJA", FourOfKind, mode), 483),
            ]
        );

        // test sorted bids
        bids.sort();
        assert_eq!(
            bids,
            vec![
                Bid(Hand::from("32T3K", OnePair, mode), 765),
                Bid(Hand::from("KK677", TwoPair, mode), 28),
                Bid(Hand::from("T55J5", FourOfKind, mode), 684),
                Bid(Hand::from("QQQJA", FourOfKind, mode), 483),
                Bid(Hand::from("KTJJT", FourOfKind, mode), 220),
            ]
        );
    }

    #[test]
    fn test_hand_cmp_one() {
        let h1 = parse_hand("33332", Mode::Normal);
        let h2 = parse_hand("2AAAA", Mode::Normal);
        assert_eq!(h1.cards, cards("33332"));
        assert_eq!(h1.typ, Type::FourOfKind);
        assert_eq!(h2.cards, cards("2AAAA"));
        assert_eq!(h2.typ, Type::FourOfKind);
        assert!(h1 > h2);
    }

    #[test]
    fn test_joker_ord() {
        let h1 = parse_hand("JKKK2", Mode::Jokers);
        assert!(h1.typ == Type::FourOfKind);
        let h2 = parse_hand("QQQQ2", Mode::Jokers);
        assert!(h2.typ == Type::FourOfKind);
        assert!(h1 < h2);
        let h3 = parse_hand("K1JJ2", Mode::Jokers);
        assert_eq!(h3.typ, Type::ThreeOfKind);
    }

    fn cards(chs: &str) -> Vec<Card> {
        chs.chars().map(Card).collect()
    }
}
