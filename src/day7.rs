use std::cmp::Ordering;

#[derive(Debug, Eq)]
pub struct Hand {
    cards: Vec<usize>,
    hand_type: usize,
    bid: usize,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let order = self.hand_type.cmp(&other.hand_type);
        if order.is_eq() {
            return self.cards.cmp(&other.cards);
        }
        order
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.cards.eq(&other.cards)
    }
}

pub fn get_cards(input: &str, _joker: Option<char>) -> Vec<Hand> {
    let cards = "23456789TJQKA";
    let joker = _joker.map(|char| cards.find(char).unwrap() + 1);
    println!("Joker at {:?}", joker);
    let mut hands: Vec<Hand> = input.lines().map(|line| {
        // Bind cards and bid
        let cards: Vec<usize> = line[0..5].chars().map(|char| cards.find(char).unwrap() + 1).collect();
        let bid = line[6..].parse().unwrap();

        // Find hand type
        let mut ordered = cards.clone();
        ordered.sort();
        let mut last_index = 0;
        let mut last_seen = ordered[0];
        let mut has_jokers = !joker.is_none() && last_seen == joker.unwrap();
        let mut same_joker_index = 0;
        let mut same = vec![0];
        for i in 0..ordered.len() {
            if last_seen == ordered[i] {
                same[last_index] = same[last_index] + 1;
            } else {
                same.push(1);
                last_index += 1;
                last_seen = ordered[i];
                if !joker.is_none() && last_seen == joker.unwrap() {
                    has_jokers = true;
                    same_joker_index = last_index;
                }
            }
        }
        let mut number_of_jokers = 0;
        if has_jokers && same.len() > 1 {
            number_of_jokers = same[same_joker_index];
            println!("{} {:?}[{}] {}", line, same, same_joker_index, number_of_jokers);
            same = same.iter().enumerate().filter_map(|(index, value)| {
                if index == same_joker_index {
                    return None;
                }
                return Some(*value);
            }).collect();
        }
        same.sort();
        if number_of_jokers > 0 {
            same[last_index - 1] += number_of_jokers;
        }
        let hand_type = match same[..] {
            [5] => 6,
            [1, 4] => 5,
            [2, 3] => 4,
            [1, 1, 3] => 3,
            [1, 2, 2] => 2,
            [1, 1, 1, 2] => 1,
            [1, 1, 1, 1, 1] => 0,
            _ => { println!("{}", line); panic!("hand_type not found") },
        };

        // Create hand with bid
        return Hand {
            cards: cards.iter().map(|card| {
                if !joker.is_none() && *card == joker.unwrap() { return 0; }
                return *card;
            }).collect(),
            hand_type,
            bid,
        }
    }).collect();
    hands.sort();
    return hands;
}

#[aoc(day7, part1)]
pub fn part1(input: &str) -> usize {
    return get_cards(input, None).iter().enumerate().fold(0, |acc, (rank, hand)| acc + (rank + 1) * hand.bid);
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> usize {
    return get_cards(input, Some('J')).iter().enumerate().fold(0, |acc, (rank, hand)| acc + (rank + 1) * hand.bid);
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    // part 1
    #[test]
    fn sample1() {
        assert_eq!(part1("32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483"), 6440);
    }

    // part 2
    #[test]
    fn sample2() {
        assert_eq!(part2("JJJJJ 5"), 5);
        assert_eq!(part2("32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483"), 5905);
    }

}
