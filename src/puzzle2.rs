use std::collections::HashMap;

use regex::Regex;

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
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

#[derive(Debug)]
pub struct GameParseInputError;

impl TryFrom<&str> for Game {
    type Error = GameParseInputError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let game_regex = Regex::new(r"^Game (\d+)$").unwrap();
        let cube_regex = Regex::new(r"(\d+)\s(\w+)").unwrap();
        let (game_str, cube_set_str) = value.split_once(':').unzip();

        let Some(game_id) = game_str
            .and_then(|s| game_regex.captures(s))
            .and_then(|capture| capture.get(1).map(|r| r.as_str()))
            .map(|id| id.parse::<i32>().expect("Error parsing game id"))
        else {
            return Err(GameParseInputError);
        };

        let Some(cubes_str) = cube_set_str
            .map(|str| str.split(';').flat_map(|s| s.split(',')))
            .map(|split| split.map(|s| s.trim()).collect::<Vec<&str>>())
        else {
            return Err(GameParseInputError);
        };

        let cube_amounts = cubes_str
            .iter()
            .map(|&c| {
                let match_results = cube_regex.captures(c);
                let result = match_results
                    .map(|c| c.get(2).zip(c.get(1)))
                    .map(|pair| pair.map(|(color, amount)| (color.as_str(), amount.as_str())))
                    .flatten()
                    .map(|(color, amount)| {
                        (
                            CubeColor::try_from(color)
                                .map_err(|_| GameParseInputError)
                                .expect("Error parsing cube color"),
                            amount
                                .parse::<i32>()
                                .map_err(|_| GameParseInputError)
                                .expect("Error parsing cube amount"),
                        )
                    });
                result.expect("Error parsing game")
            })
            .collect::<Vec<(CubeColor, i32)>>();

        let mut parsed_cube_sets: HashMap<CubeColor, i32> = HashMap::new();
        for (color, amount) in cube_amounts {
            if parsed_cube_sets.contains_key(&color) {
                if let Some(value) = parsed_cube_sets.get_mut(&color) {
                    *value += amount;
                }
            } else {
                parsed_cube_sets.insert(color, amount);
            }
        }
        Ok(Game {
            id: game_id,
            cubes: parsed_cube_sets,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzle2::Game;

    use super::CubeColor;

    #[test]
    fn parse_game_struct() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game = Game::try_from(input).unwrap();

        assert!(game.id == 1);
        assert!(game.cubes.get(&CubeColor::Blue) == Some(&9));
        assert!(game.cubes.get(&CubeColor::Red) == Some(&5));
        assert!(game.cubes.get(&CubeColor::Green) == Some(&4));
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
