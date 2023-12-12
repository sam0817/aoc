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

fn parse_data(content: &str) -> Vec<(String, Vec<i32>)> {
    content.lines().map(|line| {
        let mut line = line.split(" ");
        let data = line.next().unwrap();
        let data = ".".to_string() + data + ".";
        let nums = line.next().unwrap();
        let nums = nums.split(",").collect::<Vec<&str>>().iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        // println!("data: {}, nums: {:?}", data, nums);
        (data.to_string(), nums)
    }).collect::<Vec<_>>()

}
fn handle_cut_data(data: &str, nums: &Vec<i32>)  {
    let mut mask_count = 0;
    data.chars().enumerate().for_each(|(i, c)| {
        if c == '#' {
            println!("i: {}, c: {}", i, c);
        }
    });
}

fn cut_left(data: &str, nums: &Vec<i32>) -> (String, Vec<i32>) {
    let mut index = 0;
    let mut mask_count = 0;
    let mut interval = 0;
    let mut mech_count = 0;
    for (i, c) in data.chars().enumerate() {
        index = i;
        if i == 0 { continue; }
        if c == '.' && interval > 0 { break; }
        if c == '?' {
            mask_count += 1;
            interval += 1;
        }
        if c == '#' {
            interval += 1;
            mech_count += 1;
        }
    }
    println!("mask_count: {}, interval: {}, mech: {}, idx: {}", mask_count, interval, mech_count, index);
    (data.to_string(), nums.to_vec())
}

fn reduce_dot(data: &str) -> String {
    let mut result = String::new();
    for c in data.chars() {
        if result.as_bytes().last() == Some(&('.' as u8)) && c == '.' {
            continue;
        }
        result.push(c);
    }
    result
}

// ###
// ?#?
// ?###? -> 1, 3, 1 => 4 -> (1+1),  5-> 1
// ????? 1 ->5 , 2-> 4, 3-> 3, 4-> 2, 5-> 1
fn part_data_handling(data: String, num: i32) -> i32 {
    if data.len() == num as usize { return 1; }

    if data.as_bytes().iter().filter(|x| **x == ('#' as u8)).count() == num as usize {
        return 1;
    }

    if data.as_bytes().iter().filter(|x| **x == ('#' as u8)).count() == 0 {
        return data.len() as i32 + 1 - num;
    }

    0
}

fn part1(content: &str) {
    let data = parse_data(content);
    let mut resolved = 0;
    data.iter().for_each(|(data, nums)| {
        // println!("data: {}, nums: {:?}", data, nums);
        let data = reduce_dot(data);

        // perfect split
        if data.as_bytes().iter().filter(|x| **x == ('.' as u8)).count() == nums.len() + 1 {
            // println!("data perfect split");
            let mut split_data = data.split(".").collect::<Vec<&str>>();
            split_data.remove(0);
            split_data.remove(split_data.len() - 1);
            let mut answer: isize = 1;
            split_data.iter().enumerate().for_each(|(i, x)| {
                let r = part_data_handling(x.to_string(), nums[i]);
                // println!("data: {}, -> r: {}", x, r);
                answer *= r as isize;
            });
            resolved += 1;
            // println!("answer: {}", answer);
            return;
        }

        if data.as_bytes().iter().filter(|x| **x == ('.' as u8)).count() == nums.len() {
            // println!("data: {}, nums: {:?}", data, nums);
            println!("data split 2 part");
            let mut split_data = data.split(".").collect::<Vec<&str>>();
            split_data.remove(0);
            split_data.remove(split_data.len() - 1);
            println!("split_data: {:?} vs nums {:?}", split_data, nums);
        }
    });
    println!("resolved: {}", resolved);
}




fn part2(content: &str) {
    let data = parse_data(content);
    let mut max_diff = 0;
    data.iter().for_each(|(data, nums)| {
       let sum = nums.iter().sum::<i32>();
         let diff = data.len() as i32 - sum - nums.len() as i32 - 1;
            if diff > max_diff {
                max_diff = diff;
            }
    });
    println!("max_diff: {}", max_diff);
}