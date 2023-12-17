use std::{collections::HashMap, fs, ops::Index};

fn puzzle1() {
    let number_names: HashMap<&str, char> = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
        ("zero", '0'),
    ]);
    let file = fs::read_to_string("puzzle1.txt").expect("File not found.");
    let answer: u32 = file
        .lines()
        .map(|l| {
            let first_digit = l.chars().find(|c| c.is_numeric()).unwrap();
            let last_digit = l.chars().rfind(|c| c.is_numeric()).unwrap();
            format!("{}{}", first_digit, last_digit)

            // find first number, then find first word. compare indices. pick smaller one.
        })
        .map(|n| n.parse::<u32>().unwrap())
        .sum();
    println!("{answer}")
}

fn main() {
    puzzle1()
}
