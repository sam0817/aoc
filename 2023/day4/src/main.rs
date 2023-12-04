use std::collections::HashMap;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input").expect("Should have been able to read the file");

    // part 1
    part1(&contents[..]);

    // part 2
    part2(&contents[..]);
}

fn part1(contents: &str) {
    let mut sum = 0;
    for line in contents.lines() {
        sum += split_line(line);
    }
    println!("Result: {}", sum);
}

fn point_fn(i: i32) -> i32 {
    let base: i32 = 2;
    if i == 0 {
        0
    } else if i == 1 {
        1
    } else {
        2 * point_fn(i - 1)
    }
}

fn parse_ints(line: &str) -> Vec<i32> {
    // println!("line:{}", line);
    line.trim().split(" ")
        .collect::<Vec<&str>>()
        .iter()
        .filter(|x| x.len() > 0)
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

fn split_line(line: &str) -> i32 {
    let mut count = 0;
    let split = line.split(":").collect::<Vec<&str>>()[1].trim();
    let mut split = split.split("|").collect::<Vec<&str>>();
    let win_list = parse_ints(split[0]);
    let owned = parse_ints(split[1]);
    owned.iter().for_each(|x| {
        if win_list.contains(x) {
            count += 1;
        }
    });
    point_fn(count)
}

fn part2(contents: &str) {
    let mut card_transform = HashMap::<i32, Vec<i32>>::new();
    let mut card_in_hand = HashMap::<i32, i32>::new();
    for line in contents.lines() {
        let card = split_line_to_card(line);
        // println!("{:?}", card);
        card_in_hand.insert(card.0, card.1);
    }
    let mut cards = Vec::<Card>::new();
    card_in_hand.iter().for_each(|(k, v)| {
        let n = card_map(*k, *v);
        card_transform.insert(*k, n.clone());
        cards.push(Card { no: *k, is_open: false })
    });
    // println!("card_transform: {:?}", card_transform);
    open_cards(cards, card_transform);
}

fn card_map(no: i32, pt: i32) -> Vec<i32> {
    if pt == 0 {
        vec![]
    } else {
        (1..=pt)
            .into_iter()
            .map(|x| x + no)
            .collect()
    }
}

fn split_line_to_card(line: &str) -> (i32, i32) {
    let mut count = 0;
    let split = line.split(":").collect::<Vec<&str>>();
    let card_no = split[0].replace("Card", "").trim().parse::<i32>().unwrap();
    let mut split = split[1].trim().split("|").collect::<Vec<&str>>();
    let win_list = parse_ints(split[0]);
    let owned = parse_ints(split[1]);
    owned.iter().for_each(|x| {
        if win_list.contains(x) {
            count += 1;
        }
    });
    (card_no, count)
}

#[derive(Debug, Clone, Copy)]
struct Card {
    no: i32,
    is_open: bool,
}

fn open_cards(cards: Vec<Card>, tran_table: HashMap<i32, Vec<i32>>) {
    let mut all = cards.to_vec();
    // for _ in 0..3 {
    loop {
        if all.iter().all(|x| x.is_open) {
            break;
        }
        let mut new_added = Vec::<i32>::new();
        all.iter_mut().for_each(|x| {
            if !x.is_open {
                x.is_open = true;
                let table = &mut tran_table.get(&x.no).unwrap().to_vec();
                // println!("table:{:?}", table);
                new_added.append(table);
            }
        });
        new_added.iter().for_each(|x| {
            all.push(Card { no: *x, is_open: false })
        });
        // println!("new:{:?}", new_added);
        // println!("all: {:?} added: {:?}", all.len(), new_added.len());
    }
    println!("Result: {:?}", all.len());
}