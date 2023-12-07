use itertools::Itertools;
use std::{io::stdin, str::FromStr};

#[derive(Debug)]
struct Card {
    id: usize,
    winning: Vec<u32>,
    ours: Vec<u32>,
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, rest) = s.split(":").collect_tuple().unwrap();
        let id = usize::from_str(id.split_whitespace().skip(1).next().unwrap()).unwrap();
        let (winning, ours) = rest
            .split("|")
            .map(|x| {
                x.split_whitespace()
                    .map(|d| u32::from_str(d).unwrap())
                    .collect_vec()
            })
            .collect_tuple()
            .unwrap();
        Ok(Card { id, winning, ours })
    }
}

fn main() {
    for line in stdin().lines() {
        let card = Card::from_str(&line.unwrap());
        println!("{:?}", card);
    }
}
