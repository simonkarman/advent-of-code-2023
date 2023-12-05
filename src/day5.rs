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
}

#[derive(Debug)]
pub struct Almanac {
    seeds: Vec<u32>,
    maps: Vec<Map>,
}

impl Almanac {
    fn convert(&self, value: u32) -> u32 {
        return self.maps.iter().fold(value, |acc, map| map.convert(acc));
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

#[aoc(day5, part2)]
pub fn part2(_almanac: &Almanac) -> u32 {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    // part 1
    #[test]
    fn sample1() {
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
        assert_eq!(part1(&input_generator(example)), 35);
    }

    // part 2
    #[test]
    fn sample2() {
        assert_eq!(part2(&input_generator("")), 0);
    }

}
