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

#[aoc(day8, part2)]
pub fn part2(_input: &Map) -> i32 {
    return 0;
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
        assert_eq!(part2(&example()), 0);
    }

}
