use std::collections::HashMap;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input").expect("Should have been able to read the file");

    // part 1
    part1(&contents[..]);
    // let result: i32 = contents.lines().map(|line| {
    //     part1(line)
    // }).sum();

    // println!("Result: {:?}", symbols);

    // part 2
    let result: i32 = contents
        .lines()
        .map(|line| {
            // part2_sum_bags(line)
            1
        })
        .sum();

    println!("Result: {}", result);
}

fn part1(contents: &str) {
    let (symbols,numbers) = part1_symbols(contents);
}
struct Num {
    num: i32,
    line: i32,
    start: i32,
    end: i32,
}

fn part1_symbols(contents: &str) -> (HashMap<(i32, i32), char> ,Vec<Num>) {
    let mut symbol = HashMap::<(i32, i32), char>::new();
    let mut numbers = Vec::<Num>::new();
    let mut x: i32 = 0;
    for line in contents.lines() {
        x += 1;
        let mut y = 0;
        let mut start = 0;
        let mut end = 0;
        let mut num_stack = String::new();
        for c in line.chars() {
            y += 1;
            match c {
                '.' => {
                    if num_stack.len() > 0 {
                        let num = num_stack.parse::<i32>().unwrap();
                        numbers.push(Num {
                            num,
                            line: x,
                            start,
                            end,
                        });
                    }
                    num_stack.clear();
                }
                '0'..='9' => {
                    if num_stack.len() == 0 {
                      start = y;
                    }
                    num_stack.push(c);
                    end = y;
                }
                _ => {
                    if num_stack.len() > 0 {
                        let num = num_stack.parse::<i32>().unwrap();
                        numbers.push(Num {
                            num,
                            line: x,
                            start,
                            end,
                        });
                    }
                    num_stack.clear();
                    symbol.insert((x, y), c);
                }
            }
        }
    }
    // for num in numbers {
    //     println!("{} {} {} {}", num.num, num.line, num.start, num.end);
    // }
    (symbol, numbers)
}
