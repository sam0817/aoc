use std::fs;

fn main() {
    let contents = fs::read_to_string("input")
        .expect("Should have been able to read the file");

    // part 1
    let result: i32 = contents.lines().map(|line| {
        0
    }).sum();

    println!("Result: {}", result);

    // part 2
    let result: i32 = contents.lines().map(|line| {
        0
    }).sum();

    println!("Result: {}", result);
}


