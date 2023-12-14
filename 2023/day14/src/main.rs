use std::fs;

fn main() {
    // let contents = fs::read_to_string("input")
    let contents = fs::read_to_string("example")
        .expect("Should have been able to read the file");

    println!("---------- part1 ----------");
    // part 1 111979
    part1(&contents[..]);

    println!("---------- part2 ----------");
    // part 2
    part2(&contents[..]);
}

fn parse_map(content: &str) -> Vec<Vec<char>> {
    let mut map = vec![];
    for line in content.lines() {
        let mut row = vec![];
        for c in line.chars() {
            row.push(c);
        }
        map.push(row);
    }
    map
}

fn part1(content: &str) {
    let mut map = parse_map(content);
    // printmap(&map);
    // println!("----------------");
    rolling_north(&mut map);
    let sum = cal_weight(&map);
    println!("sum: {}", sum);
}

fn rolling_north(map: &mut Vec<Vec<char>>) {
    for r in 1..map.len() {
        for c in 0..map[r].len() {
            if map[r][c] == 'O' {
                let mut new_r = r;
                'b: for rr in (0..r).rev() {
                    if map[rr][c] == '.' {
                        new_r = rr;
                    } else {
                        break 'b;
                    }
                }
                if new_r != r {
                    map[new_r][c] = 'O';
                    map[r][c] = '.';
                }
            }
            // println!("r: {}, c: {}", r, c);
        }
    }
}

fn rolling_south(map: &mut Vec<Vec<char>>) {
    for r in (0..map.len()).rev() {
        for c in 0..map[r].len() {
            if map[r][c] == 'O' {
                let mut new_r = r;
                'b: for rr in ((r + 1)..map.len()) {
                    if map[rr][c] == '.' {
                        new_r = rr;
                    } else {
                        break 'b;
                    }
                }
                if new_r != r {
                    map[new_r][c] = 'O';
                    map[r][c] = '.';
                }
            }
            // println!("r: {}, c: {}", r, c);
        }
    }
}

fn rolling_west(map: &mut Vec<Vec<char>>) {
    for r in 0..map.len() {
        for c in 1..map[r].len() {
            if map[r][c] == 'O' {
                let mut new_c = c;
                'b: for cc in (0..c).rev() {
                    if map[r][cc] == '.' {
                        new_c = cc;
                    } else {
                        break 'b;
                    }
                }
                if new_c != c {
                    map[r][new_c] = 'O';
                    map[r][c] = '.';
                }
            }
            // println!("r: {}, c: {}", r, c);
        }
    }
}

fn rolling_east(map: &mut Vec<Vec<char>>) {
    for r in 0..map.len() {
        for c in (0..map[r].len()).rev() {
            if map[r][c] == 'O' {
                let mut new_c = c;
                'b: for cc in ((c + 1)..map[r].len()) {
                    if map[r][cc] == '.' {
                        new_c = cc;
                    } else {
                        break 'b;
                    }
                }
                if new_c != c {
                    map[r][new_c] = 'O';
                    map[r][c] = '.';
                }
            }
            // println!("r: {}, c: {}", r, c);
        }
    }
}

// cycle ↑←↓→
fn part2(content: &str) {
    let mut map = parse_map(content);
    for i in 0..1 {
        println!("----------------");
        rolling_north(&mut map);
        printmap(&map);
        println!("----------------");
        rolling_west(&mut map);
        printmap(&map);
        println!("----------------");
        rolling_south(&mut map);
        printmap(&map);
        println!("----------------");
        rolling_east(&mut map);
        printmap(&map);
        if i % 100000 == 0 {
            println!("i: {}", i);
        }
        // print!("{}, ", i);
    }
    // printmap(&map);
    // println!("----------------");
    // printmap(&map);
    let sum = cal_weight(&map);
    println!("sum: {}", sum);
}

fn cal_weight(map: &Vec<Vec<char>>) -> usize {
    let mut sum = 0;
    for r in 0..map.len() {
        let times = map.len() - r;
        let count = map[r].iter().filter(|c| **c == 'O').count();
        sum += times * count;
    }

    sum
}

fn printmap(map: &Vec<Vec<char>>) {
    for row in map {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}