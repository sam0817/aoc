use std::collections::HashMap;
use std::fs;

fn main() {
    // let contents = fs::read_to_string("input")
    let contents = fs::read_to_string("example")
        .expect("Should have been able to read the file");

    println!("---------- part1 ----------");
    // part 1
    part1(&contents[..]);

    println!("---------- part2 ----------");
    // part 2
    part2(&contents[..]);
}


fn parse_data(content: &str) -> (HashMap<String, String>, Vec<String>) {
    let mut workflows = HashMap::<String, String>::new();
    let mut parts = Vec::<String>::new();
    let mut flow_finished = false;
    content.lines().for_each(|line| {
        if (line.len() == 0) {
            flow_finished = true;
            return;
        }
        if flow_finished {
            parts.push(line.to_string());
        } else {
            let workflow = line.split("{").collect::<Vec<&str>>();
            workflows.insert(workflow[0].to_string(), workflow[1].to_string());
        }
    });
    (workflows, parts)
}

fn part1(content: &str) {
    let (map, parts) = parse_data(content);
    println!("{:?}", map);
    println!("{:?}", parts);
}

fn part2(content: &str) {}