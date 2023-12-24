use std::{collections::HashMap, fs};

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

#[derive(Debug)]
pub struct Game {
    pub id: i32,
    pub cubes: Vec<HashMap<CubeColor, i32>>,
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

        let Some(cubes_str) = cube_set_str.map(|str| {
            str.split(';')
                .map(|s| s.split(',').collect::<Vec<&str>>())
                .collect::<Vec<Vec<&str>>>()
        }) else {
            return Err(GameParseInputError);
        };

        let cube_amounts = cubes_str
            .iter()
            .map(|record| {
                let parsed_sets = record
                    .iter()
                    .map(|set| {
                        let match_results = cube_regex.captures(set);
                        let result = match_results
                            .map(|captures| captures.get(2).zip(captures.get(1)))
                            .and_then(|pair| {
                                pair.map(|(color, amount)| (color.as_str(), amount.as_str()))
                            })
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

                let mut map_set = HashMap::<CubeColor, i32>::new();

                parsed_sets.iter().for_each(|set| {
                    map_set.insert(set.0, set.1);
                });
                map_set
            })
            .collect::<Vec<HashMap<CubeColor, i32>>>();

        Ok(Game {
            id: game_id,
            cubes: cube_amounts,
        })
    }
}

impl Game {
    pub fn power(&self) -> i32 {
        let minimum_red = self
            .cubes
            .iter()
            .max_by(|a, b| {
                a.get(&CubeColor::Red)
                    .unwrap_or(&0)
                    .cmp(b.get(&CubeColor::Red).unwrap_or(&0))
            })
            .and_then(|cube| cube.get(&CubeColor::Red))
            .unwrap_or(&1);

        let minimum_green = self
            .cubes
            .iter()
            .max_by(|a, b| {
                a.get(&CubeColor::Green)
                    .unwrap_or(&0)
                    .cmp(b.get(&CubeColor::Green).unwrap_or(&0))
            })
            .and_then(|cube| cube.get(&CubeColor::Green))
            .unwrap_or(&1);

        let minimum_blue = self
            .cubes
            .iter()
            .max_by(|a, b| {
                a.get(&CubeColor::Blue)
                    .unwrap_or(&0)
                    .cmp(b.get(&CubeColor::Blue).unwrap_or(&0))
            })
            .and_then(|cube| cube.get(&CubeColor::Blue))
            .unwrap_or(&1);

        minimum_red * minimum_green * minimum_blue
    }
}

pub fn answer() -> (i32, i32) {
    let max_red_cubes = 12;
    let max_green_cubes = 13;
    let max_blue_cubes = 14;

    let input = fs::read_to_string("puzzle2.txt").expect("File not found");
    let games = input
        .lines()
        .map(|l| Game::try_from(l).unwrap())
        .collect::<Vec<Game>>();

    let sum_of_games: i32 = games
        .iter()
        .filter(|game| {
            game.cubes.iter().all(|set| {
                set.get(&CubeColor::Red) <= Some(&max_red_cubes)
                    && set.get(&CubeColor::Green) <= Some(&max_green_cubes)
                    && set.get(&CubeColor::Blue) <= Some(&max_blue_cubes)
            })
        })
        .map(|game| game.id)
        .sum();

    let power_of_games = games.iter().map(|game| game.power()).sum();

    (sum_of_games, power_of_games)
}

#[cfg(test)]
mod tests {
    use crate::puzzle2::Game;

    use super::CubeColor;

    const TEST_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

    #[test]
    fn parse_game_struct() {
        let game = Game::try_from(TEST_INPUT).unwrap();

        assert!(game.id == 1);
        assert!(game.cubes[0].get(&CubeColor::Blue) == Some(&3));
        assert!(game.cubes[0].get(&CubeColor::Red) == Some(&4));

        assert!(game.cubes[1].get(&CubeColor::Red) == Some(&1));
        assert!(game.cubes[1].get(&CubeColor::Green) == Some(&2));
        assert!(game.cubes[1].get(&CubeColor::Blue) == Some(&6));

        assert!(game.cubes[2].get(&CubeColor::Green) == Some(&2));
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

    #[test]
    fn test_game_power() {
        let game = Game::try_from(TEST_INPUT).unwrap();

        assert!(game.power() == 48)
    }
}
