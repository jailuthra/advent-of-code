use std::{collections::HashMap, io::stdin};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Pipe {
    LeftRight,
    UpDown,
    DownRight,
    DownLeft,
    UpRight,
    UpLeft,
    Start,
}

impl From<char> for Pipe {
    fn from(value: char) -> Self {
        match value {
            '-' => Self::LeftRight,
            '|' => Self::UpDown,
            'F' => Self::DownRight,
            '7' => Self::DownLeft,
            'L' => Self::UpRight,
            'J' => Self::UpLeft,
            'S' => Self::Start,
            _ => panic!("Unknown pipe type"),
        }
    }
}

impl Into<char> for Pipe {
    fn into(self) -> char {
        match self {
            Self::LeftRight => '-',
            Self::UpDown => '|',
            Self::DownRight => '┌',
            Self::DownLeft => '┐',
            Self::UpRight => '└',
            Self::UpLeft => '┘',
            Self::Start => 'S',
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Node {
    pipe: Option<Pipe>,
    pos: (usize, usize),
}

fn convert_graph(vec: &Vec<Vec<Node>>) -> HashMap<&Node, Vec<&Node>> {
    let mut graph: HashMap<&Node, Vec<&Node>> = HashMap::new();
    for i in 0..vec.len() {
        let row = vec.get(i).unwrap();
        for j in 0..row.len() {
            let n = row.get(j).unwrap();
            if n.pipe.is_none() {
                continue;
            }
            graph.insert(n, Vec::new());
            // Check Left
            if j > 0
                && (n.pipe == Some(Pipe::LeftRight)
                    || n.pipe == Some(Pipe::DownLeft)
                    || n.pipe == Some(Pipe::UpLeft)
                    || n.pipe == Some(Pipe::Start))
            {
                let neighbor = row.get(j - 1).unwrap();
                match &neighbor.pipe {
                    None => {}
                    Some(p) => match p {
                        Pipe::LeftRight | Pipe::DownRight | Pipe::UpRight | Pipe::Start => {
                            graph.get_mut(n).unwrap().push(neighbor)
                        }
                        _ => {}
                    },
                }
            }
            // Check Right
            if j < row.len() - 1
                && (n.pipe == Some(Pipe::LeftRight)
                    || n.pipe == Some(Pipe::DownRight)
                    || n.pipe == Some(Pipe::UpRight)
                    || n.pipe == Some(Pipe::Start))
            {
                let neighbor = row.get(j + 1).unwrap();
                match &neighbor.pipe {
                    None => {}
                    Some(p) => match p {
                        Pipe::LeftRight | Pipe::DownLeft | Pipe::UpLeft | Pipe::Start => {
                            graph.get_mut(n).unwrap().push(neighbor)
                        }
                        _ => {}
                    },
                }
            }
            // Check Up
            if i > 0
                && (n.pipe == Some(Pipe::UpDown)
                    || n.pipe == Some(Pipe::UpLeft)
                    || n.pipe == Some(Pipe::UpRight)
                    || n.pipe == Some(Pipe::Start))
            {
                let neighbor = vec.get(i - 1).unwrap().get(j).unwrap();
                match &neighbor.pipe {
                    None => {}
                    Some(p) => match p {
                        Pipe::UpDown | Pipe::DownLeft | Pipe::DownRight | Pipe::Start => {
                            graph.get_mut(n).unwrap().push(neighbor)
                        }
                        _ => {}
                    },
                }
            }
            // Check Down
            if i < vec.len() - 1
                && (n.pipe == Some(Pipe::UpDown)
                    || n.pipe == Some(Pipe::DownLeft)
                    || n.pipe == Some(Pipe::DownRight)
                    || n.pipe == Some(Pipe::Start))
            {
                let neighbor = vec.get(i + 1).unwrap().get(j).unwrap();
                match &neighbor.pipe {
                    None => {}
                    Some(p) => match p {
                        Pipe::UpDown | Pipe::UpLeft | Pipe::UpRight | Pipe::Start => {
                            graph.get_mut(n).unwrap().push(neighbor)
                        }
                        _ => {}
                    },
                }
            }
        }
    }
    graph
}

fn find_loop<'a>(graph: &'a HashMap<&Node, Vec<&Node>>, current: &'a Node) -> Vec<&'a Node> {
    let mut chain = Vec::new();
    let mut visited = HashMap::new();
    // add current node to loop
    chain.push(current);
    visited.insert(current, true);

    // pick neighbour
    let mut current = *graph.get(current).unwrap().get(0).unwrap();
    loop {
        // add neighbour to loop
        chain.push(current);
        visited.insert(current, true);
        //println!("Current Node {:?}", current);
        //println!("\tChain: {:?}", chain);
        // find next neighbour that is not already visited
        match graph
            .get(current)
            .unwrap()
            .iter()
            .find(|&x| x.pos != current.pos && !visited.contains_key(x))
        {
            None => break,
            Some(x) => current = x,
        }
    }
    chain
}

fn count_inside_points(
    graph: &HashMap<&Node, Vec<&Node>>,
    chain: &Vec<&Node>,
    rows: usize,
    columns: usize,
) -> u64 {
    let mut output = Vec::new();
    let mut inside = 0u64;
    for i in 0..rows {
        let mut row = Vec::new();
        let mut pipe_count = 0;
        for j in 0..columns {
            if let Some(&n) = chain.iter().find(|n| n.pos == (i, j)) {
                row.push(Pipe::into(*n.pipe.as_ref().unwrap()));
                match n.pipe.as_ref().unwrap() {
                    // Count Vertical Pipe Crossings
                    Pipe::UpDown | Pipe::UpLeft | Pipe::UpRight => {
                        pipe_count += 1;
                    }
                    Pipe::Start => {
                        let start = *graph.keys().find(|&x| x.pipe == Some(Pipe::Start)).unwrap();
                        // Check if input's Start behaves like a vertical crossing
                        for &neighbour in graph.get(start).unwrap().iter() {
                            if neighbour.pos.0 as i64 == start.pos.0 as i64 - 1 {
                                pipe_count += 1;
                                break;
                            }
                        }
                    }
                    _ => {}
                };
            } else if pipe_count % 2 == 0 {
                row.push('.');
            } else {
                row.push('X');
                inside += 1;
            }
        }
        output.push(row.into_iter().collect::<String>());
    }
    for row in output {
        println!("{row}");
    }
    inside
}

fn main() {
    let mut graph: Vec<Vec<Node>> = Vec::new();
    for (i, line) in stdin().lines().enumerate() {
        let mut row = Vec::new();
        for (j, c) in line.unwrap().chars().enumerate() {
            let n: Node = match c {
                '.' => Node {
                    pipe: None,
                    pos: (i, j),
                },
                c => Node {
                    pipe: Some(Pipe::from(c)),
                    pos: (i, j),
                },
            };
            //println!("Found {:?}", n);
            row.push(n);
        }
        graph.push(row);
    }
    let rows = graph.len();
    let columns = graph.get(0).unwrap().len();
    let graph = convert_graph(&graph);
    let start = *graph.keys().find(|&x| x.pipe == Some(Pipe::Start)).unwrap();
    //println!("Start: {:?}", graph.get(start));
    let chain = find_loop(&graph, start);
    println!("Length/2: {}", chain.len() / 2);

    // PART 2
    let inside = count_inside_points(&graph, &chain, rows, columns);
    println!("INSIDE COUNT: {inside}");
}
