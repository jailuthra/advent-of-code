#[derive(Debug)]
struct Assignment {
    start: u32,
    end: u32,
}

impl TryFrom<&str> for Assignment {
    type Error = &'static str;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        let split: Vec<&str> = string.split("-").collect();
        match split[..] {
            [start, end] => Ok(Self
                {
                    start: start.parse().unwrap(),
                    end: end.parse().unwrap()
                }), 
            _ => Err("Wrong input, assignment should be a string like 1-4 or 2-2 etc.")
        }
    }
}

impl Assignment {
    fn overlaps(&self, other: &Assignment) -> bool {
        other.start <= self.end && other.start >= self.start
            || self.start <= other.end && self.start >= other.start
    }
    fn totally_overlaps(&self, other: &Assignment) -> bool {
        self.start <= other.start && self.end >= other.end
            || self.start >= other.start && self.end <= other.end
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut overlap_count = 0 as u32;
    let mut totally_overlap_count = 0 as u32;
    for line in stdin.lines()
        .map(|line| line.unwrap()) {
        let assignments: Vec<Assignment> = line.split(",")
            .map(|assignment| Assignment::try_from(assignment).unwrap()).collect();
        let assignment1 = assignments.get(0).unwrap();
        let assignment2 = assignments.get(1).unwrap();
        if assignment1.overlaps(assignment2) {
            overlap_count += 1;
        }
        if assignment1.totally_overlaps(assignment2) {
            totally_overlap_count += 1;
        }
        println!("{assignment1:?} & {assignment2:?}");
    }
    println!("{overlap_count}, {totally_overlap_count}");
}
