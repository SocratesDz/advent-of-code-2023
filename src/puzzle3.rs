#[cfg(test)]
mod tests {
    use regex::Regex;

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
        let regex = Regex::new(r"(\d+)").unwrap();
        let digit_capture_slices = input
            .lines()
            .map(|line| {
                let captures = regex.captures_iter(line);
                let extracted = captures.map(|some_captures| some_captures.extract::<1>().1[0]);
                extracted.collect::<Vec<&str>>()
            })
            .flatten()
            .collect::<Vec<&str>>();
        dbg!(digit_capture_slices);
    }
}
