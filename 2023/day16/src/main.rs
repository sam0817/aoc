use std::collections::HashMap;
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

fn part1(content: &str) {
    let map = parse_map(content);
    let mut history = HashMap::<(Point, Direction), ()>::new();
    let mut light = vec![vec![0; map[0].len()]; map.len()];
    let mut points = Point { r: 0, c: 0 };
    light[points.r as usize][points.c as usize] += 1;
    let mut points = next(&points, Direction::Right, &map, &mut history, &mut light);
    loop {
        points = points.iter().flat_map(|(p, d)| {
            next(p, *d, &map, &mut history, &mut light)
        }).collect::<Vec<_>>();
        if points.len() == 0 {
            break;
        }
    }
    println!("map: {:?}", map);
    println!("points: {:?}", points);
    println!("light: {:?}", light);
    let sum = light.iter().map(|row| row.iter()
        .filter(|&&x| x > 0)
        .count()).sum::<usize>();
    println!("sum: {}", sum);
}

fn part2(content: &str) {}

fn next(p: &Point, dir: Direction, map: &Vec<Vec<char>>,
        history: &mut HashMap<(Point, Direction), ()>, light: &mut Vec<Vec<usize>>) -> Vec<(Point,Direction)>
{
    // let mut result = vec![];
    let cell = map[p.r as usize][p.c as usize];

    let mut points =match (dir, cell) {
        (Direction::Right,'/') => vec![(p.up(),Direction::Up)],
        (Direction::Right,'|') => vec![(p.up(), Direction::Up), (p.down(), Direction::Down)],
        (Direction::Right,'\\') => vec![(p.down(), Direction::Down)],
        (Direction::Right,_) => vec![(p.right(), Direction::Right)],
        (Direction::Left,'/') => vec![(p.down(),Direction::Down)],
        (Direction::Left,'|') => vec![(p.up(), Direction::Up), (p.down(), Direction::Down)],
        (Direction::Left,'\\') => vec![(p.up(), Direction::Up)],
        (Direction::Left,_) => vec![(p.left(), Direction::Left)],
        (Direction::Up,'/') => vec![(p.right(),Direction::Right)],
        (Direction::Up,'-') => vec![(p.left(), Direction::Left), (p.right(), Direction::Right)],
        (Direction::Up,'\\') => vec![(p.left(), Direction::Left)],
        (Direction::Up,_) => vec![(p.up(), Direction::Up)],
        (Direction::Down,'/') => vec![(p.left(),Direction::Left)],
        (Direction::Down,'-') => vec![(p.left(), Direction::Left), (p.right(), Direction::Right)],
        (Direction::Down,'\\') => vec![(p.right(), Direction::Right)],
        (Direction::Down,_) => vec![(p.down(), Direction::Down)],
         // (_,_) => vec![],
    };
    // let points =points.iter().filter()
    points.retain(|(p, _)| {
        p.r >= 0 && p.r < map.len() as isize && p.c >= 0 && p.c < map[0].len() as isize
    });
    points.retain(|(p, d)| {
        !history.contains_key(&(*p, *d))
        // history.keys().find(|(p1, d)| p1 == p && d==d).is_none()
    });
    points.iter().for_each(|(p, d)| {
        history.insert((p.clone(),*d), ());
        light[p.r as usize][p.c as usize] += 1;
    });
   // result
    points
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    r: isize,
    c: isize,
}

impl Point {
    fn new(r: isize, c: isize) -> Self {
        Self { r, c }
    }
    fn up(&self) -> Self {
        Self { r: self.r - 1, c: self.c }
    }
    fn down(&self) -> Self {
        Self { r: self.r + 1, c: self.c }
    }
    fn left(&self) -> Self {
        Self { r: self.r, c: self.c - 1 }
    }
    fn right(&self) -> Self {
        Self { r: self.r, c: self.c + 1 }
    }
 }

fn parse_map(content: &str) -> Vec<Vec<char>> {
    let mut map = Vec::new();
    for line in content.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        map.push(row);
    }
    map
}