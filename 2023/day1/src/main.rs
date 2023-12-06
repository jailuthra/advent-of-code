use std::io;

fn get_calibration_value_part2(s: &str) -> u32 {
    let mut digits: Vec<u32> = Vec::new();

    for (i, _) in s.char_indices() {
        let substr = &s[i..];
        if let Some(d) = substr.chars().next().unwrap().to_digit(10) {
            // either the current char is a digit
            digits.push(d);
        } else {
            // or it is a digit spelled out as a string
            if substr.starts_with("one") {
                digits.push(1);
            } else if substr.starts_with("two") {
                digits.push(2);
            } else if substr.starts_with("three") {
                digits.push(3);
            } else if substr.starts_with("four") {
                digits.push(4);
            } else if substr.starts_with("five") {
                digits.push(5);
            } else if substr.starts_with("six") {
                digits.push(6);
            } else if substr.starts_with("seven") {
                digits.push(7);
            } else if substr.starts_with("eight") {
                digits.push(8);
            } else if substr.starts_with("nine") {
                digits.push(9);
            } else if substr.starts_with("zero") {
                digits.push(0);
            }
        }
    }
    println!("{:?}", digits);
    digits.first().unwrap() * 10 + digits.last().unwrap()
}

fn get_calibration_value_part1(s: &str) -> u32 {
    let digits = s.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<u32>>();
    println!("{:?}", digits);
    digits.first().unwrap() * 10 + digits.last().unwrap()
}

fn main() {
    let stdin = io::stdin();
    let mut sum = 0u32;
    for line in stdin.lines() {
        let val = get_calibration_value_part2(line.unwrap().as_ref());
        sum += val;
    }
    println!("{sum}");
}
