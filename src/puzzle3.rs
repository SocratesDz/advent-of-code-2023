#[cfg(test)]
mod tests {
    use regex::{CaptureLocations, CaptureMatches, Captures, Match, Regex};

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
        let digit_capture_slices = input
            .lines()
            .enumerate()
            .map(|(idx, line)| {
                (
                    idx,
                    number_regex.captures_iter(line).collect::<Vec<Captures>>(),
                )
            })
            .collect::<Vec<(usize, Vec<Captures>)>>();
        dbg!(&digit_capture_slices);

        // Get every symbol
        let symbol_regex = Regex::new(r"([^\.\d\w\s])++").unwrap();
        let symbols_capture = input
            .lines()
            .enumerate()
            .map(|(idx, line)| {
                (
                    idx,
                    symbol_regex.captures_iter(line).collect::<Vec<Captures>>(),
                )
            })
            .collect::<Vec<(usize, Vec<Captures>)>>();
        dbg!(&symbols_capture);

        // for (idx, symbols) in symbols_capture
        // / for capture in symbols:
        // // let capture_index = capture.1.slice_start;
        // // check rows capture_index-1 and capture_index+1;
        // // check columns idx-1..idx+1;
        // ///
        //
        // for (row, symbols) in symbols_capture
        // / let digit = digit_capture_slices.find(|(digit_row, digit_capture)| { digit_row == row && symbols.slice.first in digit_capture.slice })

        for (row, symbols) in &symbols_capture {
            for capture in symbols {
                let start_index = capture.get(1).unwrap().start();
                let digits = &digit_capture_slices.iter().filter(|(digit_row, captures)| {
                    (&(digit_row - 1) == row || &(digit_row + 1) == row)
                        && !(captures
                            .iter()
                            .filter(|&digit_capture| {
                                digit_capture.get(1).iter().any(|&digit_regex_match| {
                                    digit_regex_match.range().contains(&(&start_index - 1))
                                        || digit_regex_match.range().contains(&start_index)
                                        || digit_regex_match.range().contains(&(&start_index + 1))
                                })
                            })
                            .collect::<Vec<&Captures>>()
                            .is_empty())
                });
                dbg!(&digits);
            }
        }
    }
}
