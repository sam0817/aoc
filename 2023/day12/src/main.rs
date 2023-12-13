use std::fs;
use log::log;

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

fn count_combinations(n: usize, r: usize) -> usize {
    if r > n {
        0
    } else {
        (1..=r).fold(1, |acc, val| acc * (n - val + 1) / val)
    }
}

fn count_permutations(n: usize, r: usize) -> usize {
    (n - r + 1..=n).product()
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
    for c in data.chars().rev() {
        count += 1;
        if c == '#' {
            break;
        }
    }
    data.len() - count
}

fn sharp_position(data: &str) -> (usize, usize) {
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
    if data.len() < num { return (0, 0); }
    if data.chars().all(|x| x == '?') { return (0, data.len() - 1); }
    let (start, end) = sharp_position(data);
    let min = end as isize + 1 - num as isize;
    let min = min.max(0) as usize;
    let max = start + num - 1;
    let max = max.min(data.len() - 1);
    (min, max)
}

/// one string for one number only
fn parse_single_piece(data: &str, num: usize) -> Option<usize> {
    if data.len() < num { return None; }
    if data.len() == num { return Some(1_usize); }
    let iter = data.as_bytes();

    if iter.iter().any(|x| *x == ('#' as u8)) {
        let (start, end) = sharp_position(data);
        let max = max_sharp_position(data);
        if 1 + max - start > num { return None; }
    }

    if data.as_bytes().iter().filter(|x| **x == ('#' as u8)).count() == num {
        return Some(1_usize);
    }

    if data.as_bytes().iter().filter(|x| **x == ('#' as u8)).count() == 0 {
        return Some(data.len() + 1 - num);
    }

    if iter[0] == '#' as u8 || iter[data.len() - 1] == '#' as u8 {
        return Some(1_usize);
    }

    let (min, max) = find_first_range(data, num);
    Some(max - min - num + 2)

    // None
}

fn parse_two_in_one_piece(data: &str, nums: Vec<usize>) -> Option<usize> {
    if nums.len() == 0 { return Some(1); }
    if data.len() < nums[0] + nums[1] + 1 { return None; }
    if data.len() == nums[0] + nums[1] + 1 { return Some(1); }
    let max = max_sharp_position(data);
    let (start, end) = sharp_position(data);
    let max_len_limit = nums.iter().sum::<usize>() + nums.len() - 1;
    if (max - start +1 ) > max_len_limit { return  None }

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

    // ??????? , 2, 2 (7,2,2) -> (5, 1, 1) ??????
    if iter.iter().all(|x| *x == ('?' as u8)) {
        let mut len = iter.len();
        nums.iter().for_each(|x| len -= (x - 1));
        let n = len - nums.len() + 1;
        let x = nums.len();
        if n < x { return None; }
        return Some(count_combinations(n, x));
    }

    // 012345678
    // ?#??????? , 3 => (0, 3)
    let mut combinations = 0;
    let (min, max) = find_first_range(data, nums[0]);
    for i in min..=(max - nums[0] + 1) {
        let end = i + nums[0];
        if iter[end] == '#' as u8 { continue; }
        let remaining = &data[(end + 1)..];
        let r = parse_single_piece(remaining, nums[1]);
        if r.is_some() { combinations += r.unwrap(); }
    }

    // 012345678
    // ????#???? 2, 3 => (0, 3)
    let (start, end) = sharp_position(data);
    if start > nums[0] {
        for i in 0..=(start - nums[0] - 1) {
            let idx = i + nums[0] + 1;
            let r = parse_single_piece(&data[idx..], nums[1]);
            if r.is_some() { combinations += r.unwrap(); }
        }
        // let rev = data.chars().rev().collect::<String>();
        // let r_nums = nums.iter().rev().map(|x| *x).collect::<Vec<usize>>();
        // let r = parse_two_in_one_piece(rev.as_str(), r_nums);
        // if r.is_some() { combinations += r.unwrap(); }
    }

    Some(combinations)
}

