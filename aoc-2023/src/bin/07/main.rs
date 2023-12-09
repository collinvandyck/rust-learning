fn main() {
    let example = include_str!("example.txt");
    let bids = parse(example);
    println!("Got {} bids", bids.len());
}

struct Card(char);
struct Hand(Vec<Card>, Type);
struct Bid(Hand, u64);
#[repr(u8)]
enum Type {
    HighCard(Card) = 1,
    OnePair(Card) = 2,
    TwoPair(Card, Card) = 3,
    ThreeOfKind(Card) = 4,
    FullHouse(Card, Card) = 5,
    FourOfKind(Card) = 6,
    FiveOfKind(Card) = 7,
}

impl From<&[Card]> for Type {
    fn from(_cards: &[Card]) -> Self {
        todo!()
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
    Hand(cards, typ)
}
