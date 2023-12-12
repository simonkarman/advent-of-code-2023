use std::collections::HashMap;
use std::time::SystemTime;

fn can_place_at(row: &[char], index: usize, size: usize, exhaustive: bool) -> bool {
    // length must be sufficient
    let length = row.len();
    if index + size > length { return false; }

    // previous must be a possible boundary or begin
    if index > 0 {
        let previous = row.get(index - 1).unwrap();
        if *previous == '#' { return false; }
    }

    // next must be a possible boundary or end
    let next = row.get(index + size).unwrap_or(&'.');
    if *next == '#' { return false; }

    // all in between must be possible
    if !row.iter().skip(index).take(size).all(|v| *v == '?' || *v == '#') {
        return false;
    }

    // ensure that the string has been exhausted (all #'s have been used)
    if exhaustive {
        return row.iter().skip(index + size).all(|v| *v != '#');
    }

    // finally, this placement is valid
    return true;
}

fn find_placements(cache: &mut HashMap<String, bool>, row: &[char], size: usize, exhaustive: bool) -> Vec<usize> {
    let mut found = vec![];
    for index in 0..row.len() {
        let key = format!("{:?}-{}-{}-{}", row, index, size, exhaustive);
        let can_place = cache.entry(key).or_insert_with(|| can_place_at(row, index, size, exhaustive));
        if *can_place {
            found.push(index);
        }
        // Stop after we have started with a hashtag,
        //  as moving further would mean leaving a hashtag behind
        if *row.get(index).unwrap() == '#' {
            break;
        }
    }
    return found;
}

fn find_arrangements(row: &[char], group_sizes: &Vec<usize>) -> usize {
    assert_ne!(group_sizes.len(), 0);
    let mut cache = HashMap::new();

    // If this is the last group placement, then return the number of possible exhaustive placements
    if group_sizes.len() == 1 {
        return find_placements(&mut cache, row, group_sizes[0], true).len();
    }

    // Otherwise recursively continue with the remaining row and groups from the different placements
    return find_placements(&mut cache, row, group_sizes[0], false).iter()
        .map(|placement| {
            let continue_from = *placement + group_sizes[0] + 1;
            if continue_from >= row.len() {
                return 0;
            }
            let remaining = row.len() - continue_from;
            let remaining_groups: Vec<usize> = group_sizes.iter().map(|v| *v).skip(1).collect();
            if remaining < (remaining_groups.len() - 1) + remaining_groups.iter().sum::<usize>() {
                return 0;
            }
            return find_arrangements(
                &row[continue_from..],
                &remaining_groups,
            )
        })
        .sum();
}

#[aoc(day12, part1)]
pub fn part1(input: &str) -> usize {
    return input.lines().map(|line| {
        let row: Vec<char> = line.split(' ').next().unwrap().chars().collect();
        find_arrangements(
            &row[..],
            &line.split(' ').skip(1).next().unwrap().split(',').map(|v| v.parse().unwrap()).collect()
        )
    }).sum();
}
#[aoc(day12, part2)]
pub fn part2(input: &str) -> usize {
    let start = SystemTime::now();
    let mut length = input.lines().count();
    return input.lines().map(|line| {
        println!("{:?}: {} left", start.elapsed().unwrap().as_secs(), length);
        length -= 1;
        let row_short = line.split(' ').next().unwrap();
        let mut row = String::from(row_short);
        let mut group_sizes: Vec<usize> = line.split(' ').skip(1).next().unwrap().split(',').map(|v| v.parse().unwrap()).collect();
        let num_of_groups = group_sizes.len();
        for _ in 0..4 {
            row.push('?');
            row.push_str(row_short);
            for j in 0..num_of_groups {
                group_sizes.push(group_sizes[j]);
            }
        }
        let row: Vec<char> = row.chars().collect();
        find_arrangements(
            &row[..],
            &group_sizes,
        )
    }).sum();
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn samples() {
        let example = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!(part1(example), 21);
        assert_eq!(part2(example), 525152);
    }

    #[test]
    fn slow() {
        assert_eq!(part2(".?????????????. 1,2,1,1,1"), 0);
    }

}
