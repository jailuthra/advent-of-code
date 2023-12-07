use itertools::Itertools;
use std::{io::stdin, str::FromStr};

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
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

#[derive(Debug)]
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
    let hands: Vec<Hand> = Vec::new();
    for line in stdin().lines() {
        println!("{:?}", Hand::from_str(&line.unwrap()));
    }
}
