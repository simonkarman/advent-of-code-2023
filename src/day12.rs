fn can_place_at(row: &str, index: usize, size: usize, exhaustive: bool) -> bool {
    // length must be sufficient
    let length = row.chars().count();
    if index + size > length { return false; }

    // previous must be a possible boundary or begin
    if index > 0 {
        let previous = row.chars().nth(index - 1).unwrap();
        if previous == '#' { return false; }
    }

    // next must be a possible boundary or end
    let next = row.chars().nth(index + size).unwrap_or('.');
    if next == '#' { return false; }

    // all in between must be possible
    if !row.chars().skip(index).take(size).all(|v| v == '?' || v == '#') {
        return false;
    }

    // ensure that the string has been exhausted (all #'s have been used)
    if exhaustive {
        return row.chars().skip(index + size).all(|v| v != '#');
    }

    // finally, this placement is valid
    return true;
}

fn find_placements(row: &str, size: usize, exhaustive: bool) -> Vec<usize> {
    let mut found = vec![];
    for index in 0..row.chars().count() {
        if can_place_at(row, index, size, exhaustive) {
            found.push(index);
        }
        // Stop after we have started with a hashtag,
        //  as moving further would mean leaving a hashtag behind
        if row.chars().nth(index).unwrap() == '#' {
            break;
        }
    }
    return found;
}

fn find_arrangements(row: &str, group_sizes: &Vec<usize>) -> usize {
    assert_ne!(group_sizes.len(), 0);

    // If this is the last group placement, then return the number of possible exhaustive placements
    if group_sizes.len() == 1 {
        return find_placements(row, group_sizes[0], true).len();
    }

    // Otherwise recursively continue with the remaining row and groups from the different placements
    return find_placements(row, group_sizes[0], false).iter()
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
    return input.lines().map(|line| find_arrangements(
        line.split(' ').next().unwrap(),
        &line.split(' ').skip(1).next().unwrap().split(',').map(|v| v.parse().unwrap()).collect()
    )).sum();
}
#[aoc(day12, part2)]
pub fn part2(input: &str) -> usize {
    let mut length = input.lines().count();
    return input.lines().map(|line| {
        println!("{} left", length);
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
        find_arrangements(
            row.as_str(),
            &group_sizes,
        )
    }).sum();
}

#[cfg(test)]
mod tests {
    use super::{find_placements, find_arrangements, part1, part2};

    #[test]
    fn find_placements_tests() {
        assert_eq!(find_placements("???", 1, false), vec![0, 1, 2]);
        assert_eq!(find_placements("?#??", 1, false), vec![1]);
        assert_eq!(find_placements("???", 2, false), vec![0, 1]);
        assert_eq!(find_placements("??.???", 4, false), vec![]);
        assert_eq!(find_placements("???..?#?#??", 3, false), vec![0, 6]);
        assert_eq!(find_placements("???..???##??", 2, false), vec![0, 1, 5, 8]);
        assert_eq!(find_placements("???..?#?#???", 2, false), vec![0, 1, 5]);
        assert_eq!(find_placements("?#?", 1, false), vec![1]);
        assert_eq!(find_placements("?#", 1, false), vec![1]);
        assert_eq!(find_placements("#?", 1, false), vec![0]);
        assert_eq!(find_placements("?????#?????", 4, false), vec![0, 2, 3, 4, 5]);


        assert_eq!(find_placements("???", 1, true), vec![0, 1, 2]);
        assert_eq!(find_placements("#??", 1, true), vec![0]);
        assert_eq!(find_placements("?????#?????", 4, true), vec![2, 3, 4, 5]);
        assert_eq!(find_placements("?#???#?????", 4, true), vec![]);
    }

    #[test]
    fn find_arrangements_tests() {
        assert_eq!(find_arrangements("???.###", &vec![1, 1, 3]), 1);
        assert_eq!(find_arrangements("????", &vec![2, 1]), 1);
        assert_eq!(find_arrangements("?###????????", &vec![3, 2, 1]), 10);
    }

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

}