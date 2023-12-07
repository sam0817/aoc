use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::fs;

fn main() {
    let contents = fs::read_to_string("input")
    // let contents = fs::read_to_string("example")
        .expect("Should have been able to read the file");

    println!("---------- part1 ----------");
    // part 1
    part1(&contents[..]);

    println!("---------- part2 ----------");
    // part 2
    part2(&contents[..]);
    // 248909434
}

fn part1(content: &str) {
    let mut hands = Vec::<Hand>::new();
    let mut sum = 0usize;
    content.lines().for_each(|line| {
        let nums = line.split_whitespace().collect::<Vec<&str>>();
        let bid = nums[1].parse::<usize>().unwrap();
        let ranks = nums[0].chars().map(|c|
            Rank::from(c.to_string().as_str())
        ).collect::<Vec<Rank>>();
        let hand = Hand { ranks, bid, rank: 0 };
        hands.push(hand);
    });
    hands.sort();
    hands.iter_mut().enumerate().for_each(|(idx, hand)| {
        hand.rank = idx + 1;
        sum += hand.bid * hand.rank;
        // println!("{:?} {:#?}-> {} ---> {}", hand, hand.kind(), hand.bid, sum);
    });
    println!("Result: {}", sum);
}

fn part2(content: &str) {
    let mut hands = Vec::<Hand2>::new();
    let mut sum = 0usize;
    content.lines().for_each(|line| {
        let nums = line.split_whitespace().collect::<Vec<&str>>();
        let bid = nums[1].parse::<usize>().unwrap();
        let mut rank2s = nums[0].chars().map(|c|
            Rank2::from(c.to_string().as_str())
        ).collect::<Vec<Rank2>>();
        // rank2s.sort();
        let hand = Hand2 { ranks: rank2s, bid, rank: 0 };
        hands.push(hand);
    });
    hands.sort_by(|a, b| {
        let mut c = a.clone();
        let mut d = b.clone();
        c.ranks.sort();
        d.ranks.sort();
        if c.kind() != d.kind() {
            return c.kind().partial_cmp(&d.kind()).unwrap_or(Ordering::Equal);
        }
        for i in 0..5 {
            if a.ranks[i] != b.ranks[i] {
                return a.ranks[i].partial_cmp(&b.ranks[i]).unwrap_or(Ordering::Equal);
            }
        }
        Ordering::Equal
    });
    hands.iter_mut().enumerate().for_each(|(idx, hand)| {
        hand.rank = idx + 1;
        sum += hand.bid * hand.rank;
        let s = hand.ranks.iter().map(|r| r.to_string()).collect::<Vec<_>>().join("");
        // println!("{} {:#?} {}-> {} ---> {}", s, hand.kind(), hand.rank, hand.bid, sum);
        // let js = hand.ranks.iter().filter(|r| **r == Rank2::Prince).count();
        // if js > sum { sum = js; }
    });
    println!("Result: {}", sum);
}

#[derive(Debug, Clone, Ord, Eq)]
struct Hand {
    ranks: Vec<Rank>,
    bid: usize,
    rank: usize,
}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.kind() == other.kind() && self.ranks == other.ranks
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.kind() == other.kind() {
            for i in 0..5 {
                if self.ranks[i] != other.ranks[i] {
                    return self.ranks[i].partial_cmp(&other.ranks[i]);
                }
            }
            Some(Ordering::Equal)
        } else {
            self.kind().partial_cmp(&other.kind())
        }
    }
}

#[derive(Debug, Clone, Ord, Eq)]
struct Hand2 {
    ranks: Vec<Rank2>,
    bid: usize,
    rank: usize,
}

impl PartialEq<Self> for Hand2 {
    fn eq(&self, other: &Self) -> bool {
        self.kind() == other.kind() && self.ranks == other.ranks
    }
}

impl PartialOrd for Hand2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.kind() == other.kind() {
            for i in 0..5 {
                if self.ranks[i] != other.ranks[i] {
                    return self.ranks[i].partial_cmp(&other.ranks[i]);
                }
            }
            Some(Ordering::Equal)
        } else {
            self.kind().partial_cmp(&other.kind())
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    pub fn kind(&self) -> HandType {
        let mut data = self.ranks.to_vec();
        data.sort();
        let card1 = data[0];
        let card2 = data[1];
        let card3 = data[2];
        let card4 = data[3];
        let card5 = data[4];

        if data[0] == data[1] && data[1] == data[2] && data[2] == data[3] && data[3] == data[4]
        {
            HandType::FiveOfAKind
        } else if card1 == card2 && card2 == card3 && card3 == card4
            || card2 == card3 && card3 == card4 && card4 == card5
        {
            HandType::FourOfAKind
        } else if card1 == card2 && card2 == card3
        { if card4 == card5 { HandType::FullHouse } else { HandType::ThreeOfAKind } } else if card2 == card3 && card3 == card4
        { if card1 == card5 { HandType::FullHouse } else { HandType::ThreeOfAKind } } else if card3 == card4 && card4 == card5
        { if card1 == card2 { HandType::FullHouse } else { HandType::ThreeOfAKind } } else if card1 == card2 && card3 == card4
            || card1 == card2 && card4 == card5
            || card2 == card3 && card4 == card5
        { HandType::TwoPairs } else if card1 == card2 || card2 == card3 || card3 == card4 || card4 == card5
        { HandType::OnePair } else { HandType::HighCard }
    }
}

