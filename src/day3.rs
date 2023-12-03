#[aoc(day3, part1)]
pub fn part1(_input: &str) -> i32 {
    let width = _input.lines().next().unwrap().chars().count() as i32;
    let height = _input.lines().count() as i32;
    let char_at = |x: i32, y: i32| -> char {
        if x < 0 || x >= width || y < 0 || y >= height {
            return '.';
        }
        return _input.chars().nth((y * (width + 1) + x) as usize).unwrap();
    };
    let is_symbol = |c: char| -> bool {
        return c != '.' && !c.is_digit(10);
    };
    let mut sum = 0;
    for y in 0..height {
        let mut x = 0;
        while x < width {
            let mut has_adjacent_symbol = false;
            let mut digits = "".to_owned();
            has_adjacent_symbol |= is_symbol(char_at(x - 1, y - 1)) || is_symbol(char_at(x - 1, y)) || is_symbol(char_at(x - 1, y + 1));
            while char_at(x, y).is_digit(10) {
                digits += char_at(x, y).to_string().as_str();
                has_adjacent_symbol |= is_symbol(char_at(x, y - 1)) || is_symbol(char_at(x, y + 1));
                x += 1;
            }
            has_adjacent_symbol |= is_symbol(char_at(x, y - 1)) || is_symbol(char_at(x, y)) || is_symbol(char_at(x, y + 1));
            if has_adjacent_symbol {
                sum += digits.parse().unwrap_or(0);
            }
            x += 1;
        }
    }
    return sum;
}

#[aoc(day3, part2)]
pub fn part2(_input: &str) -> i32 {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    // part 1
    #[test]
    fn sample1() {
        assert_eq!(part1("467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..\n"), 4361);
    }

    // part 2
    #[test]
    fn sample2() {
        assert_eq!(part2(""), 0);
    }

}
