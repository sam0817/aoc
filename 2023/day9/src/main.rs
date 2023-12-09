use std::{fs, hash::Hash, collections::HashMap};

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
    let mut sum = 0;
    content.lines().for_each(|line| {
        let nums =line.split(" ").collect::<Vec<&str>>()
            .iter()
            .map(|x| x.parse::<isize>().unwrap())
            .collect::<Vec<isize>>();
        let n = find_next(&nums);
        sum += n;
    });
    println!("sum: {}", sum);
}

fn find_next(nums: &Vec<isize>) -> isize {
    let mut step = 0_isize;
    let mut map = HashMap::<isize, Vec<isize>>::new();
    map.insert(0, nums.to_vec());
    loop {
        let origin = map.get(&step).unwrap();
        step += 1;
        let mut next = Vec::<isize>::new();
        for i in 0..(origin.len()-1) {
            next.push(origin[i+1] - origin[i]);
        }
        map.insert(step, next.to_vec());
        if next.iter().all(|x| x == &0) { break;}
    }
    let mut sum = 0;
    for i in 0..(step+1) {
        let list =  map.get(&i).unwrap();
        sum += list.last().unwrap();
    }
    sum
}

fn part2(content: &str) {
    let mut sum = 0;
    content.lines().for_each(|line| {
        let nums =line.split(" ").collect::<Vec<&str>>()
            .iter()
            .map(|x| x.parse::<isize>().unwrap())
            .collect::<Vec<isize>>();
        let n = find_next2(&nums);
        sum += n;
    });
    println!("sum: {}", sum);
}

fn find_next2(nums: &Vec<isize>) -> isize {
    let mut step = 0_isize;
    let mut map = HashMap::<isize, Vec<isize>>::new();
    map.insert(0, nums.to_vec());
    loop {
        let origin = map.get(&step).unwrap();
        step += 1;
        let mut next = Vec::<isize>::new();
        for i in 0..(origin.len()-1) {
            next.push(origin[i+1] - origin[i]);
        }
        map.insert(step, next.to_vec());
        if next.iter().all(|x| x == &0) { break;}
    }
    let mut sum = 0; // nums.first().unwrap().clone();
    for i in 0..(step+1) {
        let list =  map.get(&i).unwrap();
        sum += list.first().unwrap() * (-1_isize).pow(i as u32);
    }
    sum
}