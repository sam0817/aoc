use std::collections::{HashMap, HashSet};
use std::fs;

pub mod data;

fn main() {
    let contents = fs::read_to_string("input")
    // let contents = fs::read_to_string("example")
        .expect("Should have been able to read the file");

    println!("---------- part1 ----------");
    // part 1
    part1(&contents[..]);
    let mut y = 1;
    let xx = vec![22411,16271,20569,14429,24253,18727];
    xx.iter().for_each(|x| {
         y = lcm(y, *x);
    });
    println!("x: {}", y);
    // println!("---------- part2 ----------");
    // // part 2
    // part2(&contents[..]);
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

    // println!("instructions: {:?}", instructions);
    let mut step = 0;
    // 22411 16271 20569 14429 24253 18727
    // GQA", "GVA", "NVA", "HBA", "XCA", "AAA"
    let mut position = String::from("GQA");
    'label: loop {
        instructions.iter().for_each(|c| {
            let node = nodes.get(&position).unwrap();
            match c {
                'L' => {
                    step += 1;
                    // println!("{}: {} -> {}", step, position, node.0);
                    position = node.0.to_string();
                },
                'R' =>{
                    step += 1;
                    // println!("{}: {} -> {}", step, position, node.1);
                    position = node.1.to_string();
                },
                _ => {},
            }
            if position.as_bytes()[2] == 'Z' as u8 { return; }
        });
        if position.as_bytes()[2] == 'Z' as u8 { break 'label; }
    }
    println!("step: {}", step);
}

fn part2(content: &str) {
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
    });


    // println!("instructions: {:?}", instructions);
    let mut step = 0;
    // let mut position = String::from("AAA");
    let mut positions =Vec::<String>::new();
    nodes.iter().for_each(|(k, _)| {
        if k.as_bytes()[2] == 'A' as u8 {
            positions.push(k.to_string());
        }
    });
    // GQA", "GVA", "NVA", "HBA", "XCA", "AAA"

    println!("positions: {:?}", positions);
    return;
    'label: loop {
        instructions.iter().for_each(|c| {
            // let node = nodes.get(&position).unwrap();
            step += 1;
            match c {
                'L' => {
                    // positions.iter_mut().for_each(|p| {
                    //     let (next, _) = nodes.get(&p.to_string()).unwrap();
                    //     *p = next.to_string();
                    // });
                },
                'R' =>{
                    // positions.iter_mut().for_each(|p| {
                    //     let (_, next) = nodes.get(&p.to_string()).unwrap();
                    //     *p = next.to_string();
                    // });
                },
                _ => {},
            }
            println!("{}", step);

            if positions.iter().all(|p| p.as_bytes()[2] == 'Z' as u8) { return; }
        });
        if positions.iter().all(|p| p.as_bytes()[2] == 'Z' as u8) { break 'label; }
        // if position == "ZZZ" { break 'label; }
    }
    println!("step: {}", step);

}


fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}