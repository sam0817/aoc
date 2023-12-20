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

#[derive(Debug, Copy, Clone)]
enum NodeType {
    FlipFlop,
    Conjunction,
    Broadcast,
}

#[derive(Debug, Clone)]
struct Node {
    name: String,
    node_type: NodeType,
    receivers: Vec<String>,
}

fn parse_data(content: &str) ->Vec<Node> {
    let mut nodes = vec![];
    content.lines().for_each(|line| {
        let s = line.split(" -> ").collect::<Vec<&str>>();
        let sender = s[0];
        let mut node = match sender.chars().nth(0).unwrap() {
            '%' => {
                let name = sender[1..].to_string();
                Node {
                    name,
                    node_type: NodeType::FlipFlop,
                    receivers: vec![],
                }
            }
            '&' => {
                let name = sender[1..].to_string();
                Node {
                    name,
                    node_type: NodeType::Conjunction,
                    receivers: vec![],
                }
            }
            _ => {
                Node {
                    name: sender.to_string(),
                    node_type: NodeType::Broadcast,
                    receivers: vec![],
                }
            }
        };
        let receivers = s[1].split(", ")
            .map(|r| r.trim().to_string())
            .collect::<Vec<String>>();
        node.receivers = receivers;
        nodes.push(node);
    });
    nodes
}

fn part1(content: &str) {
    parse_data(content);
}

fn part2(content: &str) {}