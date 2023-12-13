use std::collections::HashMap;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input")
    // let contents = fs::read_to_string("example")
        .expect("Should have been able to read the file");
    println!("---------- part1 ----------");
    // part 1
    part1(&contents[..]);
    // 17255 --> too low
    // 17614 -> not the right answer
    // 32934 -> too high
    // 18558 -> too low

    println!("---------- part2 ----------");
    // part 2
    part2(&contents[..]);
}

fn parse_data(content: &str) -> HashMap::<usize, Vec<(usize, usize, usize)>> {
    let mut hash_map = HashMap::<usize, Vec<(usize, usize, usize)>>::new();
    let mut matrix_count = 0;
    let mut line_count = 0;
    for line in content.lines() {
        if line.len() == 0 {
            matrix_count += 1;
            line_count = 0;
            continue;
        }
        line_count += 1;
        let mut position = 0;
        for char in line.chars() {
            position += 1;
            let mut vec = hash_map.entry(matrix_count).or_insert(Vec::new());
            if char == '#' {
                vec.push((line_count, position, 1));
            } else {
                // vec.push((line_count, position, 0));
            }
        }
    }
    hash_map
}

fn parse_one_matrix(data: Vec<(usize, usize, usize)>) {
    let mut hash = HashMap::<(usize, usize), usize>::new();
    data.iter().for_each(|(x, y, z)| {
        hash.insert((*x, *y), *z);
    });
    println!("hash: {:?}", hash);
}

fn parse_vertical(data: HashMap<(usize, usize), usize>) {
    let width = data.keys().max_by(|(_, y1), (_, y2)| y1.cmp(y2)).unwrap().1;
    let height = data.keys().max_by(|(x1, _), (x2, _)| x1.cmp(x2)).unwrap().0;
    for x in 1..=height {
        let mut count = 0;
        for y in 1..=width {
            let value = data.get(&(x, y)).unwrap_or(&0);
            if *value == 1 {
                count += 1;
            }
        }
        println!("count: {}", count);
    }
}

fn transport(matrix: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let width = matrix[0].len();
    let height = matrix.len();
    let mut result = vec![vec![false; height]; width];
    for x in 0..width {
        for y in 0..height {
            result[x][y] = matrix[y][x];
        }
    }
    result
}

fn parse_one_matrix_2(data: Vec<(usize, usize, usize)>) -> Vec<Vec<bool>> {
    let max_line = data.iter().max_by(|(x1, _, _), (x2, _, _)| x1.cmp(x2)).unwrap().0;
    let max_column = data.iter().max_by(|(_, y1, _), (_, y2, _)| y1.cmp(y2)).unwrap().1;
    // println!("max_line: {}, max_column: {}", max_line, max_column);
    let mut matrix: Vec<Vec<bool>> = vec![vec![false; max_column]; max_line];
    data.iter().filter(|(_, _, z)| *z == 1).for_each(|(x, y, z)| {
        matrix[*x - 1][*y - 1] = true;
    });
    // println!("matrix: {:?}", matrix);
    // parse_one_line(matrix[0].to_vec());
    matrix
}

fn parse_one_line(data: Vec<bool>) -> Vec<isize> {
    let mut result = vec![0; data.len()];
    let len = data.len();
    for i in 1..len {
        let (l, r) = (i as isize - 1, i as isize);
        if l < 0 || r >= len as isize { break; }
        for cnt in 0..len {
            let (ll, rr) = (l - cnt as isize, r + cnt as isize);
            if ll < 0 || rr >= len as isize { break; }
            if data[ll as usize] == data[rr as usize] {
                result[i] += 1;
            } else {
                break;
            }
        }
    }
    // println!("{:?}", data);
    // println!("{:?}", result);
    // one_line_left(result);
    result
}

fn calculate_line_left(data: Vec<isize>) -> isize {
    let mut result = data.to_vec();
    let max = result.iter().max().unwrap().clone();
    for i in 1..data.len() {
        // result[i] = (result[i] as isize - i as isize).max(0) as usize;
        // if result[i] != 0 { result[i] = i as isize - result[i] }
        if result[i] == max { return  i as isize}
    }
    // println!("{:?}", result);
    0
}

fn cal_row_min(data: &Vec<Vec<isize>>) -> Vec<isize> {
    let mut result = vec![0];
    let width = data[0].len();
    let height = data.len();
    for col in 1..width {
        let mut min = isize::MAX;
        for row in 0..height {
            if data[row][col] < min {
                min = data[row][col];
            }
        }
        result.push(min);
        // println!("min: {}", min);
    }
    println!("result: {:?}", result);
    result
}

fn part1(content: &str) {
    let data = parse_data(content);
    let mut keys = data.keys().collect::<Vec<_>>();
    keys.sort();
    let matrix = keys.iter().map(|k| {
        let m = data.get(k).unwrap().to_vec();
        parse_one_matrix_2(m.to_vec())
    }).collect::<Vec<_>>();
    // let tr_matrix = matrix.iter().map(|m| transport(m.to_vec())).collect::<Vec<_>>();
    // parse_one_matrix_2(data.get(&0).unwrap().to_vec());
    // println!("matrix: {:?}", matrix);
    let stages = matrix.iter().map(|m| {
        let rows = m.iter().map(|line| {
            let r = parse_one_line(line.to_vec());
            // println!("r1: {:?}", r);
            // let r = calculate_line_left(r);

            // println!("r2: {:?}", r);
            r
        }).collect::<Vec<_>>();
        let col = transport(m.to_vec()).iter().map(|line| {
            let r = parse_one_line(line.to_vec());
            // let r = calculate_line_left(r);
            r
        }).collect::<Vec<_>>();
        let rs = cal_row_min(&rows);
        let cs = cal_row_min(&col);
        let r = rs.iter().position(|x| *x != 0).unwrap_or(0);
        let c = cs.iter().position(|x| *x != 0).unwrap_or(0);
        // let r = row.iter().min().unwrap().clone();
        // let c = col.iter().min().unwrap().clone();
        (r, c)
    }).collect::<Vec<_>>();
    println!("stages: {:?}", stages);
    let mut sum = 0;
    stages.iter().enumerate().for_each(|(i, (r, c))| {
       if i % 2 == 0 {
           sum += r ;
       } else {
           sum += c * 100;
       }
        // println!("stage: {}, r: {}, c: {}", i, r, c);
        // sum += r + c;
    });
    println!("sum: {}", sum);
}

fn part2(content: &str) {}