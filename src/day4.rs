#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|line| {
            let from = line.find(':').unwrap() + 1;
            let mut splits = line[from..].split('|');
            let winning_numbers: Vec<i32> = splits.next().unwrap().split_whitespace().map(|number| number.parse().unwrap()).collect();
            let your_numbers: Vec<i32> = splits.next().unwrap().split_whitespace().map(|number| number.parse().unwrap()).collect();
            return winning_numbers.iter().filter(|winning_number| your_numbers.contains(winning_number)).count();
        }).collect()
}

#[aoc(day4, part1)]
pub fn part1(cards: &[usize]) -> i32 {
    return cards.iter().map(|win_count| {
        if *win_count == 0 {
            return 0;
        }
        return i32::pow(2, (*win_count - 1) as u32);
    }).sum();
}

#[aoc(day4, part2)]
pub fn part2(cards: &[usize]) -> i32 {
    let mut copies_per_card = vec![1; cards.iter().count()];
    for (index, win_count) in cards.iter().enumerate() {
        for i in 0..*win_count {
            copies_per_card[index + i + 1] += copies_per_card[index];
        }
    }
    return copies_per_card.iter().sum();
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    // part 1
    #[test]
    fn sample1() {
        assert_eq!(part1(input_generator("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53").iter().as_ref()), 8);
    }

    // part 2
    #[test]
    fn sample2() {
        assert_eq!(part2(input_generator("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11").iter().as_ref()), 30);
    }

}
