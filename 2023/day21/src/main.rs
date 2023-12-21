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

fn parse(contents: &str) -> HashMap<(isize, isize), char> {
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

// fn step_1(points: Vec<(isize,isize)>, map: &HashMap<(isize,isize),char>) ->Vec<(isize,isize)> {
fn step_1(points: &HashSet<(isize, isize)>, map: &HashMap<(isize, isize), char>) -> HashSet<(isize, isize)> {
    let mut result = HashSet::<(isize, isize)>::new();
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

    let mut points = HashSet::<(isize, isize)>::new();
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
    let mut his = HashMap::<(isize, isize), isize>::new();
    let mut points = HashSet::<(isize, isize)>::new();
    // his.insert((start.0.0, start.0.1), 0);
    points.insert((start.0.0, start.0.1));
    let (x0, y0) = start.0.clone();
    println!("{},{}", rows, cols);
    let max_step = 50;
    let remain = (start.0.0 + start.0.1) % 2;
    let mut i = 0;
    loop {
        i += 1;
        let nexts = step_inf(points, &map, rows, cols, i, &mut his);
        // println!("p: {:?}", nexts);
        // println!("h: {:?}", his);
        points = nexts;
        //     .filter(|p| {
        //         let (x, y) = (*p).clone();
        //         !his.contains_key(&(x, y))
        //     })
        //     .map(|p| *p).collect::<HashSet<_>>();
        // his.retain(|k, v| {
        //     (k.0 + k.1) % 2 == remain
        // });
        if i >= (max_step) { break; }
    }
    let empty = map.values().filter(|v| **v !='#').count();
    let fill = his.iter().filter(|(k, v)| {
        let (x, y) = (*k).clone();
        0 < x && x <= rows && 0 < y && y <= cols
    }).collect::<Vec<_>>();
    println!("{:?} {:?}", empty, fill);
    // println!("{} - {:?}", his.len(), his);
    let result = cal_steps(his, max_step);
    println!("{:?}", result);
}

fn cal_steps(his: HashMap<(isize, isize), isize>, step: isize) -> isize {
    let remain = step % 2;
    his.values().filter(|h| **h % 2 == remain && step >= **h).count() as isize
}

fn step_inf(points: HashSet<(isize, isize)>,
            map: &HashMap<(isize, isize), char>,
            rows: isize, cols: isize, step: isize,
            his: &mut HashMap<(isize, isize), isize>) -> HashSet<(isize, isize)> {
    let mut result = HashSet::<(isize, isize)>::new();
    for point in points.iter() {
        let p = vec![
            (point.0 - 1, point.1),
            (point.0 + 1, point.1),
            (point.0, point.1 - 1),
            (point.0, point.1 + 1)];

        p.iter().for_each(|k| {
            let (mut r, mut c) = (*k).clone();
            while r < 1 { r += rows; }
            while r > rows { r -= rows; }
            while c < 1 { c += cols; }
            while c > cols { c -= cols; }

            let next = map.get(&(r, c));
            if let Some(next) = next {
                if next != &'#' {
                    if !his.contains_key(&(k.0, k.1)) {
                        result.insert((k.0, k.1));
                    }
                    his.insert((k.0, k.1), step);
                }
            } else {
                println!("ERR");
            }
        });
    }

    result
}

// fn step_inf2(points: &HashSet<(isize, isize)>,
//              map: &HashMap<(isize, isize), char>,
//              rows: isize, cols: isize, step: isize,
//              his: &mut HashMap<(isize, isize), isize>) -> HashSet<(isize, isize)> {
//     let mut result = HashSet::<(isize, isize)>::new();
//     println!("step: {}", step);
//     for point in points.iter() {
//         let p = vec![
//             (point.0 - 2, point.1),
//             (point.0 + 2, point.1),
//             (point.0 - 1, point.1 - 1),
//             (point.0 - 1, point.1 + 1),
//             (point.0 + 1, point.1 - 1),
//             (point.0 + 1, point.1 + 1),
//             (point.0, point.1 - 2),
//             (point.0, point.1 + 2)];
//
//         p.iter().for_each(|k| {
//             let (mut r, mut c) = (*k).clone();
//             if his.contains_key(&(k.0, k.1)) { return; }
//             while r < 1 { r += rows; }
//             while r > rows { r -= rows; }
//             while c < 1 { c += cols; }
//             while c > cols { c -= cols; }
//
//             let next = map.get(&(r, c));
//             if let Some(next) = next {
//                 if next != &'#' {
//                     if !his.contains_key(&(k.0, k.1)) {
//                         result.insert((k.0, k.1));
//                     }
//                     his.insert((k.0, k.1), step);
//                 }
//             } else {
//                 println!("ERR");
//             }
//         });
//     }
//
//     result
// }
