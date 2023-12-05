use std::fs;
use std::collections::HashMap;

fn main() {
    let contents = fs::read_to_string("input")
        .expect("Should have been able to read the file");

    // part 1
    part1(&contents[..]);
    // let result: i32 = contents.lines().map(|line| {
    //     part1_sum_bags(line)
    // }).sum();
    //
    // println!("Result: {}", result);

    // part 2
    // let result: i32 = contents.lines().map(|line| {
    //
    // }).sum();

    // println!("Result: {}", result);
}

fn part1(content: &str){

}