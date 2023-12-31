use std::fs;

fn main() {
    // let contents = fs::read_to_string("input")
    let contents = fs::read_to_string("example")
        .expect("Should have been able to read the file");

    println!("---------- part1 ----------");
    // part 1
    // part1(&contents[..]);

    println!("---------- part2 ----------");
    // part 2
    // part2(&contents[..]);
    part22(&contents[..]);
}

fn part22(content: &str) {
    let data = parse_raw_data_per_line(content);
    let mut grant_total: usize = 0;
    for data_i in 0..data.len() {
        let (pattern, nums) = duplicate_and_reduce(data[data_i].clone(), 3);
        println!("{data_i}: ------------\n{data:?}");
        println!("{pattern}: ,{nums:?}");
        let pattern = pattern.split(".").collect::<Vec<&str>>();
        let count = cal_possibles(&pattern, &nums);
        println!("count: {}", count);
    }
    // println!("{}", compare_matching_one("???#", 3));
    // println!("{}", compare_matching_many("???#", vec![2]));
}

fn cal_possibles(data: &Vec<&str>, nums: &Vec<usize>) -> usize {
    println!("[P_1]: data: {:?}, nums: {:?}", data, nums);
    let mut sum = 0;
    if nums.len() == 0 {
        if data.iter().any(|x| x.as_bytes().iter().any(|y| *y == ('#' as u8))) {
            println!("[P_2]: 0");
            return 0;
        }
        println!("[P_2]: 1");
        return 1;
    }
    let data_len = data.iter().map(|x| x.len()).sum::<usize>();
    let nums_sum = nums.iter().sum::<usize>();
    if nums_sum > data_len {
        println!("[P_2]: 0");
        return 0;
    }

    if nums.len() == 1 && data.len() == 1 {
        let p1 = compare_matching_one(data[0], nums[0]);
        // let p2 = parse_single_piece(data, nums[0]).unwrap_or(0);
        // println!("[1] data: {}, nums0: {}, p1: {}",data[0], nums[0], p1);
        println!("[P_3]: {:?}, {} --> {p1}", data[0], nums[0]);
        return p1;
    }

    if data.len() == 1 && nums.len() > 1 {
        let res = compare_matching_many(data[0], nums);
        println!("[P_4] data: {:?}, nums: {:?}  res: {}", data, nums, res);
        return res;
    }

    let mut sum = 0;
    if data.len() > 1 {
        // ["###", "???", "???"], => data 0, 1, 3, len:3  for i = 0..3  [0] [0..]
        for data_cut in 0..data.len() {
            let pre = &data[..data_cut].to_vec();
            if pre.iter().any(|x| x.as_bytes().iter().any(|y| *y == ('#' as u8))) {
                continue;
            }
            let data_l = &data[(data_cut)..(data_cut + 1)].to_vec();
            let data_r = &data[(data_cut + 1)..].to_vec();

            for num_cut in 0..=nums.len() {
                let num_l = &nums[..num_cut].to_vec();
                let num_r = if num_cut == nums.len() {
                    vec![]
                } else {
                    nums[num_cut..].to_vec()
                };
                let req = min_len_required(&num_l);
                if data_l[0].len() < req { continue; }
                let p1 = cal_possibles(data_l, &num_l);
                let p2 = cal_possibles(data_r, &num_r);
                // println!("data_l: {:?}, data_r: {:?}  num_l: {:?} num_r: {:?} -->({}, {})", data_l, data_r, num_l, num_r, p1, p2);
                sum += p1 * p2;
            }
            // println!("[P_5] {:?}  {:?} --> {}", data_l, num_l, p1);
            // println!("[P_5] {:?}  {:?} --> {}", data_r, num_r, p2);
        }
    }
    println!("[P_9] {:?}, {:?} -->{}", data, nums, sum);
    sum
}

fn duplicate_and_reduce(data: (String, Vec<i32>), copies: i32) -> (String, Vec<usize>) {
    let (mut big_data, nums) = data;
    big_data.remove(0);
    big_data.remove(big_data.len() - 1);
    let mut piece = big_data[..].to_string();
    let mut nums = nums.iter().map(|x| *x as usize).collect::<Vec<usize>>();
    let mut new_nums = nums.to_vec();
    for _ in 0..copies {
        big_data.push('?');
        big_data.push_str(piece.as_str());
        new_nums.append(&mut nums.to_vec())
    }
    (reduce_dot(&big_data[..]), new_nums)
}

