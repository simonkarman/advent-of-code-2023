use std::cmp;

#[derive(Debug, Copy, Clone)]
pub struct Range {
    from: u32,
    to: u32,
}


impl Range {
    fn merge(mut ranges: Vec<Range>) -> Vec<Range> {
        let initial_range_count = ranges.len();
        ranges.sort_by_key(|range| range.from);
        let mut range_index = 0;
        let mut merged_ranges = vec![];
        while range_index < initial_range_count {
            let range = ranges[range_index];

            // if this is the last range, add the remaining
            if range_index == initial_range_count - 1 {
                merged_ranges.push(Range { from: range.from, to: range.to })
            // if the range ends before the next range starts, add it
            } else {
                let mut to = range.to;
                while to >= ranges[range_index + 1].from {
                    to = cmp::max(to, ranges[range_index + 1].to);
                    range_index += 1;
                    if range_index > initial_range_count - 2 {
                        break;
                    }
                }
                merged_ranges.push(Range { from: range.from, to });
            }
            range_index += 1;
        }
        return merged_ranges;
    }
}

#[derive(Debug)]
pub struct Map {
    destinations: Vec<u32>,
    sources: Vec<u32>,
    lengths: Vec<u32>,
}

impl Map {
    fn convert(&self, value: u32) -> u32 {
        for i in 0..self.sources.len() {
            let source = self.sources[i];
            if value < source || value >= source + self.lengths[i] {
                continue;
            }
            return self.destinations[i] + (value - source);
        }
        return value;
    }
    fn find_next_rule_from(&self, value: u32) -> Option<(
        /* index of the first rule found from value */ usize,
        /* whether value lies inside this rule */ bool
    )> {
        return self.sources.iter().enumerate()
            .map(|(index, source)| (index, (source + self.lengths[index]) as i64 - (value as i64)))
            .filter(|(_, v)| *v > 0)
            .min_by_key(|(_, v)| *v)
            .map(|(index, _)| (index, value >= self.sources[index]));
    }
    fn convert_range(&self, range: Range) -> Vec<Range> {
        let mut ranges = vec![];
        let mut from = range.from;
        loop {
            // Try and find the next rule for the form value
            let rule = self.find_next_rule_from(from);

            if rule.is_none() {
                // If no next rule is found, this means that the rest of the range lies after all rules
                ranges.push(Range { from, to: range.to });

                // And we're done
                break;
            }
            let (rule_index, is_within) = rule.unwrap();
            let rule_source = self.sources[rule_index];
            if !is_within {
                // If we're not within the rule, create a range until the next rule
                let to = rule_source;

                // If we're at the end of our range before we hit the next rule, then we're done
                if range.to <= to {
                    ranges.push(Range { from, to: range.to });
                    break;
                }

                // Otherwise go create range until the start of the next rule
                ranges.push(Range { from, to });

                // And continue...
                from = to;
            }

            // Now we're within a rule, find the end of rule or end of range whichever comes first
            let rule_length = self.lengths[rule_index];
            let to = cmp::min(range.to, rule_source + rule_length);

            // Add the destination range
            let rule_destination = self.destinations[rule_index];
            ranges.push(Range {
                from: rule_destination + (from - rule_source), // note -- this doesn't have to be the start of the destination rule, as we could have started in the middle of a rule
                to: rule_destination + (to - rule_source),
            });

            // If we're at the end of our range before we hit the end of the rule, then we're done
            if to == range.to {
                break;
            }

            // continue
            from = to;
        }
        return ranges;
    }
}

#[derive(Debug)]
pub struct Almanac {
    pub(crate) seeds: Vec<u32>,
    maps: Vec<Map>,
}

