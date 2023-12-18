use std::cmp::min;
use std::collections::HashMap;
use std::fs;

fn main() {
    // let contents = fs::read_to_string("input")
        let contents = fs::read_to_string("example")
        .expect("Should have been able to read the file");

    println!("---------- part1 ----------");
    // part 1
    part1(&contents[..]);
    // 70260 --> not right
    println!("---------- part2 ----------");
    // part 2
    part2(&contents[..]);
}

fn parse_data(content: &str) -> Vec<Step> {
    let mut data: Vec<Step> = Vec::new();
    for line in content.lines() {
        let s = line.split(" ").collect::<Vec<&str>>();
        let dir = match s[0] {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Unknown direction"),
        };
        let steps = s[1].parse::<i32>().unwrap();
        let color = s[2].to_string();
        data.push(Step { dir, steps, color });
    }
    data
}

fn part1(content: &str) {
    let data = parse_data(content);
    let mut map: Vec<Point> = Vec::new();
    let mut start = Point { r: 0, c: 0 };
    let mut fill = HashMap::<Point, i32>::new();

    for step in &data {
        match step.dir {
            Direction::Up => {
                for _ in 0..step.steps {
                    start.r -= 1;
                    map.push(start.clone());
                }
            }
            Direction::Down => {
                for _ in 0..step.steps {
                    start.r += 1;
                    map.push(start.clone());
                }
            }
            Direction::Left => {
                for _ in 0..step.steps {
                    start.c -= 1;
                    map.push(start.clone());
                }
            }
            Direction::Right => {
                for _ in 0..step.steps {
                    start.c += 1;
                    map.push(start.clone());
                }
            }
        }
    }

    let mut plot = Vec::<char>::new();
    let min_r = map.iter().min_by_key(|p| p.r).unwrap().r;
    let max_r = map.iter().max_by_key(|p| p.r).unwrap().r;
    let min_c = map.iter().min_by_key(|p| p.c).unwrap().c;
    let max_c = map.iter().max_by_key(|p| p.c).unwrap().c;
    // let is_out = false;
    for r in min_r..=max_r {
        let mut last = (r, min_c);
        let mut is_line = false;
        let mut is_last_line = false;
        let mut is_edge = false;
        let mut is_last_edge = false;
        let mut is_inner = false;
        for c in min_c..=max_c {
            is_edge = map.iter().any(|p| p.r == r && p.c == c);

            // print!("{}/{} ", r, c);
            // print!("{}/{} ", is_edge, is_last_edge);
            if is_edge && !is_last_edge {
                is_line = !is_line;
            }
            if !is_edge && is_last_edge {
                is_line = !is_line;
            }


            if is_line && !is_last_line {
                is_inner = !is_inner;
            }
            if is_line {
                // fill.insert(Point { r, c }, 1);
                plot.push('#');
                // print!("#");
            } else if is_inner {
                // fill.insert(Point { r, c }, 2);
                plot.push('O');
                // print!("O");
            } else {
                // fill.insert(Point { r, c }, 0);
                plot.push('.');
                // print!(".");
            }
            last = (r, c);
            is_last_edge = is_edge;
            is_last_line = is_line;
        }
        // println!()
    }
    // println!("data: {:?}", data);
    println!("data: {:?}", map);
    let count = fill.iter().filter(|(_, v)| **v != 0).count();
    // let max_r = fill.iter().max_by_key(|(k, _)| k.r).unwrap().0.r;
    // let min_r = fill.iter().min_by_key(|(k, _)| k.r).unwrap().0.r;
    // let max_c = fill.iter().max_by_key(|(k, _)| k.c).unwrap().0.c;
    // let min_c = fill.iter().min_by_key(|(k, _)| k.c).unwrap().0.c;
    println!(" ({},{}) - ({},{})", min_r, min_c, max_r, max_c);
    println!("count: {:?}", plot.len());
    println!("count: {:?}", plot.iter().filter(|&c| *c == '#').count());
    println!("count: {:?}", plot.iter().filter(|&c| *c == 'O').count());
    println!("count: {:?}", plot.iter().filter(|&c| *c == '.').count());
}

fn part2(content: &str) {}

#[derive(Debug, Clone)]
struct Step {
    dir: Direction,
    steps: i32,
    color: String,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    r: i32,
    c: i32,
}