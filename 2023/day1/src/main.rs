use std::io;

fn get_calibration_value(s: &str) -> u32 {
    let digits = s.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<u32>>();
    digits.first().unwrap() * 10 + digits.last().unwrap()
}

fn main() {
    let stdin = io::stdin();
    let mut sum = 0u32;
    for line in stdin.lines() {
        let val = get_calibration_value(line.unwrap().as_ref());
        sum += val;
    }
    println!("{sum}");
}
