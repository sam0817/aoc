use std::collections::HashMap;
use std::fs;
use crate::Pulse::{High, Low};

fn main() {
    // let contents = fs::read_to_string("input")
    let contents = fs::read_to_string("example2")
        .expect("Should have been able to read the file");

    println!("---------- part1 ----------");
    // part 1
    part1(&contents[..]);

    println!("---------- part2 ----------");
    // part 2
    part2(&contents[..]);
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
enum NodeType {
    /// are either on or off; they are initially off. If a flip-flop module receives a high pulse,
    /// it is ignored and nothing happens. However, if a flip-flop module receives a low pulse,
    /// it flips between on and off. If it was off, it turns on and sends a high pulse.
    /// If it was on, it turns off and sends a low pulse.
    FlipFlop(bool),
    /// remember the type of the most recent pulse received from each of their connected input modules;
    /// they initially default to remembering a low pulse for each input.
    /// When a pulse is received, the conjunction module first updates its memory for that input.
    /// Then, if it remembers high pulses for all inputs, it sends a low pulse; otherwise, it sends a high pulse.
    Conjunction(Vec<(String, Pulse)>),
    /// receives a pulse, it sends the same pulse to all of its destination modules.
    Broadcast,
    Final,
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug, Clone)]
struct Node {
    name: String,
    node_type: NodeType,
    receivers: Vec<String>,
    messages: Vec<(String, Pulse)>,
    messages_history: HashMap<String, Vec<Pulse>>,
}

impl Node {
    fn new(name: String, node_type: NodeType) -> Self {
        Node {
            name,
            node_type,
            receivers: vec![],
            messages: vec![],
            messages_history: HashMap::new(),
        }
    }

    fn receive(&mut self, pulse: Pulse, from: String) {
        self.messages.push((from, pulse))
    }

    fn send(&mut self) -> Vec<(String, Pulse, String)> {
        let pulse = self.messages.first().unwrap().clone();
        self.messages.remove(0);
        self.messages_history.entry(pulse.0.clone()).or_insert(vec![]).push(pulse.1);
        match &self.node_type {
            NodeType::FlipFlop(status) => {
                if pulse.1 == Low {
                    let status = !status;
                    self.node_type = NodeType::FlipFlop(status);
                    let p = if status { High } else { Low };
                    return self.receivers.iter()
                        .map(|r| (r.clone(), p, self.name.clone()))
                        .collect::<Vec<(String, Pulse, String)>>();
                }
            }
            NodeType::Conjunction(_) => {
                let last = self.messages_history.values().map(|v| v.last().unwrap()).collect::<Vec<&Pulse>>();
                return if last.iter().any(|p| p != &&High) {
                    self.receivers.iter().map(|r| (r.clone(), High, self.name.clone())).collect::<Vec<(String, Pulse, String)>>()
                } else {
                    self.receivers.iter().map(|r| (r.clone(), Low, self.name.clone())).collect::<Vec<(String, Pulse, String)>>()
                };
            }
            NodeType::Broadcast => {
                return self.receivers.iter().map(|r| (r.clone(), pulse.1, self.name.clone())).collect::<Vec<(String, Pulse, String)>>();
            }
            NodeType::Final => {
                return vec![];
            }
        }
        vec![]
    }
}

fn parse_data(content: &str) -> Vec<Node> {
    let mut nodes = vec![];
    content.lines().for_each(|line| {
        let s = line.split(" -> ").collect::<Vec<&str>>();
        let sender = s[0];
        let mut node = match sender.chars().nth(0).unwrap() {
            '%' => Node::new(sender[1..].to_string(), NodeType::FlipFlop(false)),
            '&' => Node::new(sender[1..].to_string(), NodeType::Conjunction(vec![])),
            _ => Node::new(sender.to_string(), NodeType::Broadcast),
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
    let mut nodes = parse_data(content);
    // first round
    let broadcast = nodes.iter_mut().find(|n| n.node_type == NodeType::Broadcast).unwrap();
    // let receivers = broadcast.receivers.clone();
    broadcast.receive(Low, "button".to_string());
    let first_round = nodes.iter_mut().map(|n| {
        if n.messages.len() > 0 { n.send() } else { vec![] }
    }).collect::<Vec<_>>();
    first_round.iter().for_each(|r| {
        r.iter().for_each(|(to, pulse, from)| {
            let node = nodes.iter_mut().find(|n| n.name == *to).unwrap();
            node.receive(*pulse, from.to_string());
        })
    });

    println!("nodes: {:?}", nodes);
    println!("next: {:?}", first_round);
    let mut round = nodes.iter_mut().map(|n| {
        if n.messages.len() > 0 { n.send() } else { vec![] }
    }).collect::<Vec<_>>();

    loop {
        println!("next: {:?}", round);
        round.iter().for_each(|r| {
            r.iter().for_each(|(to, pulse, from)| {
                let node = nodes.iter_mut().find(|n| n.name == *to);
                if let Some(nd) = node {
                    nd.receive(*pulse, from.to_string());
                } else {
                    nodes.push(Node::new(to.to_string(), NodeType::Final));
                }
            })
        });
        round = nodes.iter_mut().map(|n| {
            if n.messages.len() > 0 { n.send() } else { vec![] }
        }).collect::<Vec<_>>();
        if round.iter().all(|r| r.len() == 0) {
            break;
        }
    }

    let all_pulse = nodes.iter()
        .flat_map(|n| n.messages_history.values())
        .flat_map(|v| v)
        .collect::<Vec<_>>();
    let high = all_pulse.iter().filter(|p| ***p == High).count();
    let low = all_pulse.iter().filter(|p| ***p == Low).count();
    println!("-----------final----------");
    println!("nodes: {:?}", nodes);
    println!("nodes: {:?}", round);
    // println!("all pulse: {:?} ", all_pulse);
    println!("all pulse: {:?} H: {}, L: {}", all_pulse.len(), high, low);
    // println!("all pulse: {:?}", all_pulse.iter().filter_map(|v| *v).sum::<usize>());
}

fn part2(content: &str) {}