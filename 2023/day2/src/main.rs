use std::fs;
use std::collections::HashMap;

fn main() {
    let contents = fs::read_to_string("input")
        .expect("Should have been able to read the file");

    // part 1
    let result: i32 = contents.lines().map(|line| {
        part1_sum_bags(line)
    }).sum();

    println!("Result: {}", result);

    // part 2
    let result: i32 = contents.lines().map(|line| {
        part2_sum_bags(line)
    }).sum();

    println!("Result: {}", result);
}

pub fn part1_check_requirement(bag: HashMap<String, i32>) -> bool {
    if bag.get("red").unwrap_or(&0) > &12 { return false; }
    if bag.get("green").unwrap_or(&0) > &13 { return false; }
    if bag.get("blue").unwrap_or(&0) > &14 { return false; }
    true
}

pub fn part1_sum_bags(contents: &str) -> i32 {
    let mut sum = HashMap::<String, i32>::new();
    let game_split = contents.split(":").collect::<Vec<&str>>(); // ["game 1", " 1 red, 2 green, 3 blue"]
    let game_number = game_split[0].split(" ").collect::<Vec<&str>>()[1];
    let game_number = game_number.parse::<i32>().unwrap();

    let grabs = game_split[1].split(";").collect::<Vec<&str>>();
    for grab in grabs {
        let mut put_back = HashMap::<String, i32>::new();
        grab
            .split(",")
            .collect::<Vec<&str>>() // [" 1 red", " 2 green", " 3 blue"]
            .iter()
            .map(|x| x.trim())
            .collect::<Vec<&str>>() // ["1 red", "2 green", "3 blue"]
            .iter()
            .for_each(|x| {
                let tuple = x.split(" ").collect::<Vec<&str>>();
                let color = tuple[1];
                let number = tuple[0].parse::<i32>().unwrap();
                let current = sum.get(color).unwrap_or(&0).clone();
                put_back.insert(color.to_string(), current + number);
            });  // {"red": 1, "green": 2, "blue": 3}
        if !part1_check_requirement(put_back.clone()) { return 0 }
    }
    game_number
}

pub fn part2_sum_bags(contents: &str) -> i32 {
    let mut min_require = HashMap::<String, i32>::new();

    let grabs = contents
        .split(":")
        .collect::<Vec<&str>>()[1]
        .split(";")
        .collect::<Vec<&str>>();

    for grab in grabs {
        grab
            .split(",")
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| x.trim())
            .collect::<Vec<&str>>()
            .iter()
            .for_each(|x| {
                let ball = x.split(" ").collect::<Vec<&str>>();
                let color = ball[1];
                let count = ball[0].parse::<i32>().unwrap();
                let balls_in_bag = min_require.get(color).unwrap_or(&0).clone();
                if count > balls_in_bag { min_require.insert(color.to_string(), count); }
            });
    }

    min_require
        .iter()
        .map(|(_, v)| *v)
        .reduce(|a, b| a * b)
        .unwrap_or(0)
}
