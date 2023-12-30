use std::{collections::BTreeMap, io::stdin};

use itertools::Itertools;

#[derive(Debug, Clone)]
struct Range {
    src: u64,
    dst: u64,
    len: u64,
}

#[derive(Debug, Clone)]
struct Map {
    name: String,
    ranges: BTreeMap<u64, Range>,
}

impl Range {
    // Given a single range LHS that maps
    // x -> Y
    // where x is a contiguous subset of X with len items
    //
    // And another set of ranges (RHS) that completely maps
    // Y -> Z
    //
    // Output a set of ranges that maps
    // x -> Z
    // where x is a contiguous subset of X, with len items
    fn compose(lhs: &Range, rhs: Vec<Range>) -> Vec<Range> {
        let mut out = Vec::new();
        let mut start = lhs.dst;
        let end = start + lhs.len - 1;
        for rhs_range in rhs {
            if start >= rhs_range.src && start < rhs_range.src + rhs_range.len {
                // match found, insert a new range to output
                let new = Range {
                    src: lhs.src + (start - lhs.dst),
                    dst: rhs_range.dst + (start - rhs_range.src),
                    len: (rhs_range.len - (start - rhs_range.src)).min((end + 1) - start),
                };
                start += new.len;
                out.push(new);
                // if some values in lhs range are leftover, keep searching
                let leftover = end as i64 - rhs_range.len as i64;
                if leftover <= 0 {
                    break;
                }
            }
        }
        out
    }
}

impl Map {
    // lookup the dst value of a particular src
    fn lookup(&self, src: u64) -> u64 {
        for range in self.ranges.values() {
            if src >= range.src && src < range.src + range.len {
                return range.dst + (src - range.src);
            }
            if src < range.src {
                break;
            }
        }
        return src;
    }

    // return a list of ranges that map the entire domain (0..u64::MAX) of src values
    fn filled_ranges(&self) -> Vec<Range> {
        let mut out = Vec::new();
        let mut last_unmapped = 0;
        for range in self.ranges.values() {
            if range.src > last_unmapped {
                // add missing range
                out.push(Range {
                    src: last_unmapped,
                    dst: last_unmapped,
                    len: range.src - last_unmapped,
                });
            }
            out.push(range.clone());
            last_unmapped = range.src + range.len;
        }
        out.push(Range {
            src: last_unmapped,
            dst: last_unmapped,
            len: u64::MAX - last_unmapped,
        });
        out
    }

    fn pprint(&self) {
        print!("{}:\n", self.name);
        for range in self.ranges.values() {
            println!("src: {}, dst: {}, len: {}", range.src, range.dst, range.len);
        }
    }
}

impl std::ops::Mul for Map {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut out = Self::Output {
            name: self.name.clone() + "-to-" + &rhs.name,
            ranges: BTreeMap::new(),
        };
        // Each input range can be divided into multiple output ranges
        for range in self.filled_ranges() {
            for new in Range::compose(&range, rhs.filled_ranges()) {
                out.ranges.insert(new.src, new);
            }
        }
        out
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

    (seeds, maps)
}

fn main() {
    let (seeds, maps) = parse();
    let mut locations = Vec::new();
    let mut seeds_to_locations = maps.get(0).unwrap().clone();
    // Create a combined map from seeds-to-locations
    for map in maps.iter().skip(1) {
        seeds_to_locations = seeds_to_locations * map.clone();
    }
    seeds_to_locations.pprint();
    // part 1
    for seed in &seeds {
        locations.push(seeds_to_locations.lookup(*seed));
    }
    println!("Part-1 Min {}", locations.iter().min().unwrap());
    // part 2
    let mut locations = Vec::new();
    for (&seed, &len) in seeds.iter().tuples() {
        // take a contiguous range of seeds
        let r = Range {
            src: seed,
            dst: seed,
            len,
        };
        // find possible location ranges these seeds end up at
        for seed_subset_to_locations in Range::compose(
            &r,
            seeds_to_locations
                .clone()
                .ranges
                .into_values()
                .collect_vec(),
        ) {
            // note down the possible locations
            // (only the start of the range matters as that is smallest)
            let location = seed_subset_to_locations.dst;
            locations.push(location);
        }
    }
    println!("Part-2 Min {}", locations.iter().min().unwrap());
}