fn parse_n_nums_in_n_data(data_list: &Vec<&str>, nums: &Vec<usize>) -> usize {
    let mut sum = 0;
    for i in 1..data_list.len() {
        for nx in 0..=nums.len() {
            let (num_l, num_r) = if nx == nums.len() {
                (vec![], nums[..nx].to_vec())
            } else {
                (nums[..nx].to_vec(), nums[nx..].to_vec())
            };
            let data_l = data_list[..i].to_vec();
            let count_l = if data_l.len() == 1 {
                parse_n_nums_in_one_data(data_l[0], &num_l)
            } else {
                parse_n_nums_in_n_data(&data_l, &num_l)
            };
            let data_r = data_list[i..].to_vec();
            let count_r = if data_r.len() == 1 {
                parse_n_nums_in_one_data(data_r[0], &num_r)
            } else {
                parse_n_nums_in_n_data(&data_r, &num_r)
            };
            // println!("---------------------------------------------------");
            // println!("data_l: {:?}, data_r: {:?}  num_l: {:?} num_r: {:?}", data_l, data_r, num_l, num_r);
            // println!("count_l: {}, count_r: {}", count_l, count_r);
            sum += count_l * count_r
        }
    }
    sum
}

fn parse_n_nums_in_one_data(data: &str, nums: &Vec<usize>) -> usize {
    let min_req = min_len_required(&nums);
    let data_bytes = data.as_bytes();
    if min_req == 0 {
        if data_bytes.iter().any(|x| *x == ('#' as u8)) {
            return 0;
        }
        return 1;
    }
    if nums.len() == 1 {
        if data.len() == nums[0] {
            if data_bytes.iter().any(|x| *x == ('.' as u8)) {
                return 0;
            }
            return 1;
        }
    }
    if data.len() < min_req { return 0; }
    let mut sum = 0;
    // ???##??, 2,2 len 7
    for nx in 0..=nums.len() {
        let (num_l, num_r) = if nx == nums.len() {
            (vec![], nums[..nx].to_vec())
        } else {
            (nums[..nx].to_vec(), nums[nx..].to_vec())
        };
        let min_req_l = min_len_required(&num_l);
        let min_req_r = min_len_required(&num_r);
        let cut = nums.len() - min_req_l - min_req_r;

        for ni in 0..=cut {
            let mut count_l = 0;
            let mut count_r = 0;
            // let data_l = data[ni..(ni + min_req_l)].to_vec();
            // if num_r.len() > 0 {
            //     if data.as_bytes()[ni + min_req_l] == ('#' as u8) { continue; }
            //     let data_r = data[(ni + min_req_l + 1)..].to_vec();
            //     count_r = parse_n_nums_in_one_data(data_r.as_str(), &num_r);
            // }
            // todo:
        }
    }
    // let possibilities = gen_possibility(nums.clone(), data.len());
    // possibilities.iter().for_each(|x| {
    //     if match_string(&data[..], &x[..]) {
    //         sum += 1;
    //     }
    // });
    sum
}

fn compare_matching(data: &str, nums: &Vec<usize>) -> usize {
    let num = nums[0];
    let r_num = nums[1..].to_vec();
    let min_len = min_len_required(&r_num);
    0
}

fn compare_matching_one(data: &str, num: usize) -> usize {
    let mut count = 0;
    if data.len() < num { return 0; }
    'p: for pattern_i in 0..=(data.len() - num) {
        for scan_i in 0..data.len() {
            let bytes = data.as_bytes();
            if pattern_i <= scan_i && scan_i < pattern_i + num {
                if bytes[scan_i] == ('.' as u8) {
                    continue 'p;
                }
            } else {
                if bytes[scan_i] == ('#' as u8) {
                    continue 'p;
                }
            }
        }
        count += 1;
    }
    count
}

