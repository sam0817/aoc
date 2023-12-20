use std::fs;

fn main() {
    // let contents = fs::read_to_string("input")
        let contents = fs::read_to_string("example")
        .expect("Should have been able to read the file");

    println!("---------- part1 ----------");
    // part 1
    part1(&contents[..]);

    println!("---------- part2 ----------");
    // part 2
    part2(&contents[..]);
}

fn parse_data(content: &str) {
    content.lines().for_each(|line| {
       println!("{}", line)
    });
}

fn part1(content: &str) {
    parse_data(content);
}

fn part2(content: &str) {

}