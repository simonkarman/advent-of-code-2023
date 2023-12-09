pub struct History {
    data: Vec<i32>,
}

impl History {
    pub fn new(input: &str) -> History {
        History {
            data: input.split(' ').map(|v| v.parse().unwrap()).collect(),
        }
    }

    pub fn predict(&self) -> i32 {
        let derivatives: Vec<i32> = vec![0; self.data.len() - 1].iter().enumerate()
            .map(|(index, _)| self.data[index + 1] - self.data[index]).collect();
        let last =  *self.data.get(self.data.len() - 1).unwrap();
        if derivatives.iter().all(|v| *v == 0) {
            return last;
        }
        return (History { data: derivatives }).predict() + last;
    }
}

#[aoc(day9, part1)]
pub fn part1(lines: &str) -> i32 {
    return lines.lines().map(History::new).map(|history| history.predict()).sum();
}

#[aoc(day9, part2)]
pub fn part2(_input: &str) -> i32 {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    // part 1
    #[test]
    fn sample1() {
        assert_eq!(part1("0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45"), 114);
    }

    // part 2
    #[test]
    fn sample2() {
        assert_eq!(part2(""), 0);
    }

}
