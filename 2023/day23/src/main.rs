use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    // let contents = fs::read_to_string("input")
    let contents = fs::read_to_string("example")
        .expect("Should have been able to read the file");

    println!("---------- part1 ----------");
    // part 1
    // part1(&contents[..]);

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
            for s in 0..steps.len() {
                let mut append_path = path.clone();
                append_path.push(steps[s]);
                new_path.push(append_path);
            }
        }
        // println!("steps: {:?}", new_path);
        paths = new_path;
        if i % 1000 == 0 { println!("i: {:?}", i); }
        if i > 2300 { break; }
    }
    // println!("paths: {:?}", paths);
    let mut max = 0;
    paths.iter().for_each(|p| {
        println!("path: {:?}", p.len() - 1);
        max = max.max(p.len() - 1);
    });
    println!("max: {:?}", max);
}

fn near_by_2(map: &HashMap<Point, char>, point: Point, path: &HashSet<Point>) -> Vec<Point> {
    let mut result = Vec::<Point>::new();
    let up = (point.0, point.1 - 1);
    let up_point = map.get(&up);
    if let Some(c) = up_point {
        if !path.contains(&up) {
            result.push(up);
        }
    }
    let down = (point.0, point.1 + 1);
    let down_point = map.get(&down);
    if let Some(c) = down_point {
        if !path.contains(&down) {
            result.push(down);
        }
    }

    let left = (point.0 - 1, point.1);
    let left_point = map.get(&left);
    if let Some(c) = left_point {
        if !path.contains(&left) {
            result.push(left);
        }
    }

    let right = (point.0 + 1, point.1);
    let right_point = map.get(&right);
    if let Some(c) = right_point {
        if !path.contains(&right) {
            result.push(right);
        }
    }

    result
}

fn part2_old(content: &str) {
    let map = parse(content);
    let final_row = map.keys().map(|(x, y)| y).max().unwrap().clone();
    print_map(&map);
    let mut paths = HashMap::<i32, (Vec<Point>, HashSet<Point>)>::new();
    let start = map
        .iter().find(|(k, c)| (**k).1 == 1 && **c == '.').unwrap().0;
    println!("start: {:?}", start);
    let path1_vec = vec![*start];
    let mut path1_set = HashSet::<Point>::new();
    path1_set.insert(*start);
    paths.insert(1, (path1_vec, path1_set));

    let mut i = 0;
    println!("paths: {:?}", paths);
    loop {
        i += 1;
        let max_step = paths.keys().max().unwrap().clone();
        let min_step = paths.keys().min().unwrap().clone();
        let mut new_found = false;
        let mut key_to_remove = vec![];
        for step in min_step..=max_step {
            // println!("step: {:?}", step);
            let mut max_key = paths.keys().max().unwrap().clone();
            let p = paths.get_mut(&step);
            if p.is_none() { continue; }
            let (path, set) = p.unwrap();

            let point = path.last().unwrap();
            let mut steps = near_by_2(&map, *point, &set);
            let mut new_to_add = Vec::<(i32, Vec<Point>, HashSet<Point>)>::new();
            if point.1 != final_row && steps.len() == 0 {
                key_to_remove.push(step.clone());
            }
            for s in 0..steps.len() {
                new_found = true;
                let new_point = steps[s];
                let mut new_path = path.clone();
                let mut new_set = set.clone();
                if s == steps.len() - 1 {
                    (*path).push(new_point);
                    (*set).insert(new_point);
                    continue
                } else {
                    max_key += 1;
                    new_path.push(new_point);
                    new_path.remove(0);
                    new_set.insert(new_point);
                    new_to_add.push((max_key, new_path, new_set));
                }
            }
            for k in key_to_remove.iter() {
                paths.remove(k);
            }
            for (k, p, s) in new_to_add {
                paths.insert(k, (p, s));
            }
        }


        if i % 200 == 0 { println!("i: {:?}", i); }
        if !new_found { break; }
    }

    let mut max = 0;
    paths.iter()
        .filter(|(k, (p, s))| p[p.len() - 1].1 == final_row)
        .for_each(|(k, (p, s))| {
            println!("path: {:?}", p.len() - 1);
            max = max.max(p.len() - 1);
        });
    println!("max: {:?}", max);
}

type Line = (Point, Point, usize);

fn near_by_222(map: &HashMap<Point, char>, point: Point) -> Vec<Point> {
    let diff = vec![(0, -1), (0, 1), (-1, 0), (1, 0)];
    diff.iter().map(|(dx, dy)| {
        (point.0 as i32 + dx, point.1 as i32 + dy)
    }).filter(|(x, y)| {
        map.contains_key(&(*x as usize, *y as usize))
    }).map(|(x, y)| {
        (x as usize, y as usize)
    }).collect::<Vec<Point>>()
}

