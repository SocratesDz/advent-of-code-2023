use regex::Regex;

pub struct Scratchcard {
    pub id: u32,
    pub winning_numbers: Vec<u32>,
    pub owning_numbers: Vec<u32>,
}

#[derive(Debug)]
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
        let matched_numbers = self
            .owning_numbers
            .iter()
            .filter(|picked| self.winning_numbers.contains(picked))
            .count() as i32;

        if matched_numbers > 0 {
            2_u32.pow((matched_numbers - 1).try_into().unwrap())
        } else {
            0
        }
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

#[cfg(test)]
mod tests {
    use crate::puzzle4::Scratchcard;

    #[test]
    fn test_parse_scorecard() {
        let card_input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";

        let scratchcard = Scratchcard::try_from(card_input);

        assert!(scratchcard.is_ok())
    }
}
