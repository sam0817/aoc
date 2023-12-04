use std::fs;

fn main() {
    let contents = fs::read_to_string("input").expect("Should have been able to read the file");

    // part 1
    part1(&contents[..]);

    // part 2
    part2(&contents[..]);
}

fn part1(contents: &str) {
    let mut sum = 0;
    for line in contents.lines() {
        sum += split_line(line);
    }
    println!("Result: {}", sum);
}

fn point_fn(i: i32) -> i32 {
    let base: i32 = 2;
    if i == 0 {
        0
    } else if i == 1 {
        1
    } else {
        2 * point_fn(i - 1)
    }
}

fn parse_ints(line: &str) -> Vec<i32> {
    // println!("line:{}", line);
    line.trim().split(" ")
        .collect::<Vec<&str>>()
        .iter()
        .filter(|x| x.len() > 0)
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

fn split_line(line: &str) -> i32 {
    let mut count = 0;
    let split = line.split(":").collect::<Vec<&str>>()[1].trim();
    let mut split = split.split("|").collect::<Vec<&str>>();
    let win_list  = parse_ints(split[0]);
    let owned = parse_ints(split[1]);
    owned.iter().for_each(|x| {
        if win_list.contains(x) {
            count += 1;
        }
    });
    point_fn(count)
}

fn part2(contents: &str) {}
