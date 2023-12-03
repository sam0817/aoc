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
    let mut sum = 0;
    let (symbol, numbers) = part1_symbols(contents);
    println!("{:?}",symbol);
    numbers.iter().for_each(|num| {
        for position in (num.start - 1)..=(num.end + 1) {
            if symbol.contains_key(&(num.line_no - 1, position))
                || symbol.contains_key(&(num.line_no + 1, position)) {
                println!("{} {} {} {}", num.num, num.line_no, num.start, num.end);
                sum += num.num;
                break;
            }
        }

        if symbol.contains_key(&(num.line_no, num.start - 1))
            || symbol.contains_key(&(num.line_no, num.end + 1)) {
            println!(">{} {} {} {}", num.num, num.line_no, num.start, num.end);
            sum += num.num;
        }
    });
    println!("Result: {}", sum);
}

struct Num {
    num: i32,
    line_no: i32,
    start: i32,
    end: i32,
}

fn part1_symbols(contents: &str) -> (HashMap<(i32, i32), char>, Vec<Num>) {
    let mut symbol = HashMap::<(i32, i32), char>::new();
    let mut numbers = Vec::<Num>::new();
    let mut line_no: i32 = 0;
    for line in contents.lines() {
        line_no += 1;
        let mut position = 0;
        let mut start = 0;
        let mut end = 0;
        let mut num_stack = String::new();
        for c in line.chars() {
            position += 1;
            match c {
                '.' => {
                    if num_stack.len() > 0 {
                        let num = num_stack.parse::<i32>().unwrap();
                        numbers.push(Num {
                            num,
                            line_no,
                            start,
                            end,
                        });
                    }
                    num_stack.clear();
                }
                '0'..='9' => {
                    if num_stack.len() == 0 {
                        start = position;
                    }
                    num_stack.push(c);
                    end = position;
                }
                _ => {
                    if num_stack.len() > 0 {
                        let num = num_stack.parse::<i32>().unwrap();
                        numbers.push(Num {
                            num,
                            line_no,
                            start,
                            end,
                        });
                    }
                    num_stack.clear();
                    symbol.insert((line_no, position), c);
                }
            }
        }
        if num_stack.len() > 0 {
            let num = num_stack.parse::<i32>().unwrap();
            numbers.push(Num {
                num,
                line_no,
                start,
                end,
            });
        }
    }
    // for num in numbers {
    //     println!("{} {} {} {}", num.num, num.line, num.start, num.end);
    // }
    (symbol, numbers)
}
