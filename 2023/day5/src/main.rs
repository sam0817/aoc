use std::fs;
use std::collections::HashMap;

fn main() {
    // let contents = fs::read_to_string("input")
    let contents = fs::read_to_string("example")
        .expect("Should have been able to read the file");

    // part 1
    part1(&contents[..]);
    // let result: isize = contents.lines().map(|line| {
    //     part1_sum_bags(line)
    // }).sum();
    //
    // println!("Result: {}", result);

    // part 2
    part2(&contents[..]);
    // let result: isize = contents.lines().map(|line| {
    //
    // }).sum();

    // println!("Result: {}", result);
}

fn part1(content: &str) {
    let mut seeds = Vec::<isize>::new();
    let mut rules = HashMap::<isize, HashMap<isize, isize>>::new();
    let mut rules2 = HashMap::<isize, Vec<(isize, isize, isize)>>::new();
    let mut map_index = -1;
    content.lines().for_each(|line| {
        let mut split = line.split(":").collect::<Vec<&str>>();
        // println!("{}", rules.iter().count());
        match split[0] {
            "seeds" => {
                seeds = split[1].trim().split(" ").collect::<Vec<&str>>()
                    .iter().map(|x| x.parse::<isize>().unwrap()).collect::<Vec<isize>>();
                println!("seeds: {:?}", seeds)
            }
            _ => {
                let first_char =  &split[0].chars().next().unwrap_or(' ');
                match first_char {
                    '0'..='9' => {
                        let nums = split[0].trim().split(" ").collect::<Vec<&str>>()
                            .iter().map(|x| x.parse::<isize>().unwrap()).collect::<Vec<isize>>();
                        println!("{} nums: {:?}",map_index, nums);
                        // parse_rule(nums.clone()).iter().for_each(|(k, v)| {
                        //     // if map_index == 0 { println!("{}: {} -> {}", map_index, k, v) }
                        //     // let mut map = rules.entry(map_index).or_insert(HashMap::<isize, isize>::new());
                        //     // if map_index == 0 { println!("{} -> {:#?}",map_index, map) }
                        //     // map.insert(*k, *v);
                        // });
                        let mut map = rules2.entry(map_index).or_insert(Vec::<(isize, isize, isize)>::new());
                        map.push((nums[1], nums[0], nums[2]));
                    }
                    'a'..='z' => { map_index += 1 }
                    &_ => {} }
            }
        }
    });

    println!("-----------------------");
    let result = seeds.iter().map(|x| {
        let mut seed: isize = x.clone() as isize;
        let len = rules2.len();
        for i in 0..len {
            let k = i as isize;
            // let v = rules.get(&k).unwrap();
            let v = rules2.get(&k).unwrap();
            let rec = v.iter().find(|(source, dest, range)| seed - source >= 0 && seed - source < *range)                ;
            println!("{} -> {:#?}", seed, rec);
            if let Some(rec) = rec {
                seed = run_map(*rec, seed);
                println!("{} -> {}", x, seed);
            }
            // if v.contains_key(&(result as isize)) {
            //     result = *v.get(&(result as isize)).unwrap();
            //     // println!("{} -> {}", x, result)
            // }
        };
        seed
    })
        // .inspect(|x| println!("x: {}", x))
        .collect::<Vec<_>>();
        // .max();

    println!("seeds: {:?}", seeds);
    println!("data: {:?}", result);
    // println!("rules2: {:#?}", rules2);
    println!("Result: {:#?}", result.iter().min());
}

fn run_map(tuple:(isize,isize,isize), k:isize) -> isize {
    println!("tuple: {:?}", tuple);
    let (source, dest, range) = tuple;
    let diff = k - source ;
    if (0..range).contains(&diff) {
        dest + diff
    } else {
        k
    }
}

fn parse_rule(tuple: Vec<isize> ) -> Vec<(isize, isize)> {
    let mut result = Vec::<(isize, isize)>::new();
    for i in 0..tuple[2] {
        result.push((tuple[1] + i, tuple[0] + i));
    }
    result
}
// wrong : 22956580

fn part2(content: &str) {

}

#[derive(Debug, Clone, Copy)]
pub struct Range {
    pub min: isize,
    pub max: isize,
}
pub struct Data {
    pub out_of_range: Vec<Range>,
    pub in_the_range: Vec<Range>,
}

pub fn map_data(data: Data, source_index: isize, dest_index: isize, length: isize) -> Data {
    let mut out_of_range = data.out_of_range.to_vec();
    let mut in_the_range = Vec::<Range>::new();
    // do split
    
    Data { out_of_range, in_the_range }
}

pub fn map_fn(range: Range, source_index: isize, dest_index: isize, length: isize) -> Range {
    if range.min >= source_index && range.max <= source_index + length - 1 {
        let min = dest_index + (range.min - source_index);
        let max = dest_index + (range.max - source_index);
        return Range::new(min, max);
    }
    if range.max < source_index || range.min > source_index + length - 1 {
        return range;
    }
    panic!("Not implemented")
}


pub fn split_range(from: Range, to: Range) -> Data {
    // let mut out_of_range = Vec::<Range>::new();
    // let mut in_the_range = Vec::<Range>::new();
    if from.max < to.min || from.min > to.max {
        return Data {
            out_of_range: vec![from],
            in_the_range: vec![],
        };
    }
    if from.min >= to.min && from.max <= to.max  {
        return Data {
            out_of_range: vec![],
            in_the_range: vec![from],
        };
    }
    if from.min < to.min && from.max <= to.max {
        // result.push(Range::new(from.min, to.min - 1));
        // result.push(Range::new(to.min, from.max));
        // return result;
        return Data {
            out_of_range: vec![Range::new(from.min, to.min - 1)],
            in_the_range: vec![Range::new(to.min, from.max)],
        };
    }
    if from.min >= to.min && from.max > to.max {
        return Data {
            out_of_range: vec![Range::new(to.max + 1, from.max)],
            in_the_range: vec![Range::new(from.min, to.max)],
        };
        // result.push(Range::new(from.min, to.max));
        // result.push(Range::new(to.max + 1, from.max));
        // return result;
    }
    if from.min < to.min && from.max > to.max {
        return Data {
            out_of_range: vec![Range::new(from.min, to.min - 1), Range::new(to.max + 1, from.max)],
            in_the_range: vec![Range::new(to.min, to.max)],
        };
        // result.push(Range::new(from.min, to.min - 1));
        // result.push(Range::new(to.min, to.max));
        // result.push(Range::new(to.max + 1, from.max));
        // return result;
    }
    Data { out_of_range: vec![], in_the_range: vec![] }
}

impl Range {
    pub fn new(min: isize, max: isize) -> Self {
        Self { min, max }
    }
}