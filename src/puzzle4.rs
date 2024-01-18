pub struct Scratchcard {
    pub id: u32,
    pub winning_numbers: Vec<u32>,
    pub owning_numbers: Vec<u32>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_scorecard() {
        let card_input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
    }
}
