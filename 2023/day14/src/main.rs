use std::fs;

fn main() {
    let contents = fs::read_to_string("input")
        // let contents = fs::read_to_string("example")
        .expect("Should have been able to read the file");

    println!("---------- part1 ----------");
    // part 1 111979
    part1(&contents[..]);

    println!("---------- part2 ----------");
    // part 2
    part2(&contents[..]);
}

fn parse_map (content: &str) -> Vec<Vec<char>> {
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
    let mut map  = parse_map(content);
    printmap(&map);
    println!("----------------");
    for r in 1..map.len() {
        for c in 0..map[r].len() {
            if map[r][c] == 'O' {
                let mut new_r = r;
                'b: for rr in (0..r).rev() {
                    // println!("rr: {}", rr);
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
    printmap(&map);
    // println!("{:?}", map);
    let mut sum = 0;
    for r in 0..map.len() {
        let times = map.len() - r;
        let count = map[r].iter().filter(|c| **c == 'O').count();
        sum += times * count;
    }
    println!("sum: {}", sum);
}

fn part2(content: &str) {
}

fn printmap(map: &Vec<Vec<char>>) {
    for row in map {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}