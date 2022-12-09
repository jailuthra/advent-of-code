#[derive(Debug, Copy, Clone)]
enum Move {
    Rock = 1,
    Paper,
    Scissors,
}

fn parse_move(input: &str) -> Move {
    return match input {
        "A"|"X" => Move::Rock,
        "B"|"Y" => Move::Paper,
        "C"|"Z" => Move::Scissors,
        _ => panic!("Unknown input"),
    }

}

// Part two
fn infer_move(elf_move: Move, input: &str) -> Move {
    return match input {
        // Lose
        "X" => match elf_move {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        },
        // Draw
        "Y" => match elf_move {
            Move::Rock => Move::Rock,
            Move::Paper => Move::Paper,
            Move::Scissors => Move::Scissors,
        },
        // Win
        "Z" => match elf_move {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        },
        _ => panic!("Unknown input"),
    }
}

fn score(my_move: Move, elf_move: Move) -> u32 {
        return my_move as u32 + match my_move {
            Move::Rock => match elf_move {
                Move::Rock => 3,
                Move::Paper => 0,
                Move::Scissors => 6,
            },
            Move::Paper => match elf_move {
                Move::Rock => 6,
                Move::Paper => 3,
                Move::Scissors => 0,
            },
            Move::Scissors => match elf_move {
                Move::Rock => 0,
                Move::Paper => 6,
                Move::Scissors => 3,
            },
        };
}

fn main() {
    let stdin = std::io::stdin();
    let mut total_score = 0;
    for line in stdin.lines() {
        let unwrapped = line.unwrap();
        let strategy: Vec<&str> = unwrapped.split(' ').collect();
        let elf_move = parse_move(strategy.get(0).unwrap());
        // Part 1
        // let my_move = parse_move(strategy.get(1).unwrap());
        // Part 2
        let my_move = infer_move(elf_move, strategy.get(1).unwrap());
        //println!("{:?} {:?}", elf_move, my_move);
        let score = score(my_move, elf_move);
        //println!("{score}");
        total_score += score;
    }
    println!("Total score {total_score}");
}
