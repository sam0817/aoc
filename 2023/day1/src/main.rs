use std::fs;

fn main() {
    let contents = fs::read_to_string("input")
        .expect("Should have been able to read the file");

    let result: i32 = contents.lines().map(|line| {
        get_number_in_string(line.to_string())
        // get_number_with_text_in_string(line.to_string())
    }).sum();

    println!("Result: {}", result);

    let result: i32 = contents.lines().map(|line| {
        // get_number_in_string(line.to_string())
        get_number_with_text_in_string(line.to_string())
    }).sum();

    println!("Result: {}", result);
}


pub fn get_number_in_string(string: String) -> i32 {
    let mut first = None;
    let mut last = 0;
    for x in string.chars() {
        if x.is_digit(10) {
            let num = x.to_string().parse::<i32>().unwrap();
            if first.is_none() {
                first = Some(num);
            }
            last = num;
        }
    }

    first.unwrap() * 10 + last
}

pub fn get_number_with_text_in_string(string: String) -> i32 {
    let mut first = None;
    let mut last = 0;
    // println!("string: {}", string);
    for i in 0..string.len() {
        let slice = &string[i..];
        let num  =
            if slice.starts_with("1") { 1 }
            else if slice.starts_with("2") { 2 }
            else if slice.starts_with("3") { 3 }
            else if slice.starts_with("4") { 4 }
            else if slice.starts_with("5") { 5 }
            else if slice.starts_with("6") { 6 }
            else if slice.starts_with("7") { 7 }
            else if slice.starts_with("8") { 8 }
            else if slice.starts_with("9") { 9 }
            else if slice.starts_with("one") { 1 }
            else if slice.starts_with("two") { 2 }
            else if slice.starts_with("three") { 3 }
            else if slice.starts_with("four") { 4 }
            else if slice.starts_with("five") { 5 }
            else if slice.starts_with("six") { 6 }
            else if slice.starts_with("seven") { 7 }
            else if slice.starts_with("eight") { 8 }
            else if slice.starts_with("nine") { 9 }
            else { 0 }
            ;
        // println!("slice: {}", slice);
        // println!("num: {}", num);
        // println!("first: {:?}", first);
        // println!("last: {:?}", last);

        if num == 0 {
            continue;
        }
        if first.is_none() {
            first = Some(num);
        }
        last = num;
    }

    first.unwrap() * 10 + last
}


