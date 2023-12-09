use once_cell::sync::Lazy;
use std::{cmp::Ordering, collections::HashMap};

fn main() {
    let example = include_str!("example.txt");
    let input = include_str!("input.txt");
    println!("p1ex={}", total_winnings(example));
    println!("p1in={}", total_winnings(input));
}

fn total_winnings(input: &str) -> u64 {
    let mut bids = parse(input);
    bids.sort();
    bids.iter()
        .zip(1_u64..)
        .map(|(bid, factor)| bid.1 * factor)
        .sum()
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Card(char);

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
    typ: Type,
}

impl Hand {
    #[cfg(test)]
    fn from(cards: &str, typ: Type) -> Hand {
        let cards = cards.chars().map(Card).collect();
        Hand { cards, typ }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Bid(Hand, u64);

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    HighCard(Card),
    OnePair(Card),
    TwoPair(Card, Card),
    ThreeOfKind(Card),
    FullHouse(Card, Card),
    FourOfKind(Card),
    FiveOfKind(Card),
}

type CardMap = Lazy<HashMap<char, i32>>;
static CARD_VALUES: CardMap = Lazy::new(|| "23456789TJQKA".chars().zip(1..).collect());
impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        CARD_VALUES.get(&self.0).cmp(&CARD_VALUES.get(&other.0))
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        println!("cmp for hand");
        self.typ.cmp(&other.typ).then_with(|| {
            println!("comparing cards");
            self.cards.cmp(&other.cards)
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
            c2_count.cmp(c1_count).then(c2_card.cmp(c1_card))
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

fn parse(input: &str) -> Vec<Bid> {
    input.lines().map(parse_bid).collect()
}

fn parse_bid(input: &str) -> Bid {
    let mut parts = input.splitn(2, " ");
    let hand = parse_hand(parts.next().unwrap());
    let amt = parts.next().unwrap().parse::<u64>().unwrap();
    Bid(hand, amt)
}

fn parse_hand(input: &str) -> Hand {
    let cards: Vec<_> = input.chars().map(Card).collect();
    let typ = Type::from(cards.as_slice());
    Hand { cards, typ }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_and_sort() {
        use Type::*;

        let example = include_str!("example.txt");
        let mut bids = parse(example);
        assert_eq!(
            bids,
            vec![
                Bid(Hand::from("32T3K", OnePair(Card('3'))), 765),
                Bid(Hand::from("T55J5", ThreeOfKind(Card('5'))), 684),
                Bid(Hand::from("KK677", TwoPair(Card('K'), Card('7'))), 28),
                Bid(Hand::from("KTJJT", TwoPair(Card('J'), Card('T'))), 220),
                Bid(Hand::from("QQQJA", ThreeOfKind(Card('Q'))), 483),
            ]
        );

        // test sorted bids
        bids.sort();
        assert_eq!(
            bids,
            vec![
                Bid(Hand::from("32T3K", OnePair(Card('3'))), 765),
                Bid(Hand::from("KTJJT", TwoPair(Card('J'), Card('T'))), 220),
                Bid(Hand::from("KK677", TwoPair(Card('K'), Card('7'))), 28),
                Bid(Hand::from("T55J5", ThreeOfKind(Card('5'))), 684),
                Bid(Hand::from("QQQJA", ThreeOfKind(Card('Q'))), 483),
            ]
        );
    }

    #[test]
    fn test_hand_cmp_one() {
        let h1 = parse_hand("33332");
        let h2 = parse_hand("2AAAA");
        assert_eq!(h1.cards, cards("33332"));
        assert_eq!(h1.typ, Type::FourOfKind(Card('3')));
        assert_eq!(h2.cards, cards("2AAAA"));
        assert_eq!(h2.typ, Type::FourOfKind(Card('A')));
        assert!(h1 > h2);
    }

    /*
    #[test]
    fn test_hand_cmp() {
        assert!(parse_hand("33332") > parse_hand("2AAAA"));
        assert!(parse_hand("77888") > parse_hand("77788"));
    }
    */

    #[test]
    fn test_type_ord() {
        use Type::*;
        assert_eq!(OnePair(Card('3')), OnePair(Card('3')));
        assert!(OnePair(Card('4')) > OnePair(Card('3')));
        assert!(ThreeOfKind(Card('T')) > OnePair(Card('T')));
        assert!(ThreeOfKind(Card('T')) < ThreeOfKind(Card('A')));
        assert!(ThreeOfKind(Card('A')) == ThreeOfKind(Card('A')));
    }

    fn cards(chs: &str) -> Vec<Card> {
        chs.chars().map(Card).collect()
    }
}
