use std::io::stdin;

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn calc_travel(&self, hold_time: u64) -> u64 {
        let speed = hold_time;
        (self.time - hold_time) * speed
    }

    fn calc_margin(&self) -> u64 {
        let mut margin = 0;
        for hold in 0..=self.time {
            let travel = self.calc_travel(hold);
            //println!("hold time {}, travel dist {}", hold, travel);
            if travel > self.distance {
                margin += 1;
            }
        }
        margin
    }
}

fn _extract_nums(s: &str) -> Vec<u64> {
    s.split_whitespace()
        .skip(1)
        .map(|s| u64::from_str_radix(s, 10).unwrap())
        .collect::<Vec<u64>>()
}

fn _part1() {
    let mut lines = stdin().lines();
    let timings = _extract_nums(&lines.next().unwrap().unwrap());
    let distances = _extract_nums(&lines.next().unwrap().unwrap());
    let mut total = 1u64;
    for (time, distance) in timings.into_iter().zip(distances.into_iter()) {
        let r = Race { time, distance };
        let margin = r.calc_margin();
        println!("{:?} margin {}", r, margin);
        total *= margin;
    }
    println!("{total}");
}

fn extract_num_with_kerning(s: &str) -> u64 {
    u64::from_str_radix(
        &s.split(":")
            .skip(1)
            .next()
            .unwrap()
            .split_whitespace()
            .collect::<String>(),
        10,
    )
    .unwrap()
}

fn main() {
    let mut lines = stdin().lines();
    let r = Race {
        time: extract_num_with_kerning(&lines.next().unwrap().unwrap()),
        distance: extract_num_with_kerning(&lines.next().unwrap().unwrap()),
    };
    let margin = r.calc_margin();
    println!("{:?} margin {}", r, margin);
}