impl Almanac {
    fn convert(&self, value: u32) -> u32 {
        return self.maps.iter().fold(value, |acc, map| map.convert(acc));
    }
    pub(crate) fn convert_range(&self, range: Range) -> Vec<Range> {
        return self.maps.iter().fold(
            vec![range],
            |ranges, map| {
                return Range::merge(ranges.iter().flat_map(|range| map.convert_range(*range)).collect());
            }
        );
    }
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Almanac {
    let lines: Vec<&str> = input.lines().collect();
    let seeds = lines[0].strip_prefix("seeds:").unwrap().split_whitespace().map(|seed| seed.parse().unwrap()).collect();
    let mut maps = vec![];
    let mut destinations = vec![];
    let mut sources = vec![];
    let mut lengths = vec![];
    for i in 1..lines.len() {
        let line = lines[i];
        if line.chars().count() == 0 {
            continue;
        }
        if line.contains("map") {
            if !sources.is_empty() {
                maps.push(Map { destinations: destinations.clone(), sources: sources.clone(), lengths: lengths.clone() });
                destinations.clear();
                sources.clear();
                lengths.clear();
            }
            continue;
        }
        let mut numbers = line.split(' ').map(|number| number.parse().unwrap());
        destinations.push(numbers.next().unwrap());
        sources.push(numbers.next().unwrap());
        lengths.push(numbers.next().unwrap());
    };
    if !sources.is_empty() {
        maps.push(Map { destinations: destinations.clone(), sources: sources.clone(), lengths: lengths.clone() });
    }
    return Almanac {
        seeds,
        maps,
    }
}

#[aoc(day5, part1)]
pub fn part1(almanac: &Almanac) -> u32 {
    return almanac.seeds.iter().map(|seed| almanac.convert(*seed)).min().unwrap();
}

#[aoc(day5, part2, ranges)]
pub fn part2_sets(almanac: &Almanac) -> u32 {
    let start_ranges: Vec<Range> = almanac.seeds.chunks(2).map(|range| Range { from: range[0], to: range[0] + range[1] }).collect();
    let end_ranges: Vec<Range> = start_ranges.iter().flat_map(|range| almanac.convert_range(*range)).collect();
    return end_ranges.iter().map(|range| range.from).min().unwrap();
}

#[aoc(day5, part2, naive)]
pub fn part2_naive(almanac: &Almanac) -> u32 {
    let mut seed_index = 0;
    let mut min = u32::MAX;
    while seed_index < almanac.seeds.len() {
        let from = almanac.seeds[seed_index];
        let to = from + almanac.seeds[seed_index + 1];
        for seed_value in from..to {
            min = cmp::min(almanac.convert(seed_value), min);
        }
        seed_index += 2;
    }
    return min;
}

#[cfg(test)]
mod tests {
    use super::{Almanac, input_generator, part1, part2_sets, part2_naive, Range};

    fn get_example_almanac() -> Almanac {
        let example = "seeds: 79 14 55 13

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
56 93 4";
        return input_generator(example);
    }

    // part 1
    #[test]
    fn sample1() {
        assert_eq!(part1(&get_example_almanac()), 35);
    }

    // part 2
    #[test]
    fn sample2_sets() {
        assert_eq!(part2_sets(&get_example_almanac()), 46);
    }
    #[test]
    fn sample2_naive() {
        assert_eq!(part2_naive(&get_example_almanac()), 46);
    }

    // some tests for the helpers
    #[test]
    fn sample3a() {
        let almanac = &get_example_almanac();
        let ranges = vec![
            Range { from: 0, to: 100 },
            Range { from: 49, to: 52 },
            Range { from: 51, to: 110 },
            Range { from: 98, to: 99 },
        ];
        ranges.iter().for_each(|range| {
            println!("---\n{:?}", range);
            for i in range.from..range.to {
                let option = almanac.maps[0].find_next_rule_from(i);
                if option.is_none() {
                    println!("{} -> -", i);
                    continue;
                }
                let (rule_index, is_within) = option.unwrap();
                println!("{} -> rule: {} +{} ({})",
                         i,
                         almanac.maps[0].sources[rule_index],
                         almanac.maps[0].lengths[rule_index],
                         is_within,
                );
            }
            println!("Range {:?} in map {:?} will convert to {:?}", range, almanac.maps[0], almanac.maps[0].convert_range(*range));
        });
    }

    #[test]
    fn sample3b() {
        let ranges = vec![
            Range { from: 30, to: 50 },
            Range { from: 0, to: 10 },
            Range { from: 12, to: 20 },
            Range { from: 18, to: 22 },
            Range { from: 35, to: 45 },
        ];
        println!("Merging!");
        Range::merge(ranges).iter().for_each(|range| println!("Range {:?}", range));
    }
}
