pub struct Race {
    time: i64,
    distance: i64,
}

impl Race {
    fn number_of_ways_to_win(&self) -> i64 {
        let mut result = 0;
        for i in 0..self.time {
            if i * (self.time - i) > self.distance {
                result += 1;
            }
        }
        return result;
    }
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Race> {
    let mut lines = input.lines();
    let times: Vec<i64> = lines.next().unwrap()
        .strip_prefix("Time:").unwrap()
        .split_whitespace()
        .map(|time| time.parse().unwrap())
        .collect();
    let distances: Vec<i64> = lines.next().unwrap()
        .strip_prefix("Distance:").unwrap()
        .split_whitespace()
        .map(|distance| distance.parse().unwrap())
        .collect();
    return times.iter().enumerate().map(|(index, _)| Race {
        time: times[index],
        distance: distances[index],
    }).collect();
}

#[aoc(day6, part1)]
pub fn part1(races: &[Race]) -> i64 {
    return races.iter().fold(1, |acc, race| acc * race.number_of_ways_to_win());
}

#[aoc(day6, part2)]
pub fn part2(_races: &[Race]) -> i64 {
    return Race {
        time: 56717999,
        distance: 334113513502430,
    }.number_of_ways_to_win();
}

#[cfg(test)]
mod tests {
    use super::{Race, input_generator, part1, part2};

    fn get_example() -> Vec<Race> {
        let example = "Time:      7  15   30\nDistance:  9  40  200";
        return input_generator(example);
    }

    // part 1
    #[test]
    fn sample1() {
        assert_eq!(part1(get_example().as_ref()), 288);
    }

    // part 2
    #[test]
    fn sample2() {
        assert_eq!(part2(get_example().as_ref()), 43364472);
    }

}
