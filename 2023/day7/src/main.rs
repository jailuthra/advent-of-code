use itertools::Itertools;
use std::{collections::HashMap, io::stdin, str::FromStr};

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
enum Card {
    One = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard = 0,
    Pair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
    bid: u32,
}

impl Hand {
    fn update(&mut self, s: &str) {
        for (i, c) in s.chars().enumerate() {
            let c: Card = match c {
                '1' => Card::One,
                '2' => Card::Two,
                '3' => Card::Three,
                '4' => Card::Four,
                '5' => Card::Five,
                '6' => Card::Six,
                '7' => Card::Seven,
                '8' => Card::Eight,
                '9' => Card::Nine,
                'T' => Card::Ten,
                'J' => Card::Jack,
                'Q' => Card::Queen,
                'K' => Card::King,
                'A' => Card::Ace,
                _ => panic!(),
            };
            self.cards[i] = c;
        }
    }

    fn calc_type(&self) -> HandType {
        let mut m: HashMap<&Card, u8> = HashMap::new();
        for c in &self.cards[..] {
            match m.get(&c) {
                Some(count) => {
                    m.insert(c, count + 1);
                }
                None => {
                    m.insert(c, 1);
                }
            }
        }
        let sorted = m
            .into_iter()
            .sorted_by(|(_, a), (_, b)| b.cmp(a))
            .collect_vec();
        match sorted.get(0).unwrap() {
            (_, 5) => HandType::FiveKind,
            (_, 4) => HandType::FourKind,
            (_, 1) => HandType::HighCard,
            (_, 3) => {
                if sorted.get(1).unwrap().1 == 2 {
                    HandType::FullHouse
                } else {
                    HandType::ThreeKind
                }
            }
            (_, 2) => {
                if sorted.get(1).unwrap().1 == 2 {
                    HandType::TwoPair
                } else {
                    HandType::Pair
                }
            }
            _ => panic!("Invalid number of cards"),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.calc_type() != other.calc_type() {
            self.calc_type().cmp(&other.calc_type())
        } else {
            for (s, o) in self.cards.iter().zip_eq(other.cards.iter()) {
                if s != o {
                    return s.cmp(o);
                }
            }
            std::cmp::Ordering::Equal
        }
    }
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand, bid) = s.split(" ").next_tuple().unwrap();
        let mut h = Hand {
            cards: [Card::Ace, Card::Ace, Card::Ace, Card::Ace, Card::Ace],
            bid: u32::from_str(bid).unwrap(),
        };
        h.update(hand);
        Ok(h)
    }
}

fn main() {
    let mut hands: Vec<Hand> = Vec::new();
    let mut total = 0;
    for line in stdin().lines() {
        let h = Hand::from_str(&line.unwrap()).unwrap();
        hands.push(h);
    }
    hands.sort();
    for (i, h) in hands.iter().enumerate() {
        let winnings = (i + 1) as u32 * h.bid;
        println!("{:?} winnings {}", h, winnings);
        total += winnings;
    }
    println!("Total {total}");
}