fn parse_n_in_one_piece(data: &str, nums: Vec<usize>) -> Option<usize> {
    if nums.len() == 0 { return Some(1); }
    let min_len = nums.iter().sum::<usize>() + nums.len() - 1;
    if data.len() < min_len { return None; }
    if data.len() == min_len { return Some(1); }

    let iter = data.as_bytes();

    if iter.iter().all(|x| *x == ('?' as u8)) {
        let mut len = iter.len();
        nums.iter().for_each(|x| len -= (x - 1));
        let n = len - nums.len() + 1;
        let x = nums.len();
        if n < x { return None; }
        return Some(count_combinations(n, x));
    }

    if iter[0] == '#' as u8 {
        let num = nums[0] + 1; // add split dot
        let remaining = &data[num..];
        let r_nums = nums[1..].to_vec();
        if r_nums.len() == 1 { return parse_single_piece(remaining, r_nums[0]); }
        return parse_n_in_one_piece(remaining, r_nums);
    }

    if iter[data.len() - 1] == '#' as u8 {
        let num = nums[nums.len() - 1]; // num - 1 + 1
        let remaining = &data[..data.len() - num - 1];
        let r_nums = nums[..(nums.len() - 1)].to_vec();
        if r_nums.len() == 1 { return parse_single_piece(remaining, r_nums[0]); }
        return parse_n_in_one_piece(remaining, r_nums);
    }


    let mut combinations = 0;
    let (min, max) = find_first_range(data, nums[0]);

    // ???## , 2
    if min > nums[0] {
        let data_rev = data.chars().rev().collect::<String>();
        let nums_rev = nums.iter().rev().map(|x| *x).collect::<Vec<usize>>();
        return parse_n_in_one_piece(data_rev.as_str(), nums_rev);
    }

    for i in min..=(max - nums[0] + 1) {
        let end = i + nums[0];
        if end >= data.len() { continue; }
        if iter[end] == '#' as u8 { continue; }
        let remaining = &data[(end + 1)..];
        let r_nums = nums[1..].to_vec();
        let r = if r_nums.len() == 1 {
            parse_single_piece(remaining, r_nums[0])
        } else {
            parse_n_in_one_piece(remaining, r_nums)
        };
        if r.is_some() { combinations += r.unwrap(); }
    }

    // if data.chars().all(|x| x == '?') { return Some(combinations); }

    // let rev = data.chars().rev().collect::<String>();
    // let rev_nums = nums.iter().rev().map(|x| *x).collect::<Vec<usize>>();
    // let (min, max) = find_first_range(&rev[..], rev_nums[0]);
    // for i in min..=(max - rev_nums[0] + 1) {
    //     let end = i + rev_nums[0];
    //     if end >= rev.len() { continue; }
    //     if iter[end] == '#' as u8 { continue; }
    //     let remaining = &rev[(end + 1)..];
    //     let r_nums = rev_nums[1..].to_vec();
    //     let r = if r_nums.len() == 1 {
    //         parse_single_piece(remaining, r_nums[0])
    //     } else {
    //         parse_n_in_one_piece(remaining, r_nums)
    //     };
    //     if r.is_some() { combinations += r.unwrap(); }
    // }

    let (start, end) = sharp_position(data);
    if start > nums[0] {
        for i in 0..=(start - nums[0] - 1) {
            let idx = i + nums[0] + 1;
            let r_data = &data[idx..];
            let r_nums = nums[1..].to_vec();
            let r = if r_nums.len() == 1 {
                parse_single_piece(r_data, r_nums[0])
            } else {
                parse_n_in_one_piece(r_data, r_nums)
            };
            if r.is_some() { combinations += r.unwrap(); }
        }
    }

    Some(combinations)
}

fn split_data(data: &str) -> Vec<String> {
    // let data = reduce_dot(data);
    let mut split = data.split(".").collect::<Vec<&str>>();
    split.remove(0);
    split.remove(split.len() - 1);
    split.iter().map(|x| x.to_string()).collect::<Vec<String>>()
}

fn parse_chooser(data: &str, nums: Vec<usize>) -> usize {
    if nums.len() == 0 {return  1}
    if nums.len() == 1 { return parse_single_piece(data, nums[0]).unwrap_or(0); }
    if nums.len() == 2 { return parse_two_in_one_piece(data, nums).unwrap_or(0); }
    if nums.len() > 2 { return parse_n_in_one_piece(data, nums).unwrap_or(0); }
    0
}

fn min_len_required(nums: Vec<usize>) -> usize {
    if nums.len() == 0 { return 0 }
    let sum = nums.iter().sum::<usize>();
    sum + nums.len() -1
}

