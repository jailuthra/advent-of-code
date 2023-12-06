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

fn extract_nums(s: &str) -> Vec<u64> {
    s.split_whitespace()
        .skip(1)
        .map(|s| u64::from_str_radix(s, 10).unwrap())
        .collect::<Vec<u64>>()
}

fn main() {
    let mut lines = stdin().lines();
    let timings = extract_nums(&lines.next().unwrap().unwrap());
    let distances = extract_nums(&lines.next().unwrap().unwrap());
    let mut total = 1u64;
    for (time, distance) in timings.into_iter().zip(distances.into_iter()) {
        let r = Race { time, distance };
        let margin = r.calc_margin();
        println!("{:?} margin {}", r, margin);
        total *= margin;
    }
    println!("{total}");
}
