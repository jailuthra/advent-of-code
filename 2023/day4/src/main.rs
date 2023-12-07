use itertools::Itertools;
use std::{collections::HashMap, io::stdin, str::FromStr};

#[derive(Debug)]
struct Card {
    id: usize,
    winning: Vec<u32>,
    ours: Vec<u32>,
    matches: usize,
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
        Ok(Card {
            matches: Card::matches(&winning, &ours),
            id,
            winning,
            ours,
        })
    }
}

impl Card {
    fn matches(winning: &Vec<u32>, ours: &Vec<u32>) -> usize {
        let mut found = 0;
        for w in &winning[..] {
            found += match ours.iter().find(|o| *o == w) {
                Some(_) => 1,
                None => 0,
            }
        }
        found
    }

    fn score(&self) -> u64 {
        if self.matches > 0 {
            1 << (self.matches - 1)
        } else {
            0
        }
    }
}

fn add_card(map: &mut HashMap<usize, u32>, card: &Card, to_add: u32) {
    if let Some(count) = map.get(&card.id) {
        map.insert(card.id, count + to_add);
    } else {
        map.insert(card.id, to_add);
    }
}

fn main() {
    let mut score = 0u64;
    let mut cards = Vec::new();
    // Part 1
    for line in stdin().lines() {
        let card = Card::from_str(&line.unwrap()).unwrap();
        println!("{:?} score {}", card, card.score());
        score += card.score();
        cards.push(card);
    }
    println!("Score {score}");

    // Part 2
    let mut map = HashMap::new();
    for (i, card) in cards.iter().enumerate() {
        // Add current card to total
        add_card(&mut map, card, 1);
        // Add next "matches" # of cards to total
        let matches = card.matches;
        match matches {
            0 => continue,
            matches => {
                let count = matches.min(cards.len() - i);
                for next in cards.iter().skip(i + 1).take(count) {
                    let to_add: u32 = map.get(&card.id).unwrap().to_owned();
                    add_card(&mut map, next, to_add);
                }
            }
        }
    }
    println!("Total cards {:?}", map.values().sum::<u32>());
}
