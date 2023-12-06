use std::{str::FromStr, num::ParseIntError, io::stdin};
use itertools::Itertools;

#[derive(Debug)]
struct Subset {
    red: u32,
    green: u32,
    blue: u32,
}

impl FromStr for Subset {
    type Err = ParseIntError; 

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut subset = Subset {red: 0, green: 0, blue: 0}; 
        for (count, color) in s.split(",").map(|x| x.trim().split(" ").next_tuple().unwrap()) {
            let count = u32::from_str(count)?;
            match color {
                "red" => { subset.red = count },
                "green" => { subset.green = count },
                "blue" => { subset.blue = count },
                _ => panic!("Invalid color"),
            }
        }
        Ok(subset)
    }
}

impl Subset {
    fn power(&self) -> u64 {
        self.red as u64 * self.green as u64 * self.blue as u64
    }
}

#[derive(Debug)]
struct Game {
    id: usize,
    subsets: Vec<Subset>,
}

impl FromStr for Game {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, subsets) = s.split(":").next_tuple().unwrap();
        let id = usize::from_str(id.strip_prefix("Game ").unwrap())?;
        let subsets = subsets.trim().split(";").map(|x| Subset::from_str(x).unwrap()).collect::<Vec<Subset>>();
        Ok(Game{id, subsets})
    }
}

impl Game {
    /**
     * Determine is a given game is possible given the bag was loaded with "load" subset
     */
    fn is_possible(&self, load: Subset) -> bool {
        for subset in &self.subsets {
            if subset.red > load.red || subset.green > load.green || subset.blue > load.blue {
                return false;
            }
        }
        true
    }

    fn min_viable_set(&self) -> Subset {
        let mut mvs = Subset {red: 0, green: 0, blue: 0};
        for subset in &self.subsets {
            if subset.red > mvs.red {
                mvs.red = subset.red;
            }
            if subset.green > mvs.green {
                mvs.green = subset.green;
            }
            if subset.blue > mvs.blue {
                mvs.blue = subset.blue;
            }
        }
        mvs
    }
}

fn main() {
    let mut sum = 0;
    let mut powersum = 0;
    for line in stdin().lines() {
        let game = Game::from_str(&line.unwrap()).unwrap();
        if game.is_possible(Subset {red: 12, green: 13, blue: 14}) {
            sum += game.id;
        }
        powersum += game.min_viable_set().power();
    }
    println!("{sum} {powersum}");
}
