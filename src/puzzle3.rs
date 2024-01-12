#[cfg(test)]
mod tests {
    use regex::{CaptureLocations, Regex, CaptureMatches, Captures, Match};

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
                let mut locations = number_regex.capture_locations();
                number_regex.captures_read(&mut locations, line);
                (idx, locations)
            })
            .collect::<Vec<(usize, CaptureLocations)>>();
        dbg!(digit_capture_slices);

        // Get every symbol
        let symbol_regex = Regex::new(r"([^\.\d\w\s])++").unwrap();
        let symbols_capture = input
            .lines()
            .enumerate()
            .map(|(idx, line)| (idx, symbol_regex.captures_iter(line).collect::<Vec<Captures>>()))
            .collect::<Vec<(usize, Vec<Captures>)>>();
        dbg!(symbols_capture);

        // for (idx, symbols) in symbols_capture
        // / for capture in symbols:
        // // let capture_index = capture.1.slice_start;
        // // check rows capture_index-1 and capture_index+1;
        // // check columns idx-1..idx+1;
        // // 

    }
}