fn compare_matching_many(data: &str, nums: &Vec<usize>) -> usize {
    let num1 = nums[0];
    if nums.len() == 1 { return compare_matching_one(data, num1); }
    if nums.len() == 0 {
        if data.as_bytes().iter().any(|x| *x == ('#' as u8)) {
            return 0;
        }
        return 1;
    }
    if &data[num1..=num1] == "#" {
        // println!("[P1]: {:?}, {} --> 0", data, nums[0]);
        return 0;
    }
    // let num_l = &nums[..1].to_vec();
    let num_r = &nums[1..].to_vec();
    let req_r = min_len_required(num_r);
    // '????? 2
    let mut sum = 0;
    for cut in 0..(data.len() - req_r - num1 + 1) {
        let data_l = &data[cut..(num1 + cut)];

        let data_r = if num1 + cut + 1 >= data.len() {
            ""
        } else {
            &data[(num1 + cut + 1)..]
        };
        let p1 = compare_matching_one(data_l, num1);
        let p2 = compare_matching_many(data_r, num_r);
        println!("data_l: {:?}, data_r: {:?}  num_l: {:?} num_r: {:?}", data_l, data_r, num1, num_r);
        sum += p1 * p2;
    }

    // println!("[P2] L: {:?}, {:?} ---> {}", data_l, num_l, p1);
    // println!("[P2] R: {:?}, {:?} ---> {}", data_r, num_r, p2);
    // println!("data_l: {}, data_r: {}, num_r: {:?}", data_l, data_r, num_r);
    // return p1 * p2;
    sum
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
    if data.len() < num { return Some(1); }
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
    let iter = data.as_bytes();

    // ??????? , 2, 2 (7,2,2) -> (5, 1, 1) ??????
    if iter.iter().all(|x| *x == ('?' as u8)) {
        let mut len = iter.len();
        nums.iter().for_each(|x| len -= (x - 1));
        let n = len - nums.len() + 1;
        let x = nums.len();
        if n < x { return None; }
        return Some(count_combinations(n, x));
    }

    let max = max_sharp_position(data);
    let (start, end) = sharp_position(data);
    let max_len_limit = nums.iter().sum::<usize>() + nums.len() - 1;
    if (max - start + 1) > max_len_limit { return None; }

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
        // if end >= data.len() { continue; }
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
        if end >= data.len() { continue; } // ??????
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

    let (start, end) = sharp_position(data);
    let max = max_sharp_position(data);
    if start > nums[0] {
        // ????#??? len: 8 max 5 : limit 2, start 4
        // if  data.len() - max < nums[nums.len() - 1] + 1 {
        //     for i in 0..=(start - nums[0] - 1) {
        //         let idx = i + nums[0] + 1;
        //         let r_data = &data[idx..];
        //         let r_nums = nums[1..].to_vec();
        //         let r = if r_nums.len() == 1 {
        //             parse_single_piece(r_data, r_nums[0])
        //         } else {
        //             parse_n_in_one_piece(r_data, r_nums)
        //         };
        //         if r.is_some() { combinations += r.unwrap(); }
        //     }
        // } else {
        for i in 0..min {
            let cut = i + nums[0] + 1;
            if iter[cut - 1] == '#' as u8 { continue; }
            let remaining = &data[cut..];
            let r_nums = nums[1..].to_vec();
            let r = if r_nums.len() == 1 {
                parse_single_piece(remaining, r_nums[0])
            } else {
                parse_n_in_one_piece(remaining, r_nums)
            };
            if r.is_some() { combinations += r.unwrap(); }
        }
        // }
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
    if nums.len() == 0 { return 1; }
    if nums.len() == 1 { return parse_single_piece(data, nums[0]).unwrap_or(0); }
    if nums.len() == 2 { return parse_two_in_one_piece(data, nums).unwrap_or(0); }
    if nums.len() > 2 { return parse_n_in_one_piece(data, nums).unwrap_or(0); }
    0
}

fn min_len_required(nums: &Vec<usize>) -> usize {
    if nums.len() == 0 { return 0; }
    let sum = nums.iter().sum::<usize>();
    sum + nums.len() - 1
}

fn loop_data_matching(data: Vec<String>, nums: Vec<usize>) -> usize {
    println!("data: {:?}, nums: {:?}", data, nums);
    if nums.len() == 0 { return 1; }
    if data.len() == 0 { return 0; }
    if data.len() == 1 {
        if data[0].len() < min_len_required(&nums) { return 0; }
        let r = parse_n_in_one_piece(&data[0][..], nums);
        if r.is_some() { return r.unwrap(); }
        return 0;
    }
    // ????
    if data.len() == 2 {
        let mut sum = 0;
        if nums.len() == 1 {
            let r1 = parse_single_piece(&data[0][..], nums[0]);
            let r2 = parse_single_piece(&data[1][..], nums[0]);
            return r1.unwrap_or(0) + r2.unwrap_or(0);
        }
        for i in 0..=nums.len() {
            if nums.len() == 1 { println!("nums: {:?}", nums) }
            let nums_1 = nums[..i].to_vec();
            let nums_2 = nums[i..].to_vec();
            if data[0].len() < min_len_required(&nums_1) { continue; }
            if data[1].len() < min_len_required(&nums_2) { continue; }
            let r1 = parse_chooser(&data[0][..], nums_1.to_vec());
            let r2 = parse_chooser(&data[1][..], nums_2.to_vec());
            sum += (r1 * r2);

            println!("2i: {}, {:?}  num_1: {:?}, num_2:{:?}, r1: {}, r2: {}", i, data, nums_1, nums_2, r1, r2);
        }
        return sum;
    }
    if data.len() > 2 {
        let mut sum = 0;

        for di in 1..data.len() {
            let data_1 = data[..di].to_vec();
            let data_2 = data[di..].to_vec();
            println!("data_i: {}, data_1: {:?}, data_2:{:?}, ", di, data_1, data_2);
            for i in 0..=nums.len() {
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

// ##..## 2, 2 (6) -> 1, 1 (4) C(len - interval, num.len)
// #.#.
// #..#
// .#.#

fn gen_one(num: usize, len: usize) -> Vec<Vec<char>> {
    let mut result = vec![];
    if num > len { return result; }
    let piece = vec!['#'; num];
    for i in 0..=(len - num) {
        // print!("i: {}, ", i);
        let mut prefix = vec!['.'; i - 0];
        // println!("prefix: {:?}", prefix);
        let postfix = vec!['.'; len - num - i];
        // println!("prefix: {:?}", prefix);
        prefix.append(&mut piece.clone());
        // println!("prefix: {:?}", prefix);
        prefix.append(&mut postfix.clone());
        result.push(prefix);
    }
    result
}

fn match_string(mask: &str, trial: &str) -> bool {
    if mask.len() != trial.len() { return false; }

    for (m, t) in mask.chars().zip(trial.chars()) {
        if m == '?' { continue; }
        if m != t { return false; }
    }
    true
}

fn gen_possibility(nums: Vec<usize>, len: usize) -> Vec<String> {
    if nums.len() == 0 {
        let str = vec!['.'; len];
        return vec![str.iter().collect::<String>()];
    }
    if nums.len() == 1 { return gen_one(nums[0], len).iter().map(|x| x.iter().collect::<String>()).collect::<Vec<String>>(); }
    if len == 0 { return vec![]; }
    let min_req = nums.iter().sum::<usize>() + nums.len() - 1;
    if len < min_req { return vec![]; }
    if len == min_req {
        let mut result: Vec<char> = vec![];
        nums.iter().enumerate().for_each(|(idx, n)| {
            if idx > 0 { result.push('.'); }
            let mut piece = vec!['#'; *n];
            result.append(&mut piece)
        });
        return vec![result.iter().collect::<String>()];
    }
    let mut result = vec![];
    for i in 0..=(len - min_req) {
        // println!("i: {}, len: {}, min_req: {}, nums: {:?}", i, len, min_req, nums);
        let r_len = len - nums[0] - i - 1;
        let r_nums = nums[1..].to_vec();
        let mut prefix = vec!['.'; i - 0];
        let mut piece = vec!['#'; nums[0]];
        piece.push('.');
        prefix.append(&mut piece.clone());
        let str = prefix.iter().collect::<String>();
        let others = gen_possibility(r_nums, r_len);
        others.iter().for_each(|x| result.push(str.clone() + x));
    }
    result
}

fn part1(content: &str) {
    let data = parse_raw_data_per_line(content);
    let mut grant_total = 0;
    data.iter().enumerate().for_each(|(i, (data, nums))| {
        let data = reduce_dot(data);
        let nums = nums.iter().map(|x| *x as usize).collect::<Vec<usize>>();
        let options = gen_possibility(nums.clone(), data.len());
        let mut sum = 0;
        options.iter().for_each(|x| {
            if match_string(&data[..], &x[..]) {
                sum += 1;
            }
        });
        println!("{i}, data: {:?}, nums: {:?} ---> {}", data, nums, sum);
        grant_total += sum;
    });
    println!("grant_total: {}", grant_total)
}

fn gen_possibility_with_compare(nums: Vec<usize>, len: usize, mask: &str) -> Vec<String> {
    if nums.len() == 0 {
        let str = vec!['.'; len];
        return vec![str.iter().collect::<String>()];
    }
    if nums.len() == 1 { return gen_one(nums[0], len).iter().map(|x| x.iter().collect::<String>()).collect::<Vec<String>>(); }
    if len == 0 { return vec![]; }
    let min_req = nums.iter().sum::<usize>() + nums.len() - 1;
    println!("len: {}, min_req:{}", len, min_req);
    if len < min_req { return vec![]; }
    if len == min_req {
        let mut result: Vec<char> = vec![];
        nums.iter().enumerate().for_each(|(idx, n)| {
            if idx > 0 { result.push('.'); }
            let mut piece = vec!['#'; *n];
            result.append(&mut piece)
        });
        return vec![result.iter().collect::<String>()];
    }
    let mut result = vec![];
    for i in 0..=(len - min_req) {
        // println!("i: {}, len: {}, min_req: {}, nums: {:?}", i, len, min_req, nums);
        println!("i: {}, len: {}, min_req: {}, str {}, nums: {:?}", i, len, min_req, mask, nums);
        let mut r_len = len - nums[0] - i - 1;
        let r_nums = nums[1..].to_vec();
        let mut prefix = vec!['.'; i - 0];
        let mut piece = vec!['#'; nums[0]];
        piece.push('.');
        prefix.append(&mut piece.clone());
        let now_len = prefix.len();
        let l_mask = &mask[..now_len];
        let mut r_mask = &mask[now_len..];
        let mut l_data = prefix.iter().collect::<String>();
        // l_data.push_str(&piece.iter().collect::<String>());
        // println!("piece: {:?}, r_len: {}, r_nums: {:?}, r_data:{:?}, p_str:{:?}", piece, r_len, r_nums, r_data, p_str);
        println!("l_mask: {}, l_data: {:?}", l_mask, l_data);
        if !match_string(l_mask, &l_data[..]) { continue; }
        prefix.append(&mut piece.clone());
        println!("r_data: {}, r_nums: {:?}", r_mask, r_nums[0]);
        let str = prefix.iter().collect::<String>();
        let others = gen_possibility_with_compare(r_nums, r_len, r_mask);
        others.iter().for_each(|x| result.push(str.clone() + x));
    }
    result
}

fn part2(content: &str) {
    let data = parse_raw_data_per_line(content);
    let mut grant_total: usize = 0;
    data.iter().enumerate().for_each(|(i, (data, nums))| {
        println!("line: {}:", i + 1);
        let mut big_data = data.to_string();
        big_data.remove(0);
        big_data.remove(big_data.len() - 1);
        let trimmed = big_data.clone();
        let mut nums = nums.iter().map(|x| *x as usize).collect::<Vec<usize>>();
        let mut new_nums = nums.to_vec();
        let multiplier = 2;
        for _ in 0..multiplier {
            big_data.push('?');
            big_data.push_str(trimmed.as_str());
            new_nums.append(&mut nums.to_vec())
        }

        println!("big_data: {}", big_data);
        let mut red_data = reduce_dot(big_data.to_string().as_str());
        println!("red_data: {}", red_data);
        // println!("---data: {:?}, nums: {:?}", red_data, nums);
        // println!("data: {:?}, nums: {:?}", data, new_nums);
        let options = gen_possibility_with_compare(new_nums.clone(), red_data.len(), &red_data[..]);
        let mut sum = 0;
        options.iter().for_each(|x| {
            if match_string(&data[..], &x[..]) {
                sum += 1;
            }
        });
        println!("{i}, ---> {}", sum);
        grant_total += sum as usize;
    });
    println!("grant_total: {}", grant_total)
}


#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_match_string() {
    //     assert!(!match_string("????".to_string(), "###".to_string()));
    //     assert!(!match_string("???".to_string(), "####".to_string()));
    //     assert!(!match_string("??.".to_string(), "###".to_string()));
    //     assert!(match_string(".?.".to_string(), ".#.".to_string()));
    //     assert!(match_string("#?#".to_string(), "#.#".to_string()));
    // }

    #[test]
    fn test_compare_matching_one() {
        // #號數量 == num
        assert_eq!(compare_matching_one("###", 3), 1);
        assert_eq!(compare_matching_one("#??#", 3), 0);
        assert_eq!(compare_matching_one("?#?", 3), 1);
        assert_eq!(compare_matching_one("?###?", 3), 1);
        assert_eq!(compare_matching_one("?###", 3), 1);
        assert_eq!(compare_matching_one("???###", 3), 1);
        assert_eq!(compare_matching_one("###????", 3), 1);
        assert_eq!(compare_matching_one("???###???", 3), 1);
        // #號數量 == 0
        assert_eq!(compare_matching_one("???", 3), 1);
        assert_eq!(compare_matching_one("????", 3), 2);
        assert_eq!(compare_matching_one("?????", 3), 3);
        assert_eq!(compare_matching_one("?????", 2), 4);
        assert_eq!(compare_matching_one("??????", 3), 4);

        // 長度 == num
        assert_eq!(compare_matching_one("?#", 2), 1);
        assert_eq!(compare_matching_one("#?", 2), 1);

        // #開頭 或 # 結尾
        assert_eq!(compare_matching_one("#??", 2), 1);
        assert_eq!(compare_matching_one("??#", 2), 1);
        assert_eq!(compare_matching_one("??##", 3), 1);
        assert_eq!(compare_matching_one("??#?#", 4), 1);
        assert_eq!(compare_matching_one("#????????????", 4), 1);
        assert_eq!(compare_matching_one("????????#?#", 4), 1);

        // #號數量 == 1
        assert_eq!(compare_matching_one("??#", 2), 1);
        assert_eq!(compare_matching_one("???#", 2), 1);
        assert_eq!(compare_matching_one("???#", 3), 1);
        assert_eq!(compare_matching_one("????#", 4), 1);
        assert_eq!(compare_matching_one("#??", 2), 1);
        assert_eq!(compare_matching_one("#???", 2), 1);
        assert_eq!(compare_matching_one("?#?", 2), 2);
        assert_eq!(compare_matching_one("?#??", 2), 2);
        assert_eq!(compare_matching_one("?#??", 2), 2);
        assert_eq!(compare_matching_one("??#??", 3), 3);
        assert_eq!(compare_matching_one("?#??", 3), 2);
        assert_eq!(compare_matching_one("?#?", 3), 1);
        assert_eq!(compare_matching_one("??#??", 4), 2);
        assert_eq!(compare_matching_one("???????#????", 4), 4);
        assert_eq!(compare_matching_one("???????#??", 5), 3);
        assert_eq!(compare_matching_one("???????#????????", 5), 5);
    }


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

    // #[test]
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
        assert_eq!(parse_n_in_one_piece("?##?????#??????", vec![3, 1, 3, 1, 1]), Some(20));
        // ?##?????#??????
        // .###..#.###.... -> 1
        // .###.#..###.... -> 1
        // .###.#.###..... -> 3
        // ###...#.###.... -> 1  ---- 15
        // ###..#.###..... -> 3
        // ###..#..###.... -> 1
        // ###.#.###.#.#.. -> 3
        // ###.#.###..#.#. -> 2
        // ###.#.###...#.# -> 1
        // ###.#..###.#.#. -> 3
        // ###.#...###.#.# -> 1
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