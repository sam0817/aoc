use std::collections::HashMap;
use std::fs;

pub mod data;

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
    let mut line_no = 0;
    let mut instructions = Vec::<char>::new();
    let mut nodes = HashMap::<String, (String, String)>::new();
    content.lines().for_each(|line| {
        line_no += 1;
        if line_no == 1 {
            line.chars().for_each(|c| { instructions.push(c); });
            return;
        }
        if line_no == 2 { return; }
        let a = line[0..3].to_string();
        let b = line[7..10].to_string();
        let c = line[12..15].to_string();
        nodes.insert(a, (b, c));
        // println!("{}: {}", line_no, line);
        // println!("{}: {} {} {}", line_no, a, b, c);
        // println!("{}: {:?}", line_no, nodes);
    });

    println!("instructions: {:?}", instructions);
    let mut step = 0;
    let mut position = String::from("AAA");
    'label: loop {
        instructions.iter().for_each(|c| {
            let node = nodes.get(&position).unwrap();
            match c {
                'L' => {
                    step += 1;
                    println!("{}: {} -> {}", step, position, node.0);
                    position = node.0.to_string();
                },
                'R' =>{
                    step += 1;
                    println!("{}: {} -> {}", step, position, node.1);
                    position = node.1.to_string();
                },
                _ => {},
            }
            if position == "ZZZ" { return; }
        });
        if position == "ZZZ" { break 'label; }
    }
    println!("step: {}", step);
}

fn part2(content: &str) {}

