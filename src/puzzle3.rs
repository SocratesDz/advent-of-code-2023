#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DigitCapture {
    pub row: isize,
    pub column_range: (isize, isize),
    pub text: String,
    pub value: u32,
}

impl DigitCapture {
    fn is_adjacent(&self, (row, column): (isize, isize)) -> bool {
        let row_range = (self.row - 1)..(self.row + 2);
        let column_range = (self.column_range.0 - 1)..(self.column_range.1 + 2);
        row_range.contains(&row) && column_range.contains(&column)
    }
}

#[derive(Debug)]
pub struct SymbolCapture {
    pub row: usize,
    pub column: usize,
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use regex::Regex;

    use crate::puzzle3::{DigitCapture, SymbolCapture};

    #[test]
    fn test_puzzle() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
        dbg!(input);

        // Get number locations with slices
        let number_regex = Regex::new(r"(\d+)").unwrap();
        let digit_captures: Vec<DigitCapture> = input
            .lines()
            .enumerate()
            .map(|(idx, line)| (idx, number_regex.captures_iter(line)))
            .flat_map(|(row, captures)| {
                captures.map(move |capture| {
                    let regex_match = capture.get(1).unwrap();
                    let text = regex_match.as_str().to_string();
                    let column_range = (
                        regex_match.range().start as isize,
                        regex_match.range().end as isize - 1,
                    );
                    let value = text.parse::<u32>().unwrap();
                    DigitCapture {
                        row: row as isize,
                        column_range,
                        text,
                        value,
                    }
                })
            })
            .collect();

        dbg!(&digit_captures);

        // Get every symbol
        let symbol_regex = Regex::new(r"([^\.\d\w\s])++").unwrap();
        let symbol_captures: Vec<SymbolCapture> = input
            .lines()
            .enumerate()
            .map(|(idx, line)| (idx, symbol_regex.captures_iter(line)))
            .flat_map(|(row, captures)| {
                captures.map(move |capture| {
                    let regex_match = capture.get(1).unwrap();
                    let column = regex_match.range().start;
                    SymbolCapture { row, column }
                })
            })
            .collect();

        dbg!(&symbol_captures);

        let mut number_matches: HashSet<&DigitCapture> = HashSet::new();

        for symbol in symbol_captures {
            digit_captures
                .iter()
                .filter(|number| number.is_adjacent((symbol.row as isize, symbol.column as isize)))
                .for_each(|number| {
                    number_matches.insert(number);
                });
        }

        dbg!(&number_matches);

        let answer: u32 = number_matches
            .iter()
            .fold(0, |acc, number| acc + &number.value);

        dbg!(answer);
    }
}
