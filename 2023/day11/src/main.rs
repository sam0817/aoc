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

fn part1(content: &str) {
    let mut data = handle_data(content);
    let max_x = data.iter().map(|(x, _)| x).max().unwrap().clone();
    let max_y = data.iter().map(|(_, y)| y).max().unwrap().clone();
    println!("max_x: {}, max_y: {}", max_x, max_y);
    for y in 1..=max_y {
        let dy = max_y - y + 1;
        let is_empty = !data.iter().any(|(_, y)| y == &dy);
        if is_empty {
            for exp in data.iter_mut().filter(|(_, y)| y > &dy) {
                exp.1 += 1;
            }
        }
    }
    for x in 1..=max_x {
        let dx = max_x - x + 1;
        let is_empty = !data.iter().any(|(x, _)| x == &dx);
        if is_empty {
            for exp in data.iter_mut().filter(|(x, _)| x > &dx) {
                exp.0 += 1;
            }
        }
    }
    // println!("data: {:?}", data);
    let mut sum : usize = 0;
    for i in 0..data.len() {
        for j in i..data.len() {
            if i == j {
                continue;
            }
            let p1 = data[i];
            let p2 = data[j];
            let mut dx = p1.0 - p2.0;
            let mut dy = p1.1 - p2.1;
            if dx < 0 { dx = -dx; }
            if dy < 0 { dy = -dy; }
            sum += (dx + dy) as usize;
        }
    }
    println!("sum: {}", sum)
}

fn handle_data(content: &str) -> Vec<(i32, i32)> {
    let mut data = Vec::new();
    let mut line_no = 0;
    for line in content.lines() {
        line_no += 1;
        let mut position = 0;
        line.chars().enumerate().for_each(|(i, c)| {
            position += 1;
            if c == '#' {
                data.push((line_no, position));
            }
        });
    }
    data
}


fn part2(content: &str) {
    let data = handle_data(content);
    let max_x = data.iter().map(|(x, _)| x).max().unwrap().clone();
    let max_y = data.iter().map(|(_, y)| y).max().unwrap().clone();
    println!("max_x: {}, max_y: {}", max_x, max_y);

    let mut exp_y = Vec::new();
    let mut exp_x = Vec::new();
    for dy in 1..=max_y {
        let is_empty = !data.iter().any(|(_, y)| y == &dy);
        if is_empty { exp_y.push(dy); }
    }
    for dx in 1..=max_x {
        let is_empty = data.iter().all(|(x, _)| x != &dx);
        // println!("x: {}, is_empty: {}", x, is_empty);
        if is_empty { exp_x.push(dx); }
    }
    println!("data: {:?}", data);
    println!("exp_x: {:?}, exp_y: {:?}", exp_x, exp_y);
    let mut sum : usize = 0;
    // let times = 1000000;
    let times = 1000000;
    for i in 0..data.len() {
        for j in i..data.len() {
            if i == j {
                continue;
            }
            let p1 = data[i];
            let p2 = data[j];
            let diff = diff(p1, p2, exp_x.clone(), exp_y.clone(), times);
            sum += diff as usize;
        }
    }
    println!("sum: {}", sum)
}

fn diff(p1: (i32, i32), p2: (i32, i32), ex_x: Vec<i32>, ex_y: Vec<i32>, times: i32) -> i32 {
    steps(p1.0, p2.0, ex_x.clone(), times) + steps(p1.1, p2.1, ex_y.clone(), times)
}

fn steps(x1: i32, x2: i32, exp: Vec<i32>, times: i32) -> i32 {
    let mut steps = 0;
    let x_min = x1.min(x2);
    let x_max = x1.max(x2);
    for x in x_min..=x_max {
        if exp.contains(&x) {
            steps += times;
        } else {
            steps += 1;
        }
    }
    return steps - 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    // (6,2) -> (10, 5)
    // [4,8] [3,6,9]
    #[test]
    fn test_part2_steps() {
        let res = steps(6, 10, vec![4, 8], 1);
        assert_eq!(res, 4);
        let res = steps(6, 10, vec![4, 8], 2);
        assert_eq!(res, 5);
        let res = steps(2, 5, vec![3, 6, 9], 1);
        assert_eq!(res, 3);
        let res = steps(2, 5, vec![3, 6, 9], 2);
        assert_eq!(res, 4);

        let res = diff((6,2), (10,5), vec![4,8], vec![3,6,9], 1);
        assert_eq!(res, 7);
        let res = diff((6,2), (10,5), vec![4,8], vec![3,6,9], 2);
        assert_eq!(res, 9);
    }
}