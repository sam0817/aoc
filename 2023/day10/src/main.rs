use std::collections::HashMap;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input")
    // let contents = fs::read_to_string("example")
        .expect("Should have been able to read the file");

    println!("---------- part1 ----------");
    // part 1
    part1(&contents[..]);

    println!("---------- part2 ----------");
    // part 2
    part2(&contents[..]);
}


fn part1(content: &str) {
    let mut nods = HashMap::<(isize, isize), (bool, bool, bool, bool)>::new();
    let mut s: Node = Default::default();
    handlep1(content).iter().for_each(|x|{
        // (x.val.0, x.val.1, x.top, x.bottom, x.left, x.right)
        if (x.top, x.bottom, x.left, x.right) == (true, true, true, true) {
            s = x.clone();
        }
        nods.insert(x.val, (x.top, x.bottom, x.left, x.right));
    });
    let e = s.val.clone();
    // let s = nodes.iter().find(|x| x.2 && x.3 && x.4 && x.5).unwrap();
    let mut steps = 0;
    loop {
        steps += 1;
        if steps > 1 && s.val == e {
            break;
        }
        println!("{:?}", s.val);
        let right = nods.get_mut(&(s.val.0, s.val.1 + 1));
        if let Some(right) = right {
            if right.2 {
                let last = nods.get_mut(&(s.val.0, s.val.1)).unwrap();
                if last.3 {
                    s.val.1 += 1;
                    last.3 = false;
                    continue;
                }
            }
        }
        let left = nods.get_mut(&(s.val.0, s.val.1 - 1));
        if let Some(left) = left {
            if left.3 {
                let last = nods.get_mut(&(s.val.0, s.val.1)).unwrap();
                if last.2 {
                    s.val.1 -= 1;
                    last.2 = false;
                    continue;
                }
            }
        }
        let top = nods.get_mut(&(s.val.0-1, s.val.1));
        if let Some (top) = top {
            if top.1 {
                let last = nods.get_mut(&(s.val.0, s.val.1)).unwrap();
                if last.0 {
                    s.val.0 -= 1;
                    last.0 = false;
                    continue;
                }
            }
        }
        let bottom = nods.get_mut(&(s.val.0+1, s.val.1));
        if let Some(bottom) = bottom {
            if bottom.0 {
                let last = nods.get_mut(&(s.val.0 , s.val.1)).unwrap();
                if last.1 {
                    s.val.0 += 1;
                    last.1 = false;
                    continue;
                }
            }
        }
        if steps > 10 {
            break;
        }
    }
    steps -= 1;
    println!("{:?}", s);
    println!("steps: {}", steps / 2);
}

fn handlep1(content: &str) -> Vec::<Node> {
    let mut line_no = 0;
    let mut nodes = Vec::new();
    content.lines().for_each(|line| {
        line_no += 1;
        let mut position = 0;
        line.chars().collect::<Vec<char>>()
            .iter()
            .for_each(|x| {
                position += 1;
                let mut node = Node::default();
                node.val = (line_no, position);
                match x {
                    'F' => {
                        node.right = true;
                        node.bottom=true;
                    },
                    '-' => {
                        node.right = true;
                        node.left = true;
                    },
                    '|' => {
                        node.top = true;
                        node.bottom = true;
                    },
                    'L' => {
                        node.top = true;
                        node.right = true;
                    },
                    '7' => {
                        node.bottom = true;
                        node.left = true;
                    },
                    'J' => {
                        node.top = true;
                        node.left = true;
                    },
                    'S' => {
                        node.top = true;
                        node.bottom = true;
                        node.left = true;
                        node.right = true;
                    },
                    _ => {
                        // println!("else");
                    }
                }
                // println!("{:?}", node);
                nodes.push(node);
            });
    });
    nodes
}
// fn first_next(nodes: Vec<Node>) {
//     let right = nodes.iter().find(|n| n.val == (x, y+1)).unwrap();
//     if right.left {
//         steps += 1;
//         y += 1;
//         continue;
//     }
//     let left = nodes.iter().find(|n| n.val == (x, y-1) && n.val != (x1,y1)).unwrap();
//     if left.right {
//         steps += 1;
//         y -= 1;
//         continue;
//     }
//     let top = nodes.iter().find(|n| n.val == (x-1, y) && n.val != (x1,y1)).unwrap();
//     if top.bottom {
//         steps += 1;
//         x -= 1;
//         continue;
//     }
//     let bottom = nodes.iter().find(|n| n.val == (x+1, y) && n.val != (x1,y1)).unwrap();
//     if bottom.top {
//         steps += 1;
//         x += 1;
//         continue;
//     }
// }
// fn next(nodes: &Vec<Node>, x: isize, y: isize, from: char) -> (x,y) {
//     let mut node = nodes.iter().find(|node| {
//          node.val.0 == x && node.val.1 == y
//     }).unwrap().clone();
//     match from {
//         'l' => *node.left = false,
//         'r' => *node.right = false,
//         't' => *node.top = false,
//         'b' => *node.bottom = false,
//         _=>{},
//     }
//     if node.left {
//         return (node.val.0, node.val.1 - 1);
//     }
//     if node.right {
//         return (node.val.0, node.val.1 + 1);
//     }
//     if node.top {
//         return (node.val.0 - 1, node.val.1);
//     }
//     if node.bottom {
//         return (node.val.0 + 1, node.val.1);
//     }
//     node.val
// }

#[derive(Debug, Default, Copy, Clone)]
struct Node {
    val: (isize, isize),
    top: bool,
    bottom: bool,
    left: bool,
    right: bool,
}


fn part2(content: &str) {}