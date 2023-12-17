use std::{fs, ops::Index};

const number_names: [&str;10] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "zero"
];

fn puzzle1() {
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
