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
            let digit_text = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
            for (i, text) in digit_text.iter().enumerate() {
                if substr.starts_with(text) {
                    digits.push(i as u32);
                    break;
                }
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
