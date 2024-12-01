use itertools::{self, zip_eq, Itertools};
use std::{collections::HashMap, io::stdin};

fn main() {
    let mut list1: Vec<u32> = Vec::new();
    let mut list2: Vec<u32> = Vec::new();

    for line in stdin().lines() {
        let line = line.unwrap();
        let (a, b) = line.split_whitespace().collect_tuple().unwrap();
        list1.push(a.parse::<u32>().unwrap());
        list2.push(b.parse::<u32>().unwrap());
    }

    // PART 1

    list1.sort();
    list2.sort();

    let mut diff = 0;

    for (&a, &b) in zip_eq(&list1, &list2) {
        diff += a.abs_diff(b);
    }

    println!("{diff}");

    // PART 2

    let mut map: HashMap<u32, usize> = HashMap::new();

    for b in list2 {
        if let Some(count) = map.get(&b) {
            map.insert(b, count + 1);
        } else {
            map.insert(b, 1);
        }
    }

    let mut similarity = 0;

    for a in list1 {
        if let Some(count) = map.get(&a) {
            similarity += a * *count as u32;
        }
    }

    println!("{similarity}");
}
