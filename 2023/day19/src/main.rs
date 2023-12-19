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


fn parse_data(content: &str) -> (HashMap<String, String>, Vec<HashMap<PartType,usize>>) {
    let mut workflows = HashMap::<String, String>::new();
    let mut parts = Vec::<HashMap<PartType,usize>>::new();
    let mut flow_finished = false;
    content.lines().for_each(|line| {
        if (line.len() == 0) {
            flow_finished = true;
            return;
        }
        if flow_finished {
            let part = parse_parts(line);
            parts.push(part);
        } else {
            let workflow = line.split("{").collect::<Vec<&str>>();
            workflows.insert(workflow[0].to_string(), workflow[1].to_string());
        }
    });
    (workflows, parts)
}

fn parse_flow(str: &str) -> Vec<String> {
    let mut result = Vec::<String>::new();
    // str.split("->").for_each(|s| {
    //     result.push(s.trim().to_string());
    // });
    result
}

fn parse_parts(data: &str) -> HashMap<PartType, usize> {
    let mut result = HashMap::<PartType, usize>::new();
    let line = data[1..(data.len()) - 1].split(",").collect::<Vec<&str>>();
    line.iter().for_each(|s| {
        let split = s.split("=").collect::<Vec<&str>>();
        let key = split[0].trim();
        let value = split[1].trim().parse::<usize>().unwrap();
        match key {
            "x" => {
                result.insert(PartType::X, value);
            }
            "m" => {
                result.insert(PartType::M, value);
            }
            "a" => {
                result.insert(PartType::A, value);
            }
            "s" => {
                result.insert(PartType::S, value);
            }
            _ => {}
        }
    });
    result
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PartType {
    X,
    M,
    A,
    S,
}

fn part1(content: &str) {
    let (map, parts) = parse_data(content);
    println!("{:?}", map);
    println!("{:?}", parts);
}

fn part2(content: &str) {}