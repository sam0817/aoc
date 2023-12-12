use std::fs;
use log::log;

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

fn parse_raw_data_per_line(content: &str) -> Vec<(String, Vec<i32>)> {
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

fn max_sharp_position(data: &str) -> usize {
    let mut count = 0;
    for c in  data.chars().rev() {
        count+= 1;
        if c == '#' {
            break;
        }
    }
    data.len() - count
}

fn sharp_position(data: &str) -> (usize,usize) {
    let mut start = -1;
    let mut end = 0;
    for (i, c) in data.chars().enumerate() {
        if end > 0 && c != '#' { break; }
        if c == '#' {
            if start == -1 { start = i as i32; }
            if start != -1 { end = i as i32; }
            continue;
        }
    }
    (start as usize, end as usize)
}

fn find_first_range(data: &str, num: usize) -> (usize, usize) {
    let (start, end) = sharp_position(data);
    let min =  end as isize + 1  - num as isize;
    let min = min.max(0) as usize;
    let max = start + num - 1;
    let max = max.min(data.len() - 1);
    (min, max)
}

/// one string for one number only
fn parse_single_piece(data: &str, num: usize) -> Option<usize> {
    if data.len() < num { return None; }
    if data.len() == num { return Some(1_usize); }

    if data.as_bytes().iter().filter(|x| **x == ('#' as u8)).count() == num {
        return Some(1_usize);
    }

    if data.as_bytes().iter().filter(|x| **x == ('#' as u8)).count() == 0 {
        return Some(data.len()  + 1 - num);
    }

    let iter = data.as_bytes();

    if iter[0] == '#' as u8 || iter[data.len() - 1] == '#' as u8 {
        return Some(1_usize);
    }

    let (min, max) = find_first_range(data, num);
    Some(max - min - num + 2)

    // None
}

fn parse_two_in_one_piece(data: &str, nums: Vec<usize>) -> Option<usize> {
    if data.len() < nums[0] + nums[1] +1 { return None; }
    if data.len() == nums[0] + nums[1] +1 { return Some(1); }

    let iter = data.as_bytes();
    if iter[0] == '#' as u8 {
        let num = nums[0] + 1; // add split dot
        let remaining = &data[num..];
        return parse_single_piece(remaining, nums[1]);
    }

    if iter[data.len() - 1] == '#' as u8 {
        let num = nums[1]; // num - 1 + 1
        let remaining = &data[..data.len() - num - 1];
        return parse_single_piece(remaining, nums[0]);
    }
    // 012345678
    // ?#??????? , 3 => (0, 3)
    let mut combinations = 0;
    let (min, max) = find_first_range(data, nums[0]);
    for i in min..=(max - nums[0] + 1) {
        let end = i + nums[0];
        if iter[end] == '#' as u8 { continue; }
        let remaining = &data[(end+1)..];
        let r = parse_single_piece(remaining, nums[1]);
        if r.is_some() { combinations += r.unwrap(); }
    }

    let rev = data.chars().rev().collect::<String>();
    let (min, max) = find_first_range(&rev[..], nums[1]);
    for i in min..=(max - nums[1] + 1) {
        let end = i + nums[1];
        if iter[end] == '#' as u8 { continue; }
        let remaining = &rev[(end+1)..];
        let r = parse_single_piece(remaining, nums[0]);
        if r.is_some() { combinations += r.unwrap(); }
    }


    Some(combinations)
}

fn part1(content: &str) {
    let data = parse_raw_data_per_line(content);
    data.iter().for_each(|(data, nums)| {
        let data = reduce_dot(data);
        println!("data: {}, nums: {:?}", data, nums);
    });
}

fn part2(content: &str) {
    let data = parse_raw_data_per_line(content);
    data.iter().for_each(|(data, nums)| {
        let data = reduce_dot(data);
        // println!("data: {}, nums: {:?}", data, nums);
    });
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_piece() {
        // #號數量 == num
        assert_eq!(parse_single_piece("###", 3), Some(1));
        assert_eq!(parse_single_piece("?#?", 3), Some(1));
        assert_eq!(parse_single_piece("?###?", 3), Some(1));
        assert_eq!(parse_single_piece("?###", 3), Some(1));
        assert_eq!(parse_single_piece("???###", 3), Some(1));
        assert_eq!(parse_single_piece("###????", 3), Some(1));
        assert_eq!(parse_single_piece("???###???", 3), Some(1));
        // #號數量 == 0
        assert_eq!(parse_single_piece("???", 3), Some(1));
        assert_eq!(parse_single_piece("????", 3), Some(2));
        assert_eq!(parse_single_piece("?????", 3), Some(3));
        assert_eq!(parse_single_piece("?????", 2), Some(4));
        assert_eq!(parse_single_piece("??????", 3), Some(4));

        // 長度 == num
        assert_eq!(parse_single_piece("?#", 2), Some(1)); // 1 1 2 2
        assert_eq!(parse_single_piece("#?", 2), Some(1)); // 0 0

        // #開頭 或 # 結尾
        assert_eq!(parse_single_piece("#??", 2), Some(1));
        assert_eq!(parse_single_piece("??#", 2), Some(1));
        assert_eq!(parse_single_piece("??##", 3), Some(1));
        assert_eq!(parse_single_piece("??#?#", 4), Some(1));
        assert_eq!(parse_single_piece("#????????????", 4), Some(1));
        assert_eq!(parse_single_piece("????????#?#", 4), Some(1));

        // #號數量 == 1
        assert_eq!(parse_single_piece("??#", 2), Some(1));
        assert_eq!(parse_single_piece("???#", 2), Some(1));
        assert_eq!(parse_single_piece("???#", 3), Some(1));
        assert_eq!(parse_single_piece("????#", 4), Some(1));
        assert_eq!(parse_single_piece("#??", 2), Some(1));
        assert_eq!(parse_single_piece("#???", 2), Some(1));
        assert_eq!(parse_single_piece("?#?", 2), Some(2));
        assert_eq!(parse_single_piece("?#??", 2), Some(2));
        assert_eq!(parse_single_piece("?#??", 2), Some(2));
        assert_eq!(parse_single_piece("??#??", 3), Some(3));
        assert_eq!(parse_single_piece("?#??", 3), Some(2));
        assert_eq!(parse_single_piece("?#?", 3), Some(1));
        assert_eq!(parse_single_piece("??#??", 4), Some(2));
        assert_eq!(parse_single_piece("???????#????", 4), Some(4));
        assert_eq!(parse_single_piece("???????#??", 5), Some(3));
        assert_eq!(parse_single_piece("???????#????????", 5), Some(5));

        // assert_eq!(parse_single_piece("###", 2), None);
    }

    #[test]
    fn test_parse_two_in_one_piece() {
        assert_eq!(parse_two_in_one_piece("???", vec![2,1]), None);
        assert_eq!(parse_two_in_one_piece("????", vec![2,1]), Some(1));
        assert_eq!(parse_two_in_one_piece("?#??", vec![2,1]), Some(1));
        assert_eq!(parse_two_in_one_piece("???#", vec![2,1]), Some(1));
        assert_eq!(parse_two_in_one_piece("##?#", vec![2,1]), Some(1));
        assert_eq!(parse_two_in_one_piece("#????????", vec![2,1]), Some(6));
        assert_eq!(parse_two_in_one_piece("????????#", vec![1,2]), Some(6));
        assert_eq!(parse_two_in_one_piece("????????#", vec![2,1]), Some(6));
        assert_eq!(parse_two_in_one_piece("????????#", vec![2,3]), Some(4));
        assert_eq!(parse_two_in_one_piece("#????????", vec![2,3]), Some(4));
        assert_eq!(parse_two_in_one_piece("????????#", vec![3,1]), Some(5));
        assert_eq!(parse_two_in_one_piece("#????????", vec![3,1]), Some(5));

        assert_eq!(parse_two_in_one_piece("?#???????", vec![3,1]), Some(9));
        assert_eq!(parse_two_in_one_piece("??#???????", vec![3,1]), Some(15));

        assert_eq!(parse_two_in_one_piece("??#???????", vec![3,1]), Some(15));
        assert_eq!(parse_two_in_one_piece("???#???????", vec![3,1]), Some(15));
        assert_eq!(parse_two_in_one_piece("??#???????", vec![3,2]), Some(12));

        assert_eq!(parse_two_in_one_piece("????#???????", vec![3,1]), Some(16));
        assert_eq!(parse_two_in_one_piece("?????#???????", vec![3,1]), Some(17));
    }

    #[test]
    fn test_sharp_position() {
        assert_eq!(sharp_position("###"), (0, 2));
        assert_eq!(sharp_position("?###???"), (1, 3));
        assert_eq!(sharp_position("???###"), (3, 5));
        assert_eq!(sharp_position("?#?#?#"), (1, 1));
        assert_eq!(sharp_position("?#?#?#"), (1, 1));
        assert_eq!(sharp_position("?????#"), (5, 5));
    }

    #[test]
    fn test_range() {
        assert_eq!(find_first_range("###", 3), (0, 2));        // max - min - num + 2
        assert_eq!(find_first_range("????##????", 3), (3, 6)); // 6 - 3 - 3 + 2 = 2
        assert_eq!(find_first_range("?????#???????", 3), (3, 7)); // 7 - 3 - 3 + 2 = 3
        assert_eq!(find_first_range("???#???????", 3), (1, 5)); // 5 - 3 - 1 + 2 = 3
        assert_eq!(find_first_range("?#???????", 3), (0, 3));  // 5 - 5 - 0 + 2 = 2
        assert_eq!(find_first_range("?#???????", 5), (0, 5));  // 5 - 5 - 0 + 2 = 2
        assert_eq!(find_first_range("??????#??", 5), (2, 8));  // 8 - 5 - 2 + 2 = 3
        assert_eq!(find_first_range("???#???????", 3), (1, 5));  // 8 - 5 - 2 + 2 = 3
        // assert_eq!(find_range("###", 3), (0, 2));

        assert_eq!(find_first_range("?#?#??#??", 5), (0, 5));  // 8 - 5 - 2 + 2 = 3
        assert_eq!(find_first_range("?#?#??#??", 5), (0, 5));  // 8 - 5 - 2 + 2 = 3
        assert_eq!(find_first_range("?#?#??#??", 5), (0, 5));  // 8 - 5 - 2 + 2 = 3
    }

    #[test]
    fn test_max_sharp_position() {
        assert_eq!(max_sharp_position("###"), 2);
        assert_eq!(max_sharp_position("????##????"), 5);
        assert_eq!(max_sharp_position("?????##??????"), 6);
        assert_eq!(max_sharp_position("???#???????"), 3);
        assert_eq!(max_sharp_position("?#???????"), 1);
        assert_eq!(max_sharp_position("?#??????#"), 8);
        assert_eq!(max_sharp_position("??????#??"), 6);
    }
}