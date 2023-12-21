use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    // let contents = fs::read_to_string("input")
    let contents = fs::read_to_string("example")
        .expect("Should have been able to read the file");
    println!("---------- part1 ----------");
    // 849408534 --> not the right answer, answer is too high
    // 849306997 --> not the right answer, answer is too high
    // part 1
    part1(&contents[..]);

    println!("---------- part2 ----------");
    // part 2
    part2(&contents[..]);
}

fn parse(contents: &str) -> HashMap<(i32, i32), char> {
    let mut result = HashMap::new();
    let mut rows = 0;
    contents
        .lines()
        .for_each(|mut line| {
            rows += 1;
            let mut cols = 0;
            line.chars().for_each(|c| {
                cols += 1;
                result.insert((rows, cols), c);
            });
        });
    result
}

// fn step_1(points: Vec<(i32,i32)>, map: &HashMap<(i32,i32),char>) ->Vec<(i32,i32)> {
fn step_1(points: &HashSet<(i32, i32)>, map: &HashMap<(i32, i32), char>) -> HashSet<(i32, i32)> {
    let mut result = HashSet::<(i32, i32)>::new();
    for point in points.iter() {
        let up = map.get(&(point.0 - 1, point.1));
        if let Some(up) = up {
            if up != &'#' {
                result.insert((point.0 - 1, point.1));
            }
        }
        let down = map.get(&(point.0 + 1, point.1));
        if let Some(down) = down {
            if down != &'#' {
                result.insert((point.0 + 1, point.1));
            }
        }
        let left = map.get(&(point.0, point.1 - 1));
        if let Some(left) = left {
            if left != &'#' {
                result.insert((point.0, point.1 - 1));
            }
        }
        let right = map.get(&(point.0, point.1 + 1));
        if let Some(right) = right {
            if right != &'#' {
                result.insert((point.0, point.1 + 1));
            }
        }
    }

    result
}

fn part1(contents: &str) {
    let mut data = parse(contents);
    let start = data.iter().find(|(k, v)| **v == 'S').unwrap();

    let mut points = HashSet::<(i32, i32)>::new();
    points.insert((start.0.0, start.0.1));
    for i in 0..64 {
        let nexts = step_1(&points, &data);
        // for step in nexts.iter() {
        //     result.insert(*step, ());
        //     // data.remove(step);
        // }

        // println!("{:?}", nexts);
        points = nexts;
    }
    // println!("{:?}", result.len());
    // println!("{:?}", result);
    // println!("{:?}", points);
    println!("{:?}", points.len());
}

fn part2(contents: &str) {
    let mut map = parse(contents);
    let rows = map.keys().map(|k| k.0).max().unwrap();
    let cols = map.keys().map(|k| k.1).max().unwrap();
    let start = map.iter().find(|(k, v)| **v == 'S').unwrap();
    let mut his = HashMap::<(i32, i32), i32>::new();
    let mut points = HashSet::<(i32, i32)>::new();
    points.insert((start.0.0, start.0.1));
    let max_step = 6;
    for i in 0..max_step {
        let nexts = step_inf(&points, &map, rows, cols, i + 1, &mut his);
        points = nexts;
    }
    let even = his.iter().filter(|(k, v)| **v % 2 == 0).collect::<Vec<_>>();
    let odd = his.iter().filter(|(k, v)| **v % 2 == 1).collect::<Vec<_>>();
    println!("{:?}", even);
    println!("{:?}", odd);
    // println!("{:?}", );
    let result = cal_steps(his, max_step);
    println!("{:?}", result);
}

fn cal_steps(his: HashMap<(i32, i32), i32>, step: i32) -> i32 {
    let remain = step % 2;
    his.values().filter(|h| **h % 2 == remain && step >= **h).count() as i32
}

fn step_inf(points: &HashSet<(i32, i32)>,
            map: &HashMap<(i32, i32), char>,
            rows: i32, cols: i32, step: i32,
            his: &mut HashMap<(i32, i32), i32>) -> HashSet<(i32, i32)> {
    let mut result = HashSet::<(i32, i32)>::new();
    for point in points.iter() {
        let p = vec![
            (point.0 - 1, point.1),
            (point.0 + 1, point.1),
            (point.0, point.1 - 1),
            (point.0, point.1 + 1)];

        p.iter().for_each(|k| {
            let (mut r, mut c) = *k;
            while r < 1 { r += rows; }
            while r > rows { r -= rows; }
            while c < 1 { c += cols; }
            while c > cols { c -= cols; }

            let next = map.get(&(r, c));
            if let Some(next) = next {
                if next != &'#' {
                    let h = his.get(&(r, c));
                    if h.is_none() {
                        his.insert((k.0, k.1), step);
                        result.insert((k.0, k.1));
                    }
                }
            } else {
                println!("ERR");
            }
        });
    }

    result
}
