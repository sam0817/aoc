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
    handlep1(content).iter().for_each(|x| {
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
        // println!("{:?}", s.val);
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
        let top = nods.get_mut(&(s.val.0 - 1, s.val.1));
        if let Some(top) = top {
            if top.1 {
                let last = nods.get_mut(&(s.val.0, s.val.1)).unwrap();
                if last.0 {
                    s.val.0 -= 1;
                    last.0 = false;
                    continue;
                }
            }
        }
        let bottom = nods.get_mut(&(s.val.0 + 1, s.val.1));
        if let Some(bottom) = bottom {
            if bottom.0 {
                let last = nods.get_mut(&(s.val.0, s.val.1)).unwrap();
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
    // println!("{:?}", s);
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
                        node.bottom = true;
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

#[derive(Debug, Default, Copy, Clone)]
struct Node {
    val: (isize, isize),
    top: bool,
    bottom: bool,
    left: bool,
    right: bool,
}


fn part2(content: &str) {
    let mut nods = HashMap::<(isize, isize), (bool, bool, bool, bool)>::new();
    let mut s: Node = Default::default();
    handlep1(content).iter().for_each(|x| {
        // (x.val.0, x.val.1, x.top, x.bottom, x.left, x.right)
        if (x.top, x.bottom, x.left, x.right) == (true, true, true, true) {
            s = x.clone();
        }
        nods.insert(x.val, (x.top, x.bottom, x.left, x.right));
    });
    let mut nodes_2 = nods.clone();

    let e = s.val.clone();
    let mut path = Vec::<(isize, isize)>::new();
    let mut steps = 0;
    loop {
        steps += 1;
        path.push(s.val.clone());
        if steps > 1 && s.val == e {
            break;
        }
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
        let top = nods.get_mut(&(s.val.0 - 1, s.val.1));
        if let Some(top) = top {
            if top.1 {
                let last = nods.get_mut(&(s.val.0, s.val.1)).unwrap();
                if last.0 {
                    s.val.0 -= 1;
                    last.0 = false;
                    continue;
                }
            }
        }
        let bottom = nods.get_mut(&(s.val.0 + 1, s.val.1));
        if let Some(bottom) = bottom {
            if bottom.0 {
                let last = nods.get_mut(&(s.val.0, s.val.1)).unwrap();
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
    println!("{:?}", s);
    println!("{:?}, {:?}, {:?}", path[0], path[1], path[path.len() - 2]);
    let s_2 = nodes_2.get_mut(&s.val.clone()).unwrap();
    s_2.0 = false;
    s_2.1 = false;
    s_2.2 = false;
    s_2.3 = false;
    let p1 = (path[1].0 - path[0].0, path[1].1 - path[0].1);
    let p2 = (path[path.len() - 2].0 - path[0].0, path[path.len() - 2].1 - path[0].1);
    match p1 {
        (0, 1) => {
            s_2.3 = true;
        },
        (0, -1) => {
            s_2.2 = true;
        },
        (1, 0) => {
            s_2.1 = true;
        },
        (-1, 0) => {
            s_2.0 = true;
        },
        _ => {},
    }
    match p2 {
        (0, 1) => {
            s_2.3 = true;
        },
        (0, -1) => {
            s_2.2 = true;
        },
        (1, 0) => {
            s_2.1 = true;
        },
        (-1, 0) => {
            s_2.0 = true;
        },
        _ => {},
    }
    println!("{:?}", s_2);


    let max_x = nodes_2.keys().max_by(|a, b|
        a.0.partial_cmp(&b.0).unwrap()
    ).unwrap().clone().0;
    let max_y = nodes_2.keys().max_by(|a, b|
        a.1.partial_cmp(&b.1).unwrap()
    ).unwrap().clone().1;

    let mut count = 0;
    for x in 1..=(max_x) {
        let mut stack = 0;
        let mut is_open = false;
        let mut  open_tag = ' ';
        for y in 1..=(max_y) {
            let is_line = path.iter().any(|p| p == &(x, y));
            if is_line {
                let node = nodes_2.get_mut(&(x, y)).unwrap();
                match node {
                    (true, true, false, false) => { // |
                        is_open = !is_open;
                        stack += 1;
                    },
                    (false, true, false, true) => { // F
                        open_tag = 'F';
                    },
                    (true, false, false, true) => { // L
                        open_tag = 'L';
                    },
                    (false, true, true, false) => { // 7
                        if open_tag == 'L' {
                            is_open = !is_open;
                            stack += 1;
                            open_tag = ' ';
                        }
                        open_tag = ' ';
                    },
                    (true, false, true, false) => { // J
                        if open_tag == 'F' {
                            is_open = !is_open;
                            stack += 1;
                        }
                        open_tag = ' ';
                    },
                    (false, false, true, true) => { // -
                    },
                    _ => {},
                }
                // print!("L");
            } else {
                if is_open && !is_line {
                    count +=1;
                }
                // if is_open { print!("I") } else { print!("O") }
            }
        }
        // println!("{}: {}", x, count);
        // println!("{:?}", line);
    }
    println!("count: {}", count);
}