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
    println!("data: {:?}", data);
    let mut sum = 0;
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
            sum += dx + dy;
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


fn part2(content: &str) {}