use std::collections::HashMap;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input")
    // let contents = fs::read_to_string("example")
        .expect("Should have been able to read the file");

    println!("---------- part1 ----------");
    // part 1
    // part1(&contents[..]);

    println!("---------- part2 ----------");
    // part 2
    part2(&contents[..]);
    // 229885 --> too low
}

fn part1(content: &str) {
    // println!("{}", hash_algorithm("HASH"))
    let data = parse_data(content);
    let mut sum = 0;
    // println!("{:?}", data);
    for i in 0..data.len() {
        let h = hash_algorithm(data[i]);
        // println!("{}", h);
        sum += h;
    }
    println!("{}", sum);
}

fn hash_algorithm(data: &str) -> usize {
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

fn hash_algorithm2(data: &str, map: &mut HashMap<String, u8>) -> u8 {
    if map.contains_key(data) {
        return map.get(data).unwrap().clone();
    }
    let h = hash_algorithm(data) as u8;
    map.insert(data.to_string(), h);
    h
}

fn part2(content: &str) {
    let data = parse_data(content);
    let mut map = HashMap::<String, u8>::new();
    let mut boxes = HashMap::<u8, Vec<(String, u8)>>::new();
    println!("{:?}", data.len());
    for i in 0..data.len() {
        // if i> 10 { break; }
        let split = data[i].split("=").collect::<Vec<_>>();
        if split.len() == 2 {
            let label = split[0];
            let value = split[1].parse::<u8>().unwrap();
            let h = hash_algorithm2(label, &mut map);
            let lens = boxes.entry(h).or_insert(Vec::new());
            let len = lens.iter_mut().find(|x| x.0 == label);
            if len.is_none() {
                lens.push((label.to_string(), value));
            } else {
                let len = len.unwrap();
                len.1 = value;
            }

            // println!("{} = {}", split[0], split[1]);
        }

        let split = data[i].split("-").collect::<Vec<_>>();
        if split.len() == 2 {
            let label = split[0];
            let h = hash_algorithm2(label, &mut map);
            let lens = boxes.entry(h).or_insert(Vec::new());
            let len = lens.iter_mut().find(|x| x.0 == label);
            if len.is_some() {
                let mut cut = 0;
                if lens.len() == 1 {
                    lens.clear();
                    continue;
                }
                for idx in 0..lens.len() {
                    cut = idx;
                    if lens[idx].0 == label {
                        break;
                    }
                }
                *lens = lens[(cut + 1)..].to_vec();
            }
        }
        // println!("{}", h);
    }
    // println!("{:?}", boxes);
    let mut sum: usize = 0;
    boxes.iter().for_each(|(h, v)| {
        // let cnt = v.len();
        v.iter().enumerate().for_each(|(i, (_, f))| {
            // let h = hash_algorithm2(label, &mut map);
            let hh = *h as usize + 1;
            let ii = i as usize +1 ;
            print!("{} {} {} --> ", hh, ii, f);
            let sub = ii * *f as usize * hh;
            println!("{}", sub);
            sum += sub;
        });
    });
    println!("{}", sum);
}