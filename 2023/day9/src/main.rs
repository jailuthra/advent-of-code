use itertools::Itertools;
use std::io::stdin;

fn extrapolate(history: &Vec<i32>) -> i32 {
    let new = history
        .iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect_vec();
    if new.iter().find(|&&x| x != 0) == None {
        history.last().unwrap() + 0
    } else {
        history.last().unwrap() + extrapolate(&new)
    }
}

fn main() {
    let mut sum = 0;
    let mut reverse_sum = 0;
    for line in stdin().lines() {
        let history = line
            .unwrap()
            .split_whitespace()
            .map(|c| i32::from_str_radix(c, 10).unwrap())
            .collect::<Vec<i32>>();
        let e = extrapolate(&history);
        sum += e;
        println!("{:?}, extrapolation {}", history, e);
        let history: Vec<i32> = history.into_iter().rev().collect();
        let e = extrapolate(&history);
        println!("Reverse {:?}, extrapolation {}", history, e);
        reverse_sum += e;
    }
    println!("SUM: {sum}");
    println!("REVERSE SUM: {reverse_sum}");
}
