#[derive(Debug, PartialEq)]
struct AlmanacMap(Vec<(u64, u64, u64)>);

impl AlmanacMap {
    pub fn process_map(&self, seed: u64) -> u64 {
        // The set of maps should by sorted by the source_range
        // If none of the source_range start at 0, make an additional range from 0 to the
        // earlier source_range and insert it at the start of the list.
        let mut output = seed;
        for &(dest, source, range) in self.0.iter() {
            if seed >= source && seed < (source + range) {
                output = (seed - source) + dest;
            }
        }
        output
    }
}

impl TryFrom<&str> for AlmanacMap {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut triples = vec![];
        for line in value.lines() {
            if line.chars().next().unwrap().is_ascii_digit() {
                let nums: Vec<Result<u64, Self::Error>> = line
                    .split(' ')
                    .map(|n| n.parse::<u64>().map_err(|_| "Can't parse map number"))
                    .collect();
                triples.push((nums[0]?, nums[1]?, nums[2]?));
            }
        }
        triples.sort_by_key(|t| t.1);

        Ok(AlmanacMap(triples))
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzle5::AlmanacMap;

    #[test]
    fn test_puzzle_answer_part_1() {
        let puzzle_input = r#"
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
        "#
        .trim();

        // Maps are used to convert from a source category to a destination category.
        // The first map (seed-to-soil) describes how to convert a seed number (the source)
        // to a soil number (the destination).
        //
        // Each map contains: the destination range start, the source range start and the range
        // length.
        //
        // With the map, you can look up the soil number required for each initial seed number.
        //
        // Source numbers that aren't mapped correspond to the same destination number. So,
        // seed number 10 corresponds to soil number 10.
        //
        // The whole map has to be laid out as a single list (or range) of range of numbers. That's
        // why there are several maps.
        //
        // TODO: Find the lowest location number that corresponds to any of the initial seeds.
        //
        // ======= Parse input ==========
        // 1 - Get seeds from the first line of the input.
        // 2 - Make an enum that contains a vec of triples. Called AlmanacMap.
        // 2.1 - Create an entry for each map.
        // 3 - Iterate over the lines of the inputs.
        // 3.1 - If it starts with "seeds:" then parse the rest of the string to capture the numbers.
        //   Put them in a vec<u64>.
        // 3.2 - If it starts with a character, then evaluate the TryFrom from the AlmanacMap.
        // 3.3 - If it starts with a number, start adding truples to a new vec<u64>.
        // 3.4 - If it is a blank line or end of string, stop adding truples to the new vec<u64>.
        // ======= Evaluate the seed and maps
        // 4 - Work from inside to the outside.
        // 5 - Make a function that takes a number and a list of truples. Return the mapped number
        //   corresponding to the map, or the value itself otherwise.
        // 6 - Make a function that does the above with a single seed and a vec of truples coming
        //   from an AlmanacMap enum.
    }

    #[test]
    fn test_seed_to_map_process() {
        let seed = 79;
        let mut map = vec![(50, 98, 2), (52, 50, 48)];
        map.sort_by_key(|m| m.0);

        let seed_to_soil_map = AlmanacMap(map);
        let output = seed_to_soil_map.process_map(seed);

        assert_eq!(output, 81);
    }

    #[test]
    fn test_parse_map_input() {
        let input = "seed-to-soil map:\n\
            50 98 2\n\
            52 50 48";

        let almanac_map = AlmanacMap::try_from(input).unwrap();

        assert_eq!(almanac_map, AlmanacMap(vec![(52, 50, 48), (50, 98, 2)]));
    }
}
