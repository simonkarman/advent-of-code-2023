type Example = (usize, usize);

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<Example> {
    input
        .lines()
        .map(|l| {
            let mut example = l.trim().split(',');
            let x = example.next().unwrap();
            (
                x.chars().count(),
                x.chars().count()
            )
        }).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[Example]) -> usize {
    return input.iter().map(|&(a, b)| a + b).sum();
}

#[aoc(day1, part2)]
pub fn part2(_input: &[Example]) -> usize {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    // part 1
    #[test]
    fn sample1() {
        assert_eq!(part1(""), 0);
        assert_eq!(part1(""), 0);
    }

    // part 2
    #[test]
    fn sample2() {
        assert_eq!(part2(""), 0);
        assert_eq!(part2(""), 0);
    }

}
