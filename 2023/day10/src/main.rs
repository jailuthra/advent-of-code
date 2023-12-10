use std::{collections::HashMap, io::stdin};

#[derive(Debug, PartialEq, Eq, Hash)]
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
    let graph = convert_graph(&graph);
    let start = *graph.keys().find(|&x| x.pipe == Some(Pipe::Start)).unwrap();
    //println!("Start: {:?}", graph.get(start));
    let chain = find_loop(&graph, start);
    println!("Length/2: {}", chain.len() / 2);
}
