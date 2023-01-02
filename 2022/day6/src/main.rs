use std::collections::HashMap;

// Part 1
//const MARKER_LEN: usize = 4;
// Part 2
const MARKER_LEN: usize = 14;

fn main() {
    let mut line = String::new();
    let mut set: HashMap<char, usize> = HashMap::with_capacity(4);
    _ = std::io::stdin().read_line(&mut line);
    let buf: Vec<char>  = line.chars().collect();
    for i in 0..buf.len()-1 {
        let c = buf.get(i).unwrap();
        if set.contains_key(c) {
            let count = set.get_mut(c).unwrap();
            *count += 1;
        } else {
            set.insert(*c, 1);
        }
        println!("Set {:?}", set);
        if i < (MARKER_LEN - 1) { continue; }
        if set.keys().len() == MARKER_LEN {
            println!("{}", i+1);
            return;
        } else {
            let prev = buf.get(i-(MARKER_LEN-1)).unwrap();
            let count = set.remove(prev).unwrap();
            if count > 1 {
                set.insert(*prev, count-1);
            }
        }
    }
}
