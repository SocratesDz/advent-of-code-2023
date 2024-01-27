use std::fs;

use regex::Regex;

#[derive(Clone, Debug)]
pub struct Scratchcard {
    pub id: u32,
    pub winning_numbers: Vec<u32>,
    pub owning_numbers: Vec<u32>,
}

#[derive(Debug, Clone)]
pub struct ScratchcardError(&'static str);

impl Scratchcard {
    fn parse_numbers_string(
        number_match_regex: &Regex,
        number_str: &str,
    ) -> Result<Vec<u32>, ScratchcardError> {
        number_match_regex
            .captures_iter(number_str)
            .map(|number| {
                number
                    .get(1)
                    .unwrap()
                    .as_str()
                    .parse::<u32>()
                    .map_err(|_| ScratchcardError("Can't parse numbers from scratchcard string."))
            })
            .collect::<Result<Vec<u32>, ScratchcardError>>()
    }

    pub fn get_score(&self) -> u32 {
        let matched_numbers = self.get_matching_count();

        if matched_numbers > 0 {
            2_u32.pow((matched_numbers - 1).try_into().unwrap())
        } else {
            0
        }
    }

    pub fn get_matching_count(&self) -> i32 {
        self.owning_numbers
            .iter()
            .filter(|picked| self.winning_numbers.contains(picked))
            .count() as i32
    }
}

impl TryFrom<&str> for Scratchcard {
    type Error = ScratchcardError;

    // Expecting string in the format "Card ##: (#+)+ | (#+)+"
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let Some((_, card_numbers_str)) = value.split_once(':') else {
            return Err(ScratchcardError("Can't parse scratchcard string."));
        };

        let number_regex = Regex::new(r"(\d+)").expect("");

        let Some(card_numbers) =
            card_numbers_str
                .split_once('|')
                .map(|(winning_numbers, picked_numbers)| {
                    let winning = Scratchcard::parse_numbers_string(&number_regex, winning_numbers);
                    let picked = Scratchcard::parse_numbers_string(&number_regex, picked_numbers);
                    (winning, picked)
                })
        else {
            return Err(ScratchcardError("Can't parse scratchcard numbers."));
        };

        match card_numbers {
            (Ok(winning_numbers), Ok(owning_numbers)) => Ok(Scratchcard {
                id: 0,
                winning_numbers,
                owning_numbers,
            }),
            (Err(e), _) | (_, Err(e)) => Err(e),
        }
    }
}

pub fn answer() -> (i32, i32) {
    let input = fs::read_to_string("puzzle4.txt").expect("Puzzle file not found.");

    let scratchcards = input
        .lines()
        .map(Scratchcard::try_from)
        .collect::<Vec<Result<Scratchcard, ScratchcardError>>>();

    // Sum all scores of every scratchcard
    let answer_part_1 = &scratchcards
        .iter()
        .map(|card| card.as_ref().map(|c| c.get_score()))
        .fold(0, |acc, score| acc + score.unwrap_or(0));

    // Sum all scores (plus one) of every winning scratchcard
    let answer_part_2 = &scratchcards
        .iter()
        .map(|card| card.as_ref().map(|c| c.get_score() + 1))
        .fold(0, |acc, card_count| acc + card_count.unwrap_or(0));

    (*answer_part_1 as i32, *answer_part_2 as i32)
}

#[cfg(test)]
mod tests {
    use crate::puzzle4::Scratchcard;

    use super::ScratchcardError;

    #[test]
    fn test_parse_scorecard() {
        let card_input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";

        let scratchcard = Scratchcard::try_from(card_input);

        assert!(scratchcard.is_ok())
    }

    #[test]
    fn test_puzzle_answer_part_1() {
        let puzzle_input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

        let answer = puzzle_input
            .lines()
            .map(Scratchcard::try_from)
            .map(|card| card.map(|c| c.get_score()))
            .fold(0, |acc, count| acc + count.unwrap_or(0));

        assert!(answer == 13)
    }

    #[test]
    fn test_puzzle_answer_part_2() {
        let puzzle_input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

        let mut scratchcards = puzzle_input
            .lines()
            .map(Scratchcard::try_from)
            .collect::<Vec<Result<Scratchcard, ScratchcardError>>>();

        let mut card_count = 0;
        let mut cards_idx: usize = 0;

        // while cards_idx < scratchcards.len() {
        let Some(card) = scratchcards.get(cards_idx) else {
            return;
        };
        match card {
            Ok(c) => {
                let matching_count = c.get_matching_count();
                card_count += matching_count + 1;
                if matching_count > 0 {
                    let mut cards = scratchcards.clone();
                    let (before, after) = cards.split_at_mut(cards_idx + 1);

                    dbg!(&before);
                    // dbg!(cards);
                    dbg!(&after);

                    dbg!(&matching_count);
                    dbg!(&cards_idx);
                    dbg!(cards_idx + (matching_count as usize));

                    let mut next_cards = Vec::new();
                    after[0..(matching_count as usize)].clone_into(&mut next_cards);
                    let mut new_cards = before.to_vec();
                    new_cards.append(&mut next_cards);
                    new_cards.append(&mut after.to_vec());
                    scratchcards = new_cards;
                }
            }
            Err(_) => {
                todo!("Handle error")
            }
        }
        cards_idx += 1;
        // }

        assert!(card_count == 30)
    }
}