impl From<&str> for Rank {
    fn from(value: &str) -> Self {
        match value {
            "2" => Rank::Two,
            "3" => Rank::Three,
            "4" => Rank::Four,
            "5" => Rank::Five,
            "6" => Rank::Six,
            "7" => Rank::Seven,
            "8" => Rank::Eight,
            "9" => Rank::Nine,
            "T" => Rank::Ten,
            "J" => Rank::Prince,
            "Q" => Rank::Queen,
            "K" => Rank::King,
            "A" => Rank::Ace,
            _ => panic!("Invalid rank: {}", value),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Prince,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Rank2 {
    Prince,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl From<&str> for Rank2 {
    fn from(value: &str) -> Self {
        match value {
            "2" => Rank2::Two,
            "3" => Rank2::Three,
            "4" => Rank2::Four,
            "5" => Rank2::Five,
            "6" => Rank2::Six,
            "7" => Rank2::Seven,
            "8" => Rank2::Eight,
            "9" => Rank2::Nine,
            "T" => Rank2::Ten,
            "J" => Rank2::Prince,
            "Q" => Rank2::Queen,
            "K" => Rank2::King,
            "A" => Rank2::Ace,
            _ => panic!("Invalid rank: {}", value),
        }
    }
}

impl Hand2 {
    pub fn kind(&self) -> HandType {
        let mut data = self.ranks.to_vec();
        data.sort();
        let card1 = data[0];
        let card2 = data[1];
        let card3 = data[2];
        let card4 = data[3];
        let card5 = data[4];

        if data[0] == data[1] && data[1] == data[2] && data[2] == data[3] && data[3] == data[4]
        {
            HandType::FiveOfAKind
        } else if card1 == card2 && card2 == card3 && card3 == card4
            || card2 == card3 && card3 == card4 && card4 == card5
        {
            if data.iter().any(|r| *r == Rank2::Prince) {
                HandType::FiveOfAKind
            } else {
                HandType::FourOfAKind
            }
        } else if card1 == card2 && card2 == card3
        {
            let c = data.iter().filter(|r| **r == Rank2::Prince).count();
            if c == 3 { if card4 == card5 { HandType::FiveOfAKind } else { HandType::FourOfAKind } }// else if c ==2 { HandType::FourOfAKind }}
            else if c == 2 { HandType::FiveOfAKind } else if c == 1 { HandType::FourOfAKind } else if card4 == card5 { HandType::FullHouse } else { HandType::ThreeOfAKind }
        } else if card2 == card3 && card3 == card4
        {
            let c = data.iter().filter(|r| **r == Rank2::Prince).count();
            if c == 3 { if card1 == card5 { HandType::FiveOfAKind } else { HandType::FourOfAKind } }// else if c ==2 { HandType::FourOfAKind }}
            else if c == 2 { HandType::FiveOfAKind } else if c == 1 { HandType::FourOfAKind } else if card1 == card5 { HandType::FullHouse } else { HandType::ThreeOfAKind }
        } else if card3 == card4 && card4 == card5
        {
            let c = data.iter().filter(|r| **r == Rank2::Prince).count();
            if c == 3 { if card1 == card2 { HandType::FiveOfAKind } else { HandType::FourOfAKind } }// else if c ==2 { HandType::FourOfAKind }}
            else if c == 2 { HandType::FiveOfAKind } else if c == 1 { HandType::FourOfAKind } else if card1 == card2 { HandType::FullHouse } else { HandType::ThreeOfAKind }
        } else if card1 == card2 && card3 == card4
            || card1 == card2 && card4 == card5
            || card2 == card3 && card4 == card5
        {
            let c = data.iter().filter(|r| **r == Rank2::Prince).count();
            if c == 2 { HandType::FourOfAKind } else if c == 1 { HandType::FullHouse } else { HandType::TwoPairs }
            // HandType::TwoPairs
        } else if card1 == card2 || card2 == card3 || card3 == card4 || card4 == card5
        {
            let c = data.iter().filter(|r| **r == Rank2::Prince).count();
            if c == 2 { HandType::ThreeOfAKind } else if c == 1 { HandType::ThreeOfAKind } else { HandType::OnePair }
            // HandType::OnePair
        } else {
            let c = data.iter().filter(|r| **r == Rank2::Prince).count();
            if c == 1 { HandType::OnePair } else { HandType::HighCard }
            // HandType::HighCard
        }
    }
}

impl Display for Rank2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Rank2::Prince => "J",
            Rank2::Two => "2",
            Rank2::Three => "3",
            Rank2::Four => "4",
            Rank2::Five => "5",
            Rank2::Six => "6",
            Rank2::Seven => "7",
            Rank2::Eight => "8",
            Rank2::Nine => "9",
            Rank2::Ten => "T",
            Rank2::Queen => "Q",
            Rank2::King => "K",
            Rank2::Ace => "A",
        };
        write!(f, "{}", s)
    }
}