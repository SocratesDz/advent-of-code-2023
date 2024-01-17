use std::{collections::HashSet, fs, ops::Range};

use regex::Regex;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct DigitCapture {
    pub row: isize,
    pub column_range: Range<isize>,
    pub text: String,
    pub value: u32,
}

impl DigitCapture {
    fn is_adjacent(&self, (row, column): (isize, isize)) -> bool {
        let row_range = (self.row - 1)..(self.row + 2);
        let column_range = (self.column_range.start - 1)..(self.column_range.end + 1);
        row_range.contains(&row) && column_range.contains(&column)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SymbolCapture {
    pub row: usize,
    pub column: usize,
    pub symbol: char,
}

pub fn parse_numbers(input: &str) -> Vec<DigitCapture> {
    let number_regex = Regex::new(r"(\d+)").expect("Wrong regex pattern");
    input
        .lines()
        .enumerate()
        .map(|(idx, line)| (idx, number_regex.captures_iter(line)))
        .flat_map(|(row, captures)| {
            captures.map(move |capture| {
                let regex_match = capture.get(1).expect("No match");
                let text = regex_match.as_str().to_string();
                let column_range =
                    (regex_match.range().start as isize)..(regex_match.range().end as isize);
                let value = text.parse::<u32>().expect("Can't parse number");
                DigitCapture {
                    row: row as isize,
                    column_range,
                    text,
                    value,
                }
            })
        })
        .collect()
}

pub fn parse_symbols(input: &str) -> Vec<SymbolCapture> {
    let symbol_regex = Regex::new(r"([^\.\d\w\s])++").expect("Wrong regex pattern");

    input
        .lines()
        .enumerate()
        .map(|(idx, line)| (idx, symbol_regex.captures_iter(line)))
        .flat_map(|(row, captures)| {
            captures.map(move |capture| {
                let regex_match = capture.get(1).expect("No match");
                let column = regex_match.range().start;
                let symbol = regex_match
                    .as_str()
                    .chars()
                    .nth(0)
                    .expect("Empty symbol string");
                SymbolCapture {
                    row,
                    column,
                    symbol,
                }
            })
        })
        .collect()
}

pub fn find_gears<'a>(
    numbers: &'a Vec<DigitCapture>,
    symbols: &'a Vec<SymbolCapture>,
) -> Vec<&'a SymbolCapture> {
    symbols
        .iter()
        .filter(|capture| {
            capture.symbol == '*'
                && numbers
                    .iter()
                    .filter(|number| {
                        number.is_adjacent((capture.row as isize, capture.column as isize))
                    })
                    .count()
                    == 2
        })
        .collect::<Vec<&'a SymbolCapture>>()
}

pub fn find_adjacents<'a>(
    symbol: &'a SymbolCapture,
    numbers: &'a Vec<DigitCapture>,
) -> Vec<&'a DigitCapture> {
    numbers
        .iter()
        .filter(|number| number.is_adjacent((symbol.row as isize, symbol.column as isize)))
        .collect()
}

pub fn answer() -> (i32, i32) {
    let input = fs::read_to_string("puzzle3.txt").expect("File not found.");

    // Get number locations with slices
    let digit_captures: Vec<DigitCapture> = parse_numbers(&input);

    // Get every symbol
    let symbol_captures: Vec<SymbolCapture> = parse_symbols(&input);

    let mut adjacent_numbers = HashSet::<&DigitCapture>::new();

    for symbol in &symbol_captures {
        digit_captures
            .iter()
            .filter(|number| number.is_adjacent((symbol.row as isize, symbol.column as isize)))
            .for_each(|number| {
                adjacent_numbers.insert(number);
            });
    }

    let answer_part1: i32 = adjacent_numbers
        .iter()
        .fold(0, |acc, number| acc + &number.value) as i32;

    ////////////////

    let gears = find_gears(&digit_captures, &symbol_captures);
    let mut adjacent_gear_numbers = HashSet::<&DigitCapture>::new();

    gears.iter().for_each(|gear| {
        digit_captures
            .iter()
            .filter(|number| number.is_adjacent((gear.row as isize, gear.column as isize)))
            .for_each(|number| {
                adjacent_gear_numbers.insert(number);
            })
    });

    let answer_part2: i32 = 0;

    (answer_part1, answer_part2)
}

#[cfg(test)]
mod tests {
    use std::ops::Range;

    use crate::puzzle3::{parse_numbers, parse_symbols, DigitCapture, SymbolCapture};

    const TEST_INPUT: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

    #[test]
    fn test_parse_numbers() {
        let digit_captures: Vec<DigitCapture> = parse_numbers(TEST_INPUT);

        assert!(
            digit_captures.first()
                == Some(&DigitCapture {
                    row: 0,
                    column_range: Range { start: 0, end: 3 },
                    text: String::from("467"),
                    value: 467
                })
        );
    }

    #[test]
    fn test_parse_symbols() {
        let symbol_capture: Vec<SymbolCapture> = parse_symbols(TEST_INPUT);

        assert!(
            symbol_capture.first()
                == Some(&SymbolCapture {
                    row: 1,
                    column: 3,
                    symbol: '*'
                })
        );
    }
}