fn find_lines(map: &HashMap<Point, char>, point: Point) -> Vec<Line> {
    let last_row = map.keys().map(|(x, y)| y).max().unwrap().clone();
    let mut steps = 0;
    let start = point.clone();
    let path = vec![point.clone()];
    let next = near_by_222(&map, point);
    // println!("next: {:?}", next);
    let mut lines = vec![];
    let mut paths = next.iter().map(|p| {
        let mut path = vec![point.clone()];
        path.push(*p);
        path
    }).collect::<Vec<Vec<Point>>>();
    // println!("paths: {:?}", paths);
    loop {
        steps += 1;
        for i in (0..paths.len()).rev() {
            let pth = &mut paths[i];
            let p = pth.last().unwrap();
            let next_step = near_by_222(&map, *p);
            if (*p).1 == last_row {
                // println!("{}: touch last row: {:?}", i, p);
                lines.push((start.clone(), p.clone(), steps));
                paths.remove(i);
                continue;
            }
            if next_step.len() == 0 {
                // println!("{}: no next step: {:?}, {:?}", i, p, pth);
                paths.remove(i);
                continue;
            }
            if next_step.len() > 1 {
                // println!("{}: next step > 1: {:?}", i, p);
                lines.push((start.clone(), p.clone(), steps));
                paths.remove(i);
                continue;
            }
            // println!("{}: keep going: {:?}", i, next_step);
            pth.push(next_step[0]);
        }
        if paths.len() == 0 { break; }
    }
    lines
}

fn convert_to_lines(map: &HashMap<Point, char>) -> HashMap<Point, Vec<Line>> {
    let mut lines_map = HashMap::<Point, Vec<Line>>::new();
    let final_row = map.keys().map(|(x, y)| y).max().unwrap().clone();
    let start = map
        .iter().find(|(k, c)| (**k).1 == 1 && **c == '.').unwrap().0;
    let mut points = vec![*start];
    let mut i = 0;
    loop {
        i += 1;
        let mut new_lines = vec![];
        for p in points {
            let lines = find_lines(&map, p);
            new_lines.extend(lines.clone());
            lines_map.insert(p, lines);
        }
        points = new_lines.iter()
            .map(|(_, end_p, _)| end_p.clone())
            .filter(|p| lines_map.get(p).is_none())
            .collect::<Vec<Point>>();
        if points.len() == 0 { break; }
    }
    lines_map
}

fn print_line_map(map: &HashMap<Point, Vec<Line>>) {
    for (k, v) in map {
        print!("{:?}: ", k);
        v.iter().for_each(|(start, end, steps)| {
            print!("-> {:?} {}, ", end, steps);
        });
        // println!();
    }
}

fn part2(content: &str) {
    let map = parse(content);
    let start = map.iter().find(|(k, c)| (**k).1 == 1 && **c == '.').unwrap().0;
    let lines_map = convert_to_lines(&map);
    print_map(&map);
    let mut paths = Vec::<Vec<Point>>::new();
    paths.push(vec![*start]);
    let mut i = 0;
    loop {
        let mut new_paths = Vec::<Vec<Point>>::new();
        let mut new_found = false;
        for path in paths {
            let last = path.last().unwrap();

            if lines_map.contains_key(last) {
                let lines = lines_map.get(last).unwrap();
                // println!("lines: {:?}", lines);
                if lines.len() == 0 {
                    new_paths.push(path.clone());
                    continue;
                }
                for line in lines {
                    new_found = true;
                    let next = line.1.clone();
                    // println!("next: {:?}", next);
                    if path.iter().all(|p| *p != next) {
                        let mut new_path = path.clone();
                        new_path.push(next);
                        new_paths.push(new_path);
                    // } else {
                        // new_paths.push(path.clone());
                    }
                }
            } else {
                new_paths.push(path.clone());
            }
        }
        paths = new_paths;
        i += 1;
        println!("i: {:?}", i);
        // println!("paths: {} {:?}", paths.len(), paths);
        if i > 200 { break; }
        if !new_found { break; }
    }
    println!("paths: {} {:?}", paths.len(), paths);
    paths.iter().for_each(|p| {
        let mut steps = 0;
        for i in 1..p.len() {
            let p1 = p[i - 1];
            let p2 = p[i];
            let step = lines_map.get(&p1).unwrap()
                .iter().find(|(_, end, steps)| {
                *end == p2
            }).unwrap().2;
            steps += step;
            // println!("{:?} -> {:?} step: {:?}", p1, p2, step);
        }
        println!("steps: {:?}", steps);
    });

    // print_line_map(&lines_map);
    let lines = find_lines(&map, (20, 20));
    // println!("lines: {:?}", lines);

    let ne = near_by_222(&map, (16, 20));
    // println!("ne: {:?}", ne);
}