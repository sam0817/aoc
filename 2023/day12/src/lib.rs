use std::collections::HashMap;

fn parse_data(content: &str) -> Vec<(String, Vec<i32>)> {
    content.lines().map(|line| {
        let mut line = line.split(" ");
        let data = line.next().unwrap();
        let data = ".".to_string() + data + ".";
        let nums = line.next().unwrap();
        let nums = nums.split(",").collect::<Vec<&str>>().iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
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

fn two_part_handing(data1: String, data2: String, nums: Vec<i32>) -> i32 {
    let mut result = 0;
    // let mut split_data = data.split(".").collect::<Vec<&str>>();
    // split_data.remove(0);
    // split_data.remove(split_data.len() - 1);
    // split_data.iter().enumerate().for_each(|(i, x)| {
    //     let r = part_data_handling(x.to_string(), num);
    //     result += r;
    // });
    // result
    result
}

fn part1(content: &str) {
    let data = parse_data(content);
    let mut resolved = 0;
    let mut perfect_split_count = 0;
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
                // println!("data: {}, nums: {:?} -> r: {}", x, nums, r);
                answer *= r as isize;
            });
            resolved += 1;
            perfect_split_count += 1;
            println!("data: {}, nums: {:?} -> answer: {}", data, nums, answer);
            return;
        }

        // // 2 part
        // if data.as_bytes().iter().filter(|x| **x == ('.' as u8)).count() == 3 {
        //     // println!("data: {}, nums: {:?}", data, nums);
        //     println!("data split 2 part");
        //     let mut split_data = data.split(".").collect::<Vec<&str>>();
        //     split_data.remove(0);
        //     split_data.remove(split_data.len() - 1);
        //     let result = two_part_handing(split_data[0].to_string(), split_data[1].to_string(), nums.to_vec());
        //     println!("split_data: {:?} vs nums {:?} -> result: {}", split_data, nums, result);
        // }
    });
    println!("perfect split: {}", perfect_split_count);
    println!("resolved: {}", resolved);
}




fn part2(content: &str) {
    let mut data = parse_data(content);
    let mut parts = HashMap::<(usize, usize),isize>::new();
    data.iter_mut().for_each(|(data, nums)| {
        let data_string = reduce_dot(data);
        let data = data_string[1..data_string.len() - 1].to_string();
        let len = data.split(".").collect::<Vec<&str>>().len();
        let entry = (len, nums.len());
        *parts.entry(entry).or_insert(0) += 1;
        // parts.get_mut(&(len as isize)).map(|x| *x += 1); //.or_insert(1);
    });
    println!("stat: {:?}", parts);
}