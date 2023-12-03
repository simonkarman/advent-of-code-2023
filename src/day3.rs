use std::collections::HashMap;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> i32 {
    let width = input.lines().next().unwrap().chars().count() as i32;
    let height = input.lines().count() as i32;
    let char_at = |x: i32, y: i32| -> char {
        if x < 0 || x >= width || y < 0 || y >= height {
            return '.';
        }
        return input.chars().nth((y * (width + 1) + x) as usize).unwrap();
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
pub fn part2(input: &str) -> i32 {
    let width = input.lines().next().unwrap().chars().count() as i32;
    let height = input.lines().count() as i32;
    let get_index = |x: i32, y: i32| -> i32 { y * (width + 1) + x };
    let char_at = |x: i32, y: i32| -> char {
        if x < 0 || x >= width || y < 0 || y >= height {
            return '.';
        }
        return input.chars().nth(get_index(x, y) as usize).unwrap();
    };
    let is_gear = |c: char| -> bool {
        return c == '*';
    };
    let mut gears: HashMap<i32, (i32, i32)> = HashMap::from([]);
    for y in 0..height {
        let mut x = 0;
        while x < width {
            let mut adjacent_gears: Vec<i32> = vec![];
            let mut digits = "".to_owned();

            // Find gears on left
            if is_gear(char_at(x - 1, y - 1)) {
                adjacent_gears.push(get_index(x - 1, y - 1))
            }
            if is_gear(char_at(x - 1, y)) {
                adjacent_gears.push(get_index(x - 1, y))
            }
            if is_gear(char_at(x - 1, y + 1)) {
                adjacent_gears.push(get_index(x - 1, y + 1))
            }

            while char_at(x, y).is_digit(10) {
                digits += char_at(x, y).to_string().as_str();
                // find gears middle
                if is_gear(char_at(x, y - 1)) {
                    adjacent_gears.push(get_index(x, y - 1))
                }
                if is_gear(char_at(x, y + 1)) {
                    adjacent_gears.push(get_index(x, y + 1))
                }
                x += 1;
            }

            // find gears on right
            if is_gear(char_at(x, y - 1)) {
                adjacent_gears.push(get_index(x, y - 1))
            }
            if is_gear(char_at(x, y)) {
                adjacent_gears.push(get_index(x, y))
            }
            if is_gear(char_at(x, y + 1)) {
                adjacent_gears.push(get_index(x, y + 1))
            }

            let number = digits.parse().unwrap_or(0);
            if number > 0 {
                adjacent_gears.iter().for_each(|gear| {
                    let (count, product) = gears.get(gear).unwrap_or(&(0, 1));
                    gears.insert(*gear, (count + 1, product * number));
                });
            }
            x += 1;
        }
    }
    return gears.iter().fold(0, |acc, (_gear_id, (count, product))| {
        if *count == 2 {
            return acc + *product;
        }
        return acc;
    });
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
        assert_eq!(part2("467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..\n"), 467835);
    }

}
