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

pub fn resolve(a: f64, b : f64, c: f64) -> (f64, f64) {
    let item = b.powi(2) - 4f64 * a * c;
    let x1 = (-b + item.sqrt()) / (2.0 * a);
    let x2 = (-b - item.sqrt()) / (2.0 * a);
    (x1, x2)
}
fn part1(content: &str) {}

fn part2(content: &str) {
    let a = 1_f64;
    let b = -51926890_f64;
    let c = 222203111261225_f64;
    let (x1, x2) = resolve(a, b, c);
    println!("x1: {}, x2: {}", x1, x2);
    let num = x1 as isize - x2 as isize;
    println!("num: {}", num)
}