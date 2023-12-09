use itertools::Itertools;
use std::io::stdin;

fn extrapolate(history: &Vec<i32>) -> i32 {
    let mut zero_count = 0;
    let new = history
        .iter()
        .tuple_windows()
        .map(|(a, b)| {
            let diff = b - a;
            if diff == 0 {
                zero_count += 1;
            }
            diff
        })
        .collect_vec();
    if zero_count == new.len() {
        history.last().unwrap() + 0
    } else {
        history.last().unwrap() + extrapolate(&new)
    }
}

fn main() {
    let mut sum = 0;
    for line in stdin().lines() {
        let history = line
            .unwrap()
            .split_whitespace()
            .map(|c| i32::from_str_radix(c, 10).unwrap())
            .collect::<Vec<i32>>();
        let e = extrapolate(&history);
        sum += e;
        println!("{:?}, extrapolation {}", history, e);
    }
    println!("SUM: {sum}");
}
