use std::{collections::HashMap, fs};

pub fn get_input() -> String {
    fs::read_to_string("puzzle1.txt").expect("File not found.")
}

pub fn answer() -> u32 {
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

    let answer: u32 = get_input()
        .lines()
        .map(|l| {
            // find first number
            let first_digit_position = l.chars().position(|c| c.is_numeric()).unwrap();
            let first_word_position = number_names
                .iter()
                .filter_map(|(word, num)| l.find(word).map(|idx| (idx, num)))
                .min_by(|a, b| a.0.cmp(&b.0));

            let first_digit = match first_word_position {
                Some((word_position, char)) => {
                    if first_digit_position < word_position {
                        l.chars().nth(first_digit_position).unwrap()
                    } else {
                        *char
                    }
                }
                None => l.chars().nth(first_digit_position).unwrap(),
            };

            // find last number
            let last_digit_position = l.chars().rev().position(|c| c.is_numeric()).unwrap();
            let last_word_position = number_names
                .iter()
                .filter_map(|(word, num)| l.rfind(word).map(|idx| (idx, num)))
                .max_by(|a, b| a.0.cmp(&b.0));

            let last_digit = match last_word_position {
                Some((word_position, char)) => {
                    if (l.len() - last_digit_position - 1) > word_position {
                        l.chars().nth_back(last_digit_position).unwrap()
                    } else {
                        *char
                    }
                }
                None => l.chars().nth_back(last_digit_position).unwrap(),
            };

            // DEBUG info
            // println!("-----------");
            // println!("Line: {}", l);
            // println!("Digit: {}{}", first_digit, last_digit);
            // println!("First digit position: {}", first_digit_position);
            // println!("First word position: {:?}", first_word_position);
            // println!("Last digit position: {}", last_digit_position);
            // println!("Last word position: {:?}", last_word_position);

            format!("{}{}", first_digit, last_digit)
        })
        .map(|n| n.parse::<u32>().unwrap())
        .sum();
    answer
}
