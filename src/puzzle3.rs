#[derive(Debug)]
pub struct DigitCapture {
    pub row: usize,
    pub column_range: (usize, usize),
    pub text: String,
    pub value: u32,
}

#[derive(Debug)]
pub struct SymbolCapture {
    pub row: usize,
    pub column: usize,
}

#[cfg(test)]
mod tests {
    use regex::{Captures, Regex};

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
            .map(|(idx, line)| {
                (
                    idx,
                    number_regex.captures_iter(line),
                )
            })
            .flat_map(|(row, captures)| {
                captures
                    .map(move |capture| {
                        let regex_match = capture.get(1).unwrap();
                        let text = regex_match.as_str().to_string();
                        let column_range = (regex_match.range().start, regex_match.range().end - 1);
                        let value = text.parse::<u32>().unwrap();
                        DigitCapture {
                            row: row,
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
        let symbols_capture: Vec<SymbolCapture> = input
            .lines()
            .enumerate()
            .map(|(idx, line)| {
                (
                    idx,
                    symbol_regex.captures_iter(line),
                )
            })
            .flat_map(|(row, captures)| {
                captures
                    .map(move |capture| {
                        let regex_match = capture.get(1).unwrap();
                        let column = regex_match.range().start;
                        SymbolCapture { row, column }
                    })
            })
            .collect();

        dbg!(&symbols_capture);
    }
}