fn loop_data_matching(data: Vec<String>, nums: Vec<usize>) -> usize {
    if nums.len() == 0 { return 1 }
    if data.len() == 0 { return 0 }
    if data.len() == 1 {
        if data[0].len() < min_len_required(nums.clone()) { return 0; }
        let r = parse_n_in_one_piece(&data[0][..], nums);
        if r.is_some() { return r.unwrap(); }
        return 0;
    }
    // ????
    if data.len() == 2 {
        let mut sum  = 0;
        if nums.len() == 1 {
            let r1 = parse_single_piece(&data[0][..], nums[0]);
            let r2 = parse_single_piece(&data[1][..], nums[0]);
            return r1.unwrap_or(0) + r2.unwrap_or(0);
        }
        for i  in 0..=nums.len() {
            if nums.len() == 1 { println!("nums: {:?}", nums)}
            let nums_1 = nums[..i].to_vec();
            let nums_2 = nums[i..].to_vec();
            if data[0].len() < min_len_required(nums_1.clone()) { continue; }
            if data[1].len() < min_len_required(nums_2.clone()) { continue; }
            let r1 = parse_chooser(&data[0][..], nums_1.to_vec());
            let r2 = parse_chooser(&data[1][..], nums_2.to_vec());
            sum += (r1 * r2);

            // println!("2i: {}, {:?}  num_1: {:?}, num_2:{:?}, r1: {}, r2: {}", i, data, nums_1, nums_2, r1, r2);
        }
        return sum;
    }
    if data.len() > 2 {
        let mut sum  = 0;

        for di  in 1..data.len() {
            let data_1 = data[..di].to_vec();
            let data_2 = data[di..].to_vec();
            // println!("data_i: {}, data_1: {:?}, data_2:{:?}, ", di, data_1, data_2);
            for i  in 0..=nums.len() {
                let nums_1 = nums[..i].to_vec();
                let r1 = loop_data_matching(data_1.clone(), nums_1.to_vec());
                let nums_2 = nums[i..].to_vec();
                let r2 = loop_data_matching(data_2.clone(), nums_2.to_vec());
                sum += (r1 * r2);
                // println!("ii: {}, num_1: {:?}, num_2:{:?}, r1: {}, r2: {}", i, nums_1, nums_2, r1, r2);
                // println!("num_i: {}, num_1: {:?}, num_2:{:?} ----> {}", i, nums_1, nums_2, sum);
            }
        }
        return sum / 2;
    }
    0
}

