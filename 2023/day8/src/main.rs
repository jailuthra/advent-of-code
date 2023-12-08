use itertools::Itertools;
use num::integer::lcm;
use std::{collections::HashMap, io::stdin};

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Left = 0,
    Right,
}

impl From<char> for Instruction {
    fn from(value: char) -> Self {
        match value {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            _ => panic!("Invalid character {} for instruction", value),
        }
    }
}

fn main() {
    let mut lines = stdin().lines();
    let instructions = lines
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .map(|c| Instruction::from(c))
        .collect::<Vec<Instruction>>();
    println!("{:?}", instructions);
    let mut map: HashMap<String, [String; 2]> = HashMap::new();
    for line in lines.skip(1) {
        let line = line.unwrap();
        let (from, to) = line.split("=").map(|s| s.trim()).collect_tuple().unwrap();
        let (l, r) = to
            .strip_prefix("(")
            .unwrap()
            .strip_suffix(")")
            .unwrap()
            .split(", ")
            .collect_tuple()
            .unwrap();
        map.insert(from.to_string(), [l.to_string(), r.to_string()]);
    }
    println!("{:?}", map);

    // Part 2
    // For Part 1 replace A with AAA and Z with ZZZ :)
    let pos = map.keys().filter(|k| k.ends_with("A")).collect_vec();
    let mut total_steps: Vec<u64> = Vec::new();
    for p in pos {
        let mut steps = 0;
        let mut pos = &p.clone();
        for &i in instructions.iter().cycle() {
            if pos.ends_with("Z") {
                break;
            }
            print!("Current {:?} Instruction {:?} ", pos, i);
            pos = &map.get(pos).unwrap()[i as usize];
            println!("Next {:?}", pos);
            steps += 1;
        }
        println!("Steps taken for {} -> {} {}", p, pos, steps);
        total_steps.push(steps);
    }
    println!(
        "Total steps taken will be {}",
        total_steps.into_iter().reduce(lcm).unwrap()
    );
}
