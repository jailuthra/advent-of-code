use itertools::{self, zip_eq, Itertools};
use std::io::stdin;

fn main() {
    let mut list1: Vec<u32> = Vec::new();
    let mut list2: Vec<u32> = Vec::new();

    for line in stdin().lines() {
        let line = line.unwrap();
        let (a, b) = line.split_whitespace().collect_tuple().unwrap();
        list1.push(a.parse::<u32>().unwrap());
        list2.push(b.parse::<u32>().unwrap());
    }

    list1.sort();
    list2.sort();

    let mut total = 0;

    for (a, b) in zip_eq(list1, list2) {
        total += a.abs_diff(b);
    }

    println!("{total}");
}
