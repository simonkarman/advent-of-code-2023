#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    fn first_and_last_number_in_string(input: &str) -> (u32, u32) {
        let (mut first, mut last) = (11, 11);
        for char in input.chars() {
            if char.is_digit(10) {
                last = char.to_digit(10).unwrap();
                if first == 11 {
                    first = last;
                }
            }
        }
        return (first, last);
    }
    return input.lines().fold(0, |acc, line| {
        let (first, last) = first_and_last_number_in_string(line);
        acc + first * 10 + last
    });
}

#[aoc(day1, part2)]
pub fn part2(_input: &str) -> usize {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    // part 1
    #[test]
    fn sample1() {
        assert_eq!(part1("12"), 12);
        assert_eq!(part1("abc2defg4adf"), 24);
        assert_eq!(part1("h7h"), 77);
        assert_eq!(part1("1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet"), 142);

    }

    // part 2
    #[test]
    fn sample2() {
        assert_eq!(part2(""), 0);
        assert_eq!(part2(""), 0);
    }

}
