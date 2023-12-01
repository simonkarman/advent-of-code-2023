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
pub fn part2(input: &str) -> u32 {
    fn first_and_last_number_in_string(input: &str) -> (u32, u32) {
        let text_digits: &[&str] = &vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
        let (mut first, mut last) = (11, 11);
        for (i, char) in input.chars().enumerate() {
            let mut is_digit = false;
            if char.is_digit(10) {
                last = char.to_digit(10).unwrap();
                is_digit = true;
            }
            for (j, text_digit) in text_digits.iter().enumerate() {
                if input[i..].starts_with(text_digit) {
                    last = (j + 1) as u32;
                    is_digit = true;
                }
            }
            if is_digit && first == 11 {
                first = last;
            }
        }
        return (first, last);
    }
    return input.lines().fold(0, |acc, line| {
        let (first, last) = first_and_last_number_in_string(line);
        acc + first * 10 + last
    });
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
        assert_eq!(part2("one5"), 15);
        assert_eq!(part2("oone53oneda"), 11);
        assert_eq!(part2("two1nine"), 29);
        assert_eq!(part2("xtwone3four"), 24);
        assert_eq!(part2("zoneight234"), 14);
        assert_eq!(part2("two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen"), 281);
    }

}
