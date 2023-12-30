use std::{collections::BTreeMap, io::stdin};

#[derive(Debug)]
struct Range {
    src: u64,
    dst: u64,
    len: u64,
}

#[derive(Debug)]
struct Map {
    name: String,
    ranges: BTreeMap<u64, Range>,
}

impl Map {
    fn lookup(&self, src: u64) -> u64 {
        for range in self.ranges.values() {
            if src >= range.src && src < range.src + range.len {
                return range.dst + (src - range.src);
            }
        }
        return src;
    }
}

fn parse() -> (Vec<u64>, Vec<Map>) {
    let mut lines = stdin().lines();
    // Parse seeds
    let seeds = lines.next().unwrap().unwrap();
    let seeds = seeds
        .split("seeds: ")
        .skip(1)
        .next()
        .unwrap()
        .split(" ")
        .map(|x| u64::from_str_radix(x, 10).unwrap())
        .collect::<Vec<u64>>();
    println!("seeds {:?}", seeds);
    // Skip empty line
    lines.next();
    // Read each map
    let mut maps = Vec::new();
    while let Some(Ok(line)) = lines.next() {
        let mut map = Map {
            name: line.split(" ").next().unwrap().to_string(),
            ranges: BTreeMap::new(),
        };
        while let Some(Ok(line)) = lines.next() {
            if line.is_empty() {
                break;
            }
            let mut s = line.split(" ").map(|x| u64::from_str_radix(x, 10).unwrap());
            let r = Range {
                dst: s.next().unwrap(),
                src: s.next().unwrap(),
                len: s.next().unwrap(),
            };
            map.ranges.insert(r.src, r);
        }
        maps.push(map);
    }
    println!("{:?}", maps);
    (seeds, maps)
}

fn main() {
    let (seeds, maps) = parse();
    let mut locations = Vec::new();
    for seed in seeds {
        let mut s = seed;
        for map in &maps {
            s = map.lookup(s);
            //println!("for seed {}, {} output is {}", seed, map.name, s);
        }
        println!("for seed {}, location is {}", seed, s);
        locations.push(s);
    }
    println!("Min Location {}", locations.iter().min().unwrap());
}
