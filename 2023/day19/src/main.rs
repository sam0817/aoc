use std::collections::{BTreeMap, HashMap};
use std::fs;

fn main() {
    let contents = fs::read_to_string("input")
    // let contents = fs::read_to_string("example")
        .expect("Should have been able to read the file");

    println!("---------- part1 ----------");
    // part 1
    part1(&contents[..]);

    println!("---------- part2 ----------");
    // part 2 167409079868000
    part2(&contents[..]);
}


fn parse_data(content: &str) -> (BTreeMap<String, WorkFlow>, Vec<HashMap<PartType, usize>>) {
    let mut workflows = BTreeMap::<String, WorkFlow>::new();
    let mut parts = Vec::<HashMap<PartType, usize>>::new();
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
            let (name, flow) = parse_flow(line);
            workflows.insert(name, flow);
        }
    });
    (workflows, parts)
}

fn parse_flow(data: &str) -> (String, WorkFlow) {
    let split = data.split("{").collect::<Vec<&str>>();
    let key = split[0].trim().to_string();
    let mut split_value = split[1][0..(split[1].len() - 1)].split(",").collect::<Vec<&str>>();
    let mut criteria = Vec::<(PartType, CompareType, usize, String)>::new();
    let else_next = split_value[split_value.len() - 1].to_string();
    split_value.remove(split_value.len() - 1);
    split_value.iter().for_each(|s| {
        let split = s.split(":").collect::<Vec<&str>>();
        let condition = split[0].trim();
        let dest = split[1].trim().to_string();
        let gt = &condition.split(">").collect::<Vec<&str>>();
        if gt.len() > 1 {
            let part_type = match gt[0].trim() {
                "x" => PartType::X,
                "m" => PartType::M,
                "a" => PartType::A,
                "s" => PartType::S,
                _ => PartType::X,
            };
            let compare_value = gt[1].trim().parse::<usize>().unwrap();
            criteria.push((part_type, CompareType::Greater, compare_value, dest));
            return;
        }
        let lt = &condition.split("<").collect::<Vec<&str>>();
        if lt.len() > 1 {
            let part_type = match lt[0].trim() {
                "x" => PartType::X,
                "m" => PartType::M,
                "a" => PartType::A,
                "s" => PartType::S,
                _ => PartType::X,
            };
            let compare_value = lt[1].trim().parse::<usize>().unwrap();
            criteria.push((part_type, CompareType::Less, compare_value, dest));
            return;
        }
    });
    (key, WorkFlow { criteria, else_next })
}

#[derive(Debug, Clone)]
struct WorkFlow {
    criteria: Vec<(PartType, CompareType, usize, String)>,
    else_next: String,
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
pub enum CompareType {
    Less,
    Greater,
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
    let mut sum = HashMap::<PartType, usize>::new();
    println!("{:?}", map);
    println!("{:?}", parts);
    for part in parts {
        let t1 = test_parts("in".to_string(), &map, &part);
        if t1 {
            part.iter().for_each(|(k, v)| {
                // println!("{:?}: {}", k, v);
                let entry = sum.entry(*k).or_insert(0);
                *entry += v;
            })
        }
    }
    println!("{:?}", sum);
    let result = sum.iter().fold(0, |acc, (k, v)| {
        acc + v
    });
    println!("result: {}", result);
}

fn test_parts(start: String, map: &BTreeMap<String, WorkFlow>, parts: &HashMap<PartType, usize>) -> bool {
    let flow = map.get(&start).unwrap();
    let mut test_next = "";
    for (pt, comp, count, next) in flow.criteria.iter() {
        let test = match comp {
            CompareType::Less => {
                parts.get(pt).unwrap() < count
            } CompareType::Greater => {
                parts.get(pt).unwrap() > count
            }
        };
        if test {
            test_next = next.as_str();
            break;
        }
    }
    if test_next == "" { test_next = flow.else_next.as_str(); }

    match test_next {
        "A" => { true }
        "R" => { false }
        _ => test_parts(test_next.to_string(), map, parts)
    }
}


fn part2(content: &str) {}