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

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
struct Cube {
    points: Vec<Point>,
}

impl Cube {
    fn new(p1: Point, p2: Point) -> Self {
        let mut points = vec![];
        for x in p1.x.min(p2.x)..=p1.x.max(p2.x) {
            for y in p1.y.min(p2.y)..=p1.y.max(p2.y) {
                for z in p1.z.min(p2.z)..=p1.z.max(p2.z) {
                    points.push(Point { x, y, z });
                }
            }
        }
        Cube { points }
    }
    fn down(&mut self) {
        if self.points.iter().any(|p| p.z == 0) {
            return;
        }
        self.points.iter_mut().for_each(|p| p.z -= 1);
    }
    fn up(&mut self) {
        self.points.iter_mut().for_each(|p| p.z += 1);
    }
    fn set_z(&mut self, z: i32) {
        let min_z = self.points.iter().map(|p| p.z).min().unwrap();
        let diff = z - min_z;
        self.points.iter_mut().for_each(|p| p.z += diff);
    }
}

fn parse(content: &str) -> Vec<((i32, i32, i32), (i32, i32, i32))> {
    let mut result = vec![];
    content
        .lines()
        .for_each(|line| {
            let sp = line.split('~').collect::<Vec<&str>>();
            let p1 = sp[0].split(',').map(|s| s.trim().parse::<i32>().unwrap()).collect::<Vec<i32>>();
            let p2 = sp[1].split(',').map(|s| s.trim().parse::<i32>().unwrap()).collect::<Vec<i32>>();
            let dist = vec![p1[0] - p2[0], p1[1] - p2[1], p1[2] - p2[2]];
            let diff = dist.iter().filter(|d| **d != 0).count();
            if diff > 1 { println!("diff: {:?}", diff); }
            result.push(((p1[0], p1[1], p1[2]), (p2[0], p2[1], p2[2])))
        });

    result
}

fn check_layer_collision(stack: &Vec<Cube>, cube: &Cube) -> bool {
    for i in 0..stack.len() {
        let c = &stack[i];
        if c.points.iter().any(|p| cube.points.contains(p)) {
            return true;
        }
    }
    false
}

fn check_layer_collision_2(stack: &Vec<&Cube>, cube: &Cube) -> bool {
    for i in 0..stack.len() {
        let c = stack[i];
        if c.points.iter().any(|p| cube.points.contains(p)) {
            return true;
        }
    }
    false
}

fn collision_points(stack: &Vec<Cube>, cube: &Cube) -> Vec<Point> {
    let mut result = vec![];
    stack.iter().for_each(|c| {
        c.points.iter().for_each(|p| {
            if cube.points.contains(p) {
                result.push(*p);
            }
        })
    });
    result
}

fn falling(cubes: &mut Vec<Cube>) -> Vec<Cube> {
    let mut stack: Vec<Cube> = vec![];
    for i in 0..cubes.len() {
        println!("i: {:?}", i);
        let mut z_max = stack.iter()
            .flat_map(|c| c.points.iter().map(|p| p.z)).max()
            .unwrap_or(1);
        println!("z_max: {:?}", z_max);
        let mut cube = cubes[i].clone();
        loop {
            cube.set_z(z_max);
            if check_layer_collision(&stack, &cube) {
                cube.set_z(z_max + 1);
                stack.push(cube.clone());
                break;
            }
            if z_max == 1 {
                stack.push(cube.clone());
                break;
            }
            z_max -= 1;
        }
    }
    stack
}

fn part1(content: &str) {
    let data = parse(content);

    let cubes = data.iter().map(|d| {
        let (l, r) = *d;
        Cube::new(Point { x: l.0, y: l.1, z: l.2 }, Point { x: r.0, y: r.1, z: r.2 })
    }).collect::<Vec<Cube>>();

    let stack = falling(&mut cubes.clone());

    stack.iter().for_each(print_cube);
    let mut sum = stack.len();
    'a: for remove_i in 0..stack.len() {
        let other = stack.iter().enumerate()
            .filter(|(j, _)| *j != remove_i)
            .map(|(_, c)| c).collect::<Vec<_>>();
        'b: for test_i in 0..other.len() {
            println!("remove_i: {:?}, test_i: {:?}", remove_i, test_i);
            let mut test_cube = other[test_i].clone();
            let test_cubes  = other.iter().enumerate().filter(|(j, _)| *j != test_i).map(|(_, c)| *c).collect::<Vec<_>>();
            test_cube.down();
            println!("test_cube: {:?}", test_cube);
            if check_layer_collision_2(&test_cubes, &test_cube) {
                // let points = collision_points(&stack, &test_cube);
                // println!("points: {:?}", points);
                continue 'b;
                // break 'b;
            } else {
                sum -= 1;
                break 'b;
            }
        }
    }

    println!("sum: {:?}", sum);
}

fn part2(content: &str) {}


fn print_cube(cube: &Cube) {
    let mut points = cube.points.clone();
    points.sort_by(|a, b| {
        if a.z == b.z {
            if a.y == b.y {
                a.x.cmp(&b.x)
            } else {
                a.y.cmp(&b.y)
            }
        } else {
            a.z.cmp(&b.z)
        }
    });
    for p in points.iter() {
        print!("({}, {}, {}), ", p.x, p.y, p.z);
    }
    println!();
}