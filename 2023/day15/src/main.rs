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
    // println!("{}", hash_algorithm("HASH"))
    let data = parse_data(content);
    let mut sum = 0;
    // println!("{:?}", data);
    for i in 0..data.len(){
        let h = hash_algorithm(data[i]);
        // println!("{}", h);
        sum+=h;
    }
    println!("{}", sum);
}

fn hash_algorithm(data :&str) -> usize {
    let bytes = data.as_bytes();
    let mut current: usize = 0;
    for i in 0..data.len() {
        current = (current + bytes[i] as usize) * 17 % 256;

    }
    current
}

fn parse_data(content: &str) -> Vec<&str> {
    let mut data: Vec<i32> = Vec::new();
    content.split(",").map(|x| x).collect::<Vec<_>>()
    // data
}

fn part2(content: &str) {

}