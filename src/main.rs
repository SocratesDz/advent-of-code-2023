use std::fs;

const number_names: [&str;10] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "zero"
];

fn puzzle1() {
    let file = fs::read_to_string("puzzle1.txt").expect("File not found.");
    let answer: u32 = file
        .lines()
        .map(|l| {
            let first = l.chars().find(|c| c.is_numeric()).unwrap();
            let last = l.chars().rev().find(|c| c.is_numeric()).unwrap();
            format!("{}{}", first, last)
        })
        .map(|n| n.parse::<u32>().unwrap())
        .sum();
    println!("{answer}")
}

fn main() {
    puzzle1()
}
