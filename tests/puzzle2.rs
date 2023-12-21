#[cfg(test)]
mod puzzle2_test {
    use regex::Regex;

    #[test]
    fn parse_game_struct() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game_cubes = input.split_once(':').unwrap();
        let regex_game_match = Regex::new(r"^Game (\d+)$").unwrap();
        let captures = regex_game_match.captures(game_cubes.0).unwrap();

        let game_id = captures.get(1).unwrap().as_str();
        assert!(game_id == "1");
    }
}