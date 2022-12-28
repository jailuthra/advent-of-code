/**
 * This could be done much more simply by using Vec<> but I wanted to practice implementing a stack
 * manually with Box<> (heap) allocated nodes
 */

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(val: T) -> Self {
        Self {val, next: None}
    }
}

#[derive(Debug)]
struct Stack<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Self {head: None}
    }

    fn push(&mut self, mut val: Box<Node<T>>) {
        val.next = self.head.take();
        self.head = Some(val);
    }

    fn append(&mut self, val: Box<Node<T>>) {
        match self.head {
            None => self.head = Some(val),
            Some(ref mut n) => {
                let mut cur = n;
                loop {
                    match cur.next {
                        None => {
                            cur.next = Some(val);
                            break;
                        },
                        Some(ref mut x) => cur = x,
                    }
                }
            }
        }
    }

    fn pop(&mut self) -> Option<Box<Node<T>>> {
        let ret = self.head.take();
        if let Some(mut node) = ret {
            self.head = node.next.take();
            return Some(node);
        }
        None
    }

    fn peek(&self) -> Option<&T> {
        match self.head {
            None => None,
            Some(ref item) => {
                Some(&item.val)
            },
        }
    }
}

#[derive(Debug)]
struct Ship<T> {
    stacks: Vec<Stack<T>>,
}

impl Ship<char> {
    fn add_row(&mut self, row: Vec<char>) {
        if self.stacks.len() == 0 {
            for i in 0..(row.len()+2)/4 {
                self.stacks.insert(i, Stack::new());
            }
        }
        for (i, val) in row.into_iter().step_by(4).enumerate() {
            if val != ' ' {
                let stack = self.stacks.get_mut(i).unwrap();
                stack.append(Box::new(Node::new(val)));
            }
        }
    }

    fn move_crates(&mut self, count: usize, from: usize, to: usize) {
        let src = self.stacks.get_mut(from).unwrap();
        let mut crates = Vec::with_capacity(count);
        for _ in 0..count {
            // part 1
            // crates.push(src.pop().expect("Stack should not be empty when moving crates"));
            // part 2
            crates.insert(0, src.pop().expect("Stack should not be empty when moving crates"));
        }
        let dst = self.stacks.get_mut(to).unwrap();
        for c in crates {
            dst.push(c);
        }
    }

    fn print_heads(&self) {
        for stack in &self.stacks {
            match stack.peek() {
                None => print!("_"),
                Some(c) => print!("{c}"),
            }
        }
        print!("\n");
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut ship: Ship<char> = Ship {stacks: Vec::new()};
    let mut recv_commands = false;
    for l in stdin.lines().map(|l| l.unwrap()) {
        if !recv_commands {
            let mut chars = l.chars().peekable();
            match chars.next() {
                Some('[') => ship.add_row(chars.collect()),
                Some(' ') => match chars.peek() {
                    Some('1'..='9') => recv_commands = true,
                    _ => ship.add_row(chars.collect()),
                },
                _ => panic!("Fuck"),
            };
        } else {
            let command: Vec<&str> = l.split(' ').collect();
            if command.len() < 6 {
                continue;
            }
            let count: usize = command.get(1).unwrap().parse().unwrap();
            let from: usize = command.get(3).unwrap().parse().unwrap();
            let to: usize = command.get(5).unwrap().parse().unwrap();
            ship.move_crates(count, from-1, to-1);
        }
        //println!("Current ship {:#?}", ship);
        ship.print_heads();
    }
}
