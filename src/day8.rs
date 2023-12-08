use std::collections::HashMap;

#[derive(Debug)]
pub struct Node {
    left: String,
    right: String,
}

#[derive(Debug)]
pub struct Map {
    instructions: String,
    nodes: HashMap<String, Node>,
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Map {
    let mut nodes = HashMap::new();
    for line in input.lines().skip(2) {
        let name: String = line.chars().take(3).collect();
        let left = line.chars().skip(7).take(3).collect();
        let right = line.chars().skip(12).take(3).collect();
        nodes.insert(name, Node {
            left,
            right,
        });
    }
    return Map {
        instructions: input.lines().next().unwrap().to_string(),
        nodes,
    }
}

#[aoc(day8, part1)]
pub fn part1(map: &Map) -> i32 {
    let mut steps = 0;
    let mut current = "AAA";
    while current != "ZZZ" {
        let direction = map.instructions.chars().nth(steps % map.instructions.chars().count()).unwrap();
        steps += 1;
        let current_node = map.nodes.get(current).unwrap();
        if direction == 'L' {
            current = current_node.left.as_str();
        } else {
            current = current_node.right.as_str();
        }
    }
    return steps as i32;
}

// returns the least common multiple of n numbers
pub fn lcm(nums: &[u64]) -> u64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}
fn gcd_of_two_numbers(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

#[aoc(day8, part2)]
pub fn part2(map: &Map) -> u64 {
    let map = map.to_owned();
    let mut all: Vec<String> = vec![];
    for name in map.nodes.keys() {
        if name.ends_with('A') {
            all.push(name.clone());
        }
    }

    let steps: Vec<u64> = all.iter().map(|start| {
        let mut current = start.to_string();
        let mut steps: usize = 0;
        while !current.ends_with('Z') {
            let direction = map.instructions.chars().nth(steps % map.instructions.chars().count()).unwrap();
            steps += 1;
            let curr_node = map.nodes.get(current.as_str()).unwrap();
            if direction == 'L' {
                current = curr_node.left.to_string();
            } else {
                current = curr_node.right.to_string();
            }
        }
        return steps as u64;
    }).collect();
    return lcm(&steps);
}

#[cfg(test)]
mod tests {
    use super::{input_generator, Map, part1, part2};

    fn example() -> Map {
        return input_generator("LLR\n\nAAA = (BBB, BBB)\nBBB = (AAA, ZZZ)\nZZZ = (ZZZ, ZZZ)");
    }

    // part 1
    #[test]
    fn sample1() {
        assert_eq!(part1(&example()), 6);
    }

    // part 2
    #[test]
    fn sample2() {
        let input = "LR\n\n11A = (11B, XXX)\n11B = (XXX, 11Z)\n11Z = (11B, XXX)\n22A = (22B, XXX)\n22B = (22C, 22C)\n22C = (22Z, 22Z)\n22Z = (22B, 22B)\nXXX = (XXX, XXX)";
        assert_eq!(part2(&input_generator(input)), 6);
    }

}
