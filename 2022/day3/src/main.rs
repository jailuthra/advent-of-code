use std::collections::{HashSet,HashMap};

#[derive(Debug)]
struct Rucksack {
    items: String,
}

impl Rucksack {
    fn find_duplicate(&self) -> char {
        let compartments = self.compartments();
        let mut unique = HashSet::new();
        for c in compartments.0.chars() {
            unique.insert(c);
        };
        for c in compartments.1.chars() {
            if unique.contains(&c) {
                return c;
            }
        };
        panic!("No duplicate item");
    }

    fn compartments(&self) -> (&str, &str) {
        self.items.split_at(self.items.len()/2)
    }

    fn find_common(group: &Vec<Self>) -> char {
        let mut item_count = HashMap::new();
        for (i, rucksack) in group.iter().enumerate() {
            for c in rucksack.items.chars() {
                match item_count.get(&c) {
                    None => item_count.insert(c, 1 << i),
                    Some(count) => item_count.insert(c, count | (1 << i)),
                };
            }
        }
        return *item_count.iter().filter(|(_,&i)| i == 0b111).next().unwrap().0;
    }
}

fn get_priority(c: char) -> u8 {
    if (c as u8) > ('a' as u8) {
        1 + (c as u8 - 'a' as u8)
    } else {
        27 + (c as u8 - 'A' as u8)
    }
}

fn _part1() {
    let stdin = std::io::stdin();
    let mut sum: u32 = 0;
    for line in stdin.lines() {
        let line = line.unwrap();
        let rucksack = Rucksack {
            items: line,
        };
        let duplicate = rucksack.find_duplicate();
        let priority = get_priority(duplicate);
        sum += priority as u32;
    }
    println!("Total: {}", sum);
}

fn part2() {
    let stdin = std::io::stdin();
    let mut sum: u32 = 0;
    let mut counter = 0;
    let mut rucksacks: Vec<Rucksack> = Vec::with_capacity(3);
    for line in stdin.lines() {
        let line = line.unwrap();
        let rucksack = Rucksack {
            items: line,
        };
        rucksacks.push(rucksack);
        counter += 1;
        if counter == 3 {
            let common = Rucksack::find_common(&rucksacks);
            let priority = get_priority(common);
            sum += priority as u32;
            counter = 0;
            rucksacks.retain(|_| false);
        }
    }
    println!("Total: {}", sum);
}

fn main() {
    part2();
}