fn part1(content: &str) {
    let data = parse_raw_data_per_line(content);
    data.iter().enumerate().for_each(|(i,(data, nums))| {
        let data = reduce_dot(data);
        let data = split_data(&data[..]);
        let r_nums = nums.iter().map(|x| *x as usize).collect::<Vec<usize>>();
        let r = loop_data_matching(data.clone(), r_nums);
        println!("{i}, data: {:?}, nums: {:?} ---> {}", data, nums, r);
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
        assert_eq!(parse_single_piece("#??#", 3), None);
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
        assert_eq!(parse_two_in_one_piece("???", vec![2, 1]), None);
        assert_eq!(parse_two_in_one_piece("????", vec![2, 1]), Some(1));
        assert_eq!(parse_two_in_one_piece("?#??", vec![2, 1]), Some(1));
        assert_eq!(parse_two_in_one_piece("???#", vec![2, 1]), Some(1));
        assert_eq!(parse_two_in_one_piece("##?#", vec![2, 1]), Some(1));
        assert_eq!(parse_two_in_one_piece("#????????", vec![2, 1]), Some(6));
        assert_eq!(parse_two_in_one_piece("????????#", vec![1, 2]), Some(6));
        assert_eq!(parse_two_in_one_piece("????????#", vec![2, 1]), Some(6));
        assert_eq!(parse_two_in_one_piece("????????#", vec![2, 3]), Some(4));
        assert_eq!(parse_two_in_one_piece("#????????", vec![2, 3]), Some(4));
        assert_eq!(parse_two_in_one_piece("????????#", vec![3, 1]), Some(5));
        assert_eq!(parse_two_in_one_piece("#????????", vec![3, 1]), Some(5));

        assert_eq!(parse_two_in_one_piece("?#???????", vec![3, 1]), Some(9));
        assert_eq!(parse_two_in_one_piece("??#???????", vec![3, 1]), Some(15));

        assert_eq!(parse_two_in_one_piece("??#???????", vec![3, 1]), Some(15));
        assert_eq!(parse_two_in_one_piece("???#???????", vec![3, 1]), Some(15));
        assert_eq!(parse_two_in_one_piece("??#???????", vec![3, 2]), Some(12));

        assert_eq!(parse_two_in_one_piece("????#???????", vec![3, 1]), Some(16));
        assert_eq!(parse_two_in_one_piece("?????#???????", vec![3, 1]), Some(17));

        assert_eq!(parse_two_in_one_piece("?#???#???????", vec![3, 1]), Some(2));
        assert_eq!(parse_two_in_one_piece("????#?#????#?????", vec![1, 1]), None);
    }

    #[test]
    fn test_parse_n_in_one_piece_complex() {
        assert_eq!(parse_n_in_one_piece("?#?????#?", vec![1, 1]), Some(1));
        assert_eq!(parse_n_in_one_piece("?#??#???#?", vec![1, 1, 1]), Some(1));
        assert_eq!(parse_n_in_one_piece("??#??????#?", vec![3, 4, 2]), Some(1));
        assert_eq!(parse_n_in_one_piece("??#??????#?", vec![3, 3, 2]), Some(4));
        assert_eq!(parse_n_in_one_piece("??#??????#?", vec![3, 1, 2]), Some(15));
        // ??#??????#?
        // ###.#....## 4
        // ###.#...##. 3
        // .###.#...## 3
        // .###.#..##. 2
        // ..###.#..## 2
        // ..###.#.##. 1
        assert_eq!(parse_n_in_one_piece("????#?#????#?????", vec![1, 1, 1, 8]), Some(7));
        // ????#?#????#?????
        // ....#.#..######## -> 3
        // ....#.#.########. -> 3
        // #.#.#.########... -> 1
    }

    #[test]
    fn test_parse_n_in_one_piece() {
        assert_eq!(parse_n_in_one_piece("???", vec![2, 1]), None);
        assert_eq!(parse_n_in_one_piece("????", vec![2, 1]), Some(1));
        assert_eq!(parse_n_in_one_piece("?#??", vec![2, 1]), Some(1));
        assert_eq!(parse_n_in_one_piece("???#", vec![2, 1]), Some(1));
        assert_eq!(parse_n_in_one_piece("##?#", vec![2, 1]), Some(1));
        assert_eq!(parse_n_in_one_piece("#????????", vec![2, 1]), Some(6));
        assert_eq!(parse_n_in_one_piece("????????#", vec![1, 2]), Some(6));
        assert_eq!(parse_n_in_one_piece("????????#", vec![2, 1]), Some(6));
        assert_eq!(parse_n_in_one_piece("????????#", vec![2, 3]), Some(4));
        assert_eq!(parse_n_in_one_piece("#????????", vec![2, 3]), Some(4));
        assert_eq!(parse_n_in_one_piece("????????#", vec![3, 1]), Some(5));
        assert_eq!(parse_n_in_one_piece("#????????", vec![3, 1]), Some(5));

        assert_eq!(parse_n_in_one_piece("?#???????", vec![3, 1]), Some(9));
        assert_eq!(parse_n_in_one_piece("??#???????", vec![3, 1]), Some(15));

        assert_eq!(parse_n_in_one_piece("??#???????", vec![3, 1]), Some(15));
        assert_eq!(parse_n_in_one_piece("???#???????", vec![3, 1]), Some(15));
        assert_eq!(parse_n_in_one_piece("??#???????", vec![3, 2]), Some(12));

        assert_eq!(parse_n_in_one_piece("????#???????", vec![3, 1]), Some(16));
        assert_eq!(parse_n_in_one_piece("?????#???????", vec![3, 1]), Some(17));

        assert_eq!(parse_n_in_one_piece("?????", vec![3, 1]), Some(1));
        // assert_eq!(parse_n_in_one_piece("??????", vec![3, 1]), Some(3));
        assert_eq!(parse_n_in_one_piece("????#?", vec![3, 1]), Some(1));
        assert_eq!(parse_n_in_one_piece("?????#?", vec![3, 1]), Some(2));
        assert_eq!(parse_n_in_one_piece("?????#?", vec![3, 2]), Some(3));

        assert_eq!(parse_n_in_one_piece("#???????", vec![3, 1, 2]), Some(1));
        assert_eq!(parse_n_in_one_piece("#????????", vec![3, 1, 2]), Some(3));
        assert_eq!(parse_n_in_one_piece("#?????????", vec![3, 1, 2]), Some(6));
        assert_eq!(parse_n_in_one_piece("?#????????", vec![3, 1, 2]), Some(9));
        assert_eq!(parse_n_in_one_piece("??#????????", vec![3, 1, 2]), Some(19));

        assert_eq!(parse_n_in_one_piece("?????#?", vec![1, 2]), Some(7));
        assert_eq!(parse_n_in_one_piece("????#?", vec![1, 2]), Some(5));
        assert_eq!(parse_n_in_one_piece("???#?", vec![1, 2]), Some(3));

        assert_eq!(parse_n_in_one_piece("??#????????", vec![3, 1, 2]), Some(19));
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
        assert_eq!(find_first_range("??????", 3), (0, 5));        // max - min - num + 2

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