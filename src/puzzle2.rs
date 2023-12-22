use std::collections::HashMap;

use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum CubeColor {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
pub struct CubeColorParsingError;

impl TryFrom<&str> for CubeColor {
    type Error = CubeColorParsingError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.trim() {
            "red" => Ok(CubeColor::Red),
            "green" => Ok(CubeColor::Green),
            "blue" => Ok(CubeColor::Blue),
            _ => Err(CubeColorParsingError),
        }
    }
}

pub struct Game {
    id: i32,
    cubes: HashMap<CubeColor, i32>,
}

pub struct GameParseInputError;

impl TryFrom<&str> for Game {
    type Error = GameParseInputError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let game_regex = Regex::new(r"^Game (\d+)$").unwrap();
        let cube_regex = Regex::new(r"(\d+)\s(\w+)").unwrap();
        let (game_str, cube_set_str) = value.split_once(':').unzip();

        let game_id = game_str
            .and_then(|s| game_regex.captures(s))
            .and_then(|capture| capture.get(1).and_then(|r| Some(r.as_str())));

        let cubes = cube_set_str
            .map(|str| str.split(','))
            .map(|split| split.map(|s| s.trim()))
            .map(|m| {
                m.map(|s| {
                    let cubes = cube_regex.captures(s);
                    let Some((Some(amount), Some(color))) =
                        cubes.map(|c| (c.get(1), c.get(2))).map(|(amount, color)| {
                            (amount.map(|a| a.as_str()), color.map(|c| c.as_str()))
                        }) else { panic!("Parse Input Error"); };
                    (amount, CubeColor::try_from(color).unwrap())
                })
            });
        // .and_then(f)
        // .split(';')
        // .flat_map(|set| set.split(','))
        // .and_then(|s| cube_regex.captures(';'));

        Err(GameParseInputError)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use regex::Regex;

    use crate::puzzle2::Game;

    use super::CubeColor;

    #[test]
    fn parse_game_struct() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game_cubes = input.split_once(':').unwrap();
        let regex_game_match = Regex::new(r"^Game (\d+)$").unwrap();
        let captures = regex_game_match.captures(game_cubes.0).unwrap();

        let game_id = captures.get(1).unwrap().as_str();
        assert!(game_id == "1");

        let game = Game {
            id: 0,
            cubes: HashMap::new(),
        };

        let regex_cubes_match = Regex::new(r"(\d+)\s(\w+)").unwrap();

        let cube_amounts: Vec<(i32, CubeColor)> = game_cubes
            .1
            .split(';')
            .flat_map(|set| set.split(','))
            .map(|s| s.trim())
            .map(|s| {
                let cubes = regex_cubes_match.captures(s).unwrap();
                let amount = cubes.get(1).unwrap().as_str().parse::<i32>().unwrap();
                let color = cubes.get(2).unwrap().as_str();
                (amount, CubeColor::try_from(color).unwrap())
            })
            .collect();

        assert!(cube_amounts[0] == (3, CubeColor::Blue));
        assert!(cube_amounts[1] == (4, CubeColor::Red));
        assert!(cube_amounts[2] == (1, CubeColor::Red));
        assert!(cube_amounts[3] == (2, CubeColor::Green));
    }

    #[test]
    fn parse_cube_color() {
        let color_str = "blue";
        let cube_color = CubeColor::try_from(color_str);

        assert!(cube_color.is_ok());
        assert!(cube_color.unwrap() == CubeColor::Blue);

        let wrong_color_str = "yellow";
        let wrong_cube_color = CubeColor::try_from(wrong_color_str);

        assert!(wrong_cube_color.is_err())
    }
}
