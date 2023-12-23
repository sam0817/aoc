use std::collections::{HashMap, HashSet};
use std::fs;

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

fn parse(content: &str) -> HashMap<Point, char> {
    let mut result = HashMap::<Point, char>::new();

    content.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c != '#' {
                result.insert((x + 1, y + 1), c);
                // result.insert((x + 1, y + 1), c);
            }
        });
    });
    result
}

fn print_map(map: &HashMap<Point, char>) {
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;

    map.keys().for_each(|(x, y)| {
        min_x = min_x.min(*x);
        max_x = max_x.max(*x);
        min_y = min_y.min(*y);
        max_y = max_y.max(*y);
    });

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            print!("{}", map.get(&(x, y)).unwrap_or(&' '));
        }
        println!();
    }
}

type Point = (usize, usize);

fn near_by(map: &HashMap<Point, char>, point: Point, path: &Vec<Point>) -> Vec<Point> {
    let mut result = Vec::<Point>::new();
    let up = (point.0, point.1 - 1);
    let up_point = map.get(&up);
    if let Some(c) = up_point {
        if *c != 'v' && !path.contains(&up) {
            result.push(up);
        }
    }
    let down = (point.0, point.1 + 1);
    let down_point = map.get(&down);
    if let Some(c) = down_point {
        if *c != '^' && !path.contains(&down) {
            result.push(down);
        }
    }

    let left = (point.0 - 1, point.1);
    let left_point = map.get(&left);
    if let Some(c) = left_point {
        if *c != '>' && !path.contains(&left) {
            result.push(left);
        }
    }

    let right = (point.0 + 1, point.1);
    let right_point = map.get(&right);
    if let Some(c) = right_point {
        if *c != '<' && !path.contains(&right) {
            result.push(right);
        }
    }

    result
}


fn step_next(map: &HashMap<Point, char>, point: Point, path: &Vec<Point>) -> Vec<Point> {
    let near_by = near_by(map, point, path);
    near_by.iter().map(|p| {
        *p
    }).collect::<Vec<Point>>()
}

fn part1(content: &str) {
    let map = parse(content);
    print_map(&map);
    let start = map
        .iter().find(|(k, c)| (**k).1 == 1 && **c == '.').unwrap().0;
    println!("start: {:?}", start);
    let mut p0 = Vec::<Point>::new();
    p0.push(*start);
    let mut paths = vec![p0];
    let mut i = 0;
    loop {
        i += 1;
        let mut new_path = vec![];
        for i in 0..paths.len() {
            let path = &paths[i];
            let point = path.last().unwrap();
            let mut steps = step_next(&map, *point, &path);
            if steps.len() == 0 {
                new_path.push(path.clone());
                continue;
            }
            for s in 0..steps.len(){
                let mut append_path = path.clone();
                append_path.push(steps[s]);
                new_path.push(append_path);
            }
        }
        // println!("steps: {:?}", new_path);
        paths = new_path;
        if i % 1000 == 0 {            println!("i: {:?}", i);}
        if i > 10000 { break; }
    }
    // println!("paths: {:?}", paths);
    let mut max = 0;
    paths.iter().for_each(|p| {
        println!("path: {:?}", p.len()-1);
        max = max.max(p.len()-1);
    });
    println!("max: {:?}", max);
}

fn part2(content: &str) {}