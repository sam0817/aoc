use std::collections::{BTreeMap, HashMap};
use std::fs;

fn main() {
    // let contents = fs::read_to_string("input")
    let contents = fs::read_to_string("example")
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
        if line.len() == 0 {
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
    // println!("{:?}", map);
    // println!("{:?}", parts);
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
            }
            CompareType::Greater => {
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


fn part2(content: &str) {
    let (map, _) = parse_data(content);
    let mut start = vec![vec!["in".to_string()]];
    let mut path = path_finding(&start, &map);
    println!("{:?}", path);
    let mut end_node: Vec<Vec<String>> = Vec::new();
    for _ in 0..100 {
        path = path_finding(&path, &map);
        let mut ends = path.iter()
            .filter(|p| p[p.len() - 1] == "A" || p[p.len() - 1] == "R")
            .cloned()
            .collect::<Vec<Vec<String>>>();
        end_node.append(&mut ends);
        path = path.iter()
            .filter(|p| p[p.len() - 1] != "A" && p[p.len() - 1] != "R")
            .cloned()
            .collect::<Vec<Vec<String>>>();
    }
    // println!("{:?}", path);
    // println!("{:?}", end_node);
    // println!("{:?}", end_node[1]);
    let mut sum = 0;
    &end_node.iter().for_each(|p| {
        let r = path_counting(p, &map);
        if p[p.len() - 1] == "A" {
            sum += r.points()
        }
        println!("{:?} - {:?}", p[p.len()-1], r.points());
    });
    println!("{:?}", sum);
    let p1 = path_counting(&end_node[3], &map);
    println!("{:?} - {:?}", &end_node[3], p1);
    println!("{:?}", p1.points());
}

fn path_counting(path: &Vec<String>, map: &BTreeMap<String, WorkFlow>) -> CriteriaSet {
    let mut result = CriteriaSet::new();
    for i in 1..path.len() {
        let entry = map.get(&path[i - 1]).unwrap();
        let r = entry.get_criteria_for_dest();
        // println!("{:?} - {:?}", &path[i - 1], entry);
        // println!("{:?} - {:?}", &path[0], r);
        let (_, set) = r.iter().find(|(dest, _)| *dest == path[i]).unwrap();
        result = result.combine(set);
    }
    result
}

fn path_finding(node: &Vec<Vec<String>>, map: &BTreeMap<String, WorkFlow>) -> Vec<Vec<String>> {
    let mut new_node: Vec<Vec<String>> = Vec::new();
    node.iter().for_each(|old_path| {
        let node = &old_path[old_path.len() - 1];
        let entry = map.get(node).unwrap();
        // let mut new_path = Vec::<String>::new();
        for (_, _, _, next) in entry.criteria.iter() {
            let mut n = old_path.clone();
            n.push(next.to_string());
            new_node.push(n)
        }
        let mut n = old_path.clone();
        n.push(entry.else_next.to_string());
        new_node.push(n)
    });
    new_node
    // path.push(entry.else_next.to_string());
    // path
}

fn criteria_add() {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Criteria {
    item: PartType,
    max: usize,
    min: usize,
}

impl Criteria {
    fn new(item: PartType) -> Self {
        Criteria {
            item,
            max: 4000,
            min: 1,
        }
    }
    fn up(item: PartType, min: usize) -> Self {
        Criteria {
            item,
            max: 4000,
            min,
        }
    }
    fn down(item: PartType, max: usize) -> Self {
        Criteria {
            item,
            max,
            min: 1,
        }
    }
    fn combine(&self, other: &Self) -> Self {
        Criteria {
            item: self.item,
            max: self.max.min(other.max),
            min: self.min.max(other.min),
        }
    }
}

impl WorkFlow {
    fn get_criteria_for_dest(&self) -> Vec<(String, CriteriaSet)> {
        let mut result = Vec::<(String, CriteriaSet)>::new();
        let mut result_counter = CriteriaSet::new();
        self.criteria.iter().for_each(|(pt, comp, count, next)| {
            let (self_cri, counter) = match comp {
                CompareType::Less => {
                    (Criteria::down(*pt, *count - 1), Criteria::up(*pt, *count))
                }
                CompareType::Greater => {
                    (Criteria::up(*pt, *count + 1), Criteria::down(*pt, *count))
                }
            };
            let mut final_cri = result_counter.clone();
            final_cri.add(self_cri);
            result.push((next.to_string(), final_cri));
            result_counter.add(counter);
        });
        result.push((self.else_next.to_string(), result_counter));

        result
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CriteriaSet {
    criteria: HashMap<PartType, Criteria>,
}

impl CriteriaSet {
    fn new() -> CriteriaSet {
        let mut criteria = HashMap::new();
        criteria.insert(PartType::X, Criteria::new(PartType::X));
        criteria.insert(PartType::A, Criteria::new(PartType::A));
        criteria.insert(PartType::S, Criteria::new(PartType::S));
        criteria.insert(PartType::M, Criteria::new(PartType::M));
        CriteriaSet {
            criteria
        }
    }

    fn add(&mut self, criteria: Criteria) {
        let item = criteria.item;
        let old = self.criteria.get(&item).unwrap();
        let new = old.combine(&criteria);
        self.criteria.insert(item, new);
    }

    fn combine(&self, other: &CriteriaSet) -> CriteriaSet {
        let mut result = CriteriaSet::new();
        self.criteria.iter().for_each(|(k, v)| {
            let other_v = other.criteria.get(k).unwrap();
            let new = v.combine(other_v);
            result.criteria.insert(*k, new);
        });
        result
    }

    fn points(&self) -> usize {
        let mut result = 1;
        self.criteria.iter().for_each(|(_, v)| {
            if v.max < v.min {
                result = 0;
            } else {
                result *= v.max - v.min + 1;
            }
        });
        result
    }
}















