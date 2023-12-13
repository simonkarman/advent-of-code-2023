enum Mirror {
    Hor(usize),
    Ver(usize),
}

struct Puzzle {
    raw: String,
    width: usize,
    height: usize,

    /* there are height number of rows and they have len width */
    rows: Vec<String>,

    /* there are width number of column and they have len height */
    columns: Vec<String>,
}

impl Puzzle {
    fn find_mirror(&self) -> Mirror {
        for column_index in 1..self.width {
            let mut offset = 0;
            loop {
                let a = &self.columns.get(column_index - offset - 1);
                let b = &self.columns.get(column_index + offset);
                if a.is_none() || b.is_none() {
                    return Mirror::Ver(column_index)
                }
                if a != b {
                    break;
                }
                offset += 1;
            }
        }
        for row_index in 1..self.height {
            let mut offset = 0;
            loop {
                let a = &self.rows.get(row_index - offset - 1);
                let b = &self.rows.get(row_index + offset);
                if a.is_none() || b.is_none() {
                    return Mirror::Hor(row_index)
                }
                if a != b {
                    break;
                }
                offset += 1;
            }
        }
        println!("{}", self.raw);
        panic!("no mirror found");
    }
}

#[aoc(day13, part1)]
pub fn part1(input: &str) -> usize {
    return input.split("\n\n")
        .map(|puzzle_input| {
            let width = puzzle_input.lines().next().unwrap().len();
            let height = puzzle_input.lines().count();
            let mut columns: Vec<Vec<char>> = vec![vec![]; width];
            puzzle_input.lines().for_each(|puzzle_line| {
                puzzle_line.chars().enumerate().for_each(|(column, char)| columns[column].push(char))
            });
            let puzzle = Puzzle {
                raw: String::from(puzzle_input),
                width,
                height,
                rows: puzzle_input.lines().map(|v| v.to_string()).collect(),
                columns: columns.iter().map(|v| v.iter().collect()).collect(),
            };
            puzzle.find_mirror()
        })
        .fold(0, |acc, v| acc + match v { Mirror::Hor(h) => h * 100, Mirror::Ver(v) => v });
}

#[aoc(day13, part2)]
pub fn part2(_input: &str) -> usize {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn samples() {
        let example = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        assert_eq!(part1(example), 405);
        assert_eq!(part2(example), 0);
    }
}
