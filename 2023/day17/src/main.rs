use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::fs;

static mut MAX_LEN: i32 = 13;

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

fn parse_map(content: &str) -> Vec<Vec<i32>> {
    let mut map: Vec<Vec<i32>> = Vec::new();
    for line in content.lines() {
        let mut row: Vec<i32> = Vec::new();
        for c in line.chars() {
            let d = c.to_digit(10).unwrap();
            row.push(d as i32)
        }
        map.push(row);
    }
    map
}

fn part1(content: &str) {
    let map = parse_map(content);
    let mut whole_map = HashMap::<(i32, i32), Vec<Path>>::new();
    let rows = map.len();
    let cols = map[0].len();
    let mut start = Path { trace: vec![(0, 0)], cost: 0 };

    println!("{:?}", start);
    let n1 = start.neighbors();
    println!("----round 1----");
    let p2 = n1.iter()
        .map(|(x, y)| {
            println!("({}, {})", x, y);
            let mut p = start.clone();
            p.add(*x, *y, &map);
            p
        })
        .collect::<Vec<_>>();
    let n2 = &p2.iter()
        .flat_map(|p| p.neighbors())
        .collect::<HashSet<_>>();
    for x in &p2 {
        let entry = whole_map.entry(x.current()).or_insert_with(Vec::new);
        if !entry.contains(x) {
            entry.push(x.clone());
        }
    }
    println!("{:?}", p2);
    println!("{:?}", n2);
    println!("{:?}", whole_map);
    println!("----round 2----");
    n2.iter().for_each(|&point| {
        let from = neighbors(point.0, point.1);
        let mut result = Vec::new();
        for &last in from.iter() {
            if let Some(paths) = whole_map.get(&last){
                for path in paths {
                    let mut p = path.clone();
                    p.add(point.0, point.1, &map);
                    if p.current() == point {
                        result.push(p)
                    }
                }
            };
        }

        let entry = whole_map.entry(point).or_insert_with(Vec::new);
        result.iter().for_each(|x| {
            if !entry.contains(x) {
                entry.push(x.clone());
            }
        });
    });

    let n = do_a_round();
    for x in whole_map {
        print!("{:?}: ", x.0);
        x.1.iter().for_each(|p| print!("{}, ", p));
        println!();
    };
}

fn do_a_round(next_points: &HashSet<(i32,i32)>, whole_map: &mut HashMap<(i32, i32), Vec<Path>>
, cost: &Vec<Vec<i32>>
) -> HashSet<(i32,i32)> {
    let mut generated_points = HashSet::new();
    next_points.iter().for_each(|&point| {
        let from = neighbors(point.0, point.1);
        let mut result = Vec::new();
        for &last in from.iter() {
            if let Some(paths) = whole_map.get(&last){
                for path in paths {
                    let mut p = path.clone();
                    p.add(point.0, point.1, &cost);
                    if p.current() == point {
                        result.push(p)
                    }
                }
            };
        }

        let entry = whole_map.entry(point).or_insert_with(Vec::new);
        result.iter().for_each(|x| {
            if !entry.contains(x) {
                entry.push(x.clone());
            }
            generated_points.insert(x.current());
        });
    });

    generated_points
}


fn part2(content: &str) {}

#[derive(Debug, Clone)]
struct Path {
    trace: Vec<(i32, i32)>,
    cost: i32,
}

impl Path {
    fn current(&self) -> (i32, i32) {
        self.trace[self.trace.len() - 1]
    }

    fn last(&self) -> Option<(i32, i32)> {
        if self.trace.len() == 1 {
            return None;
        }
        let p = self.trace[self.trace.len() - 2];
        Some(p)
    }

    fn add(&mut self, x: i32, y: i32, map: &Vec<Vec<i32>>) {
        if !self.is_valid(x, y) {
            return;
        }
        self.trace.push((x, y));
        self.cost += map[x as usize][y as usize];
        if self.trace.len() > 4 {
            self.trace.remove(0);
        }
    }

    fn last4th(&self) -> Option<(i32, i32)> {
        if self.trace.len() < 4 {
            return None;
        }
        let p = self.trace[self.trace.len() - 4];
        Some(p)
    }

    fn neighbors(&self) -> Vec<(i32, i32)> {
        let (r, c) = self.current();
        let mut neighbors = vec![
            (r - 1, c),
            (r + 1, c),
            (r, c - 1),
            (r, c + 1),
        ];
        neighbors.retain(|(x, y)| self.is_valid(*x, *y));
        neighbors
    }

    fn is_valid(&self, r: i32, c: i32) -> bool {
        if r < 0 || c < 0 {
            return false;
        }
        let diff = (r - self.current().0, c - self.current().1);

        if diff.0.pow(2) + diff.1.pow(2) > 1 {
            return false;
        }

        if r >= unsafe { MAX_LEN } || c >= unsafe { MAX_LEN } {
            return false;
        }

        if self.trace.iter().any(|(x, y)| x == &r && y == &c) {
            return false;
        }

        if let Some(l4) = self.last4th() {
            let diff = (r - l4.0, c - l4.1);
            if diff.0 > 3 || diff.0 < -2 || diff.1 > 3 || diff.1 < -2 {
                return false;
            }
        }

        true
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        s.push_str(&format!("Cost: {}, ", self.cost));
        for (r, c) in &self.trace {
            s.push_str(&format!("({}, {}) -> ", r, c));
        }
        write!(f, "{}", s)
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.trace == other.trace
    }
}

fn neighbors(r: i32, c: i32) -> Vec<(i32, i32)> {
    let mut points = vec![
        (r - 1, c),
        (r + 1, c),
        (r, c - 1),
        (r, c + 1),
    ];
    points.retain(|(x, y)| x >= &0 && y >= &0 && x < &unsafe { MAX_LEN } && y < &unsafe { MAX_LEN });
    points
}