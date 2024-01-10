#[cfg(test)]
mod tests {
    use regex::{CaptureLocations, Regex, CaptureMatches, Captures};

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
        let symbol_regex = Regex::new(r"([^\.\d\w\s])+").unwrap();
        let symbols_capture = input
            .lines()
            .map(|line| symbol_regex.captures(line))
            .collect::<Vec<Option<Captures>>>();
        dbg!(symbols_capture);
    }
}
