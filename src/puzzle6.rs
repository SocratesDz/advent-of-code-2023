use std::fs;

pub fn parse_input(input: &str) -> Vec<Record> {
    let mut lines = input.lines().into_iter();
    let time_records = lines
        .next()
        .map(|l| l.split_once("Time:"))
        .flatten()
        .map(|t| t.1.split_whitespace().collect::<Vec<&str>>())
        .expect("Error while parsing time records");
    let distance_records = lines
        .next()
        .map(|l| l.split_once("Distance:"))
        .flatten()
        .map(|t| t.1.split_whitespace().collect::<Vec<&str>>())
        .expect("Error while parsing distance records");
    time_records
        .iter()
        .zip(distance_records.iter())
        .map(|records| Record {
            time: records
                .0
                .parse::<u32>()
                .expect("Error while parsing time records"),
            distance: records
                .1
                .parse::<u32>()
                .expect("Error while parsing time records"),
        })
        .collect()
}

#[derive(Debug, PartialEq)]
pub struct Record {
    pub time: u32,
    pub distance: u32,
}

impl Record {
    pub fn speed(&self) -> f32 {
        if self.time == 0 {
            return 0.0;
        }
        self.distance as f32 / self.time as f32
    }
}

pub fn answer_part_1(input: &str) -> u32 {
    let records = parse_input(input);
    let speed_values = records
        .iter()
        .map(|record| {
            let speed = if record.speed() == record.speed().ceil() as f32 {
                record.speed() + 1.0
            } else {
                record.speed().ceil()
            } as u32;
            let initial_speed = (speed..)
                .find(|s| s * (record.time - s) > record.distance)
                .expect("Can't calculate optimal speed.");
            let speeds = (initial_speed..)
                .take_while(|s| s * (record.time - s) > record.distance)
                .collect::<Vec<_>>();
            speeds.len() as u32
        })
        .collect::<Vec<u32>>();
    speed_values.iter().product()
}

pub fn answer() -> (u32, u32) {
    let input = fs::read_to_string("puzzle6.txt").expect("Puzzle file not found.");
    (answer_part_1(input.as_str()), 0)
}

#[cfg(test)]
mod tests {
    use crate::puzzle6::{answer_part_1, parse_input, Record};

    #[test]
    fn test_puzzle_answer_part_1() {
        // The input represents a table of boat races. Each column is a different race. The rows
        // contain Time and Distance.
        // For each race, the time represents the amount of time in milliseconds the race lasted. The
        // distance value is the distance in millimeters that the boat ran during the race.
        //
        // At the start of the race, a button is pressed. For every millisecond pressed, the boat
        // will increase initial speed for 1 millimeter per millisecond. This means, if the button
        // is held for five milliseconds, the boat will start the race at a speed of 5 millimeters
        // per millisecond.
        //
        // Find the number of ways a given record could be beaten for each race. Then multiply them
        // together.
        let puzzle_input = "Time:      7  15   30\nDistance:  9  40  200";

        // 1. Determine the speed ran by the boat of the record.
        //   - v = d/t ; let speed = 9/7 => speed = 1.2857
        // 2. Determine the maximum speed under the record time.
        //   - The final speed has to be an u32, therefore, we ceil the value of the record, then final_speed = 2;
        //   - if final_speed == speed: final_speed += 1
        //   - let speed_values = [final_speed..].take_while(|s| s*(time-s) > distance)
        // 3. Count the amount of speed possibilities on between the record and the maximum
        //    possible speed.
        //   - let result = speed_values.length()
        // 4. Multiply them.
        //   - results.product()
        let result: u32 = answer_part_1(puzzle_input);
        assert_eq!(result, 288);
    }

    #[test]
    fn test_parse_puzzle_input() {
        // Parse time line
        // if line starts_with "Time:", split line on such prefix. Then split the second element on
        // whitespace.
        //
        // Parse distance line
        // Do the same as with the time line, but use the prefix "Distance:"
        //
        let input = "Time:      7  15   30\nDistance:  9  40  200";
        let records = parse_input(input);

        assert_eq!(
            records,
            vec![
                Record {
                    time: 7,
                    distance: 9
                },
                Record {
                    time: 15,
                    distance: 40
                },
                Record {
                    time: 30,
                    distance: 200
                }
            ]
        );
    }
}
