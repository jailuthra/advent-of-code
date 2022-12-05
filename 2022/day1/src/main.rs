use std::io;
use std::collections::BinaryHeap;

fn main() {
    let stdin = io::stdin();
    let mut heap = BinaryHeap::new();
    let mut calorie_sum = 0;
    for line in stdin.lines() {
        match line.unwrap().trim() {
            "" => {
                heap.push(calorie_sum);
                calorie_sum = 0;
            },
            l => {
                let calorie: u64 = l.parse().unwrap();
                calorie_sum += calorie;
            },
        }
    }
    println!("Max: {}", heap.peek().unwrap());
    let mut sum_of_max_three = 0;
    for _ in 0..3 {
        sum_of_max_three += heap.pop().unwrap();
    }
    println!("Sum of top 3: {sum_of_max_three}");
}
