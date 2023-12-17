use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, Copy, Clone)]
struct Target {
    x: usize,
    y: usize,
    cost: usize,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Path {
    x: usize,
    y: usize,
    cost: usize,

    // false for hor plane, true for ver plane
    plane: bool,
}
impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.plane.cmp(&other.plane))
            .then_with(|| self.x.cmp(&other.x))
            .then_with(|| self.y.cmp(&other.y))
    }
}
impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn solution(input: &str, range_from: usize, range_to: usize) -> usize {

    let width = input.lines().next().unwrap().chars().count();
    let height = input.lines().count();

    // Find all costs
    let mut costs: Vec<usize> = vec![];
    input.lines().for_each(|line| {
        line.chars().for_each(|c| {
            costs.push(c.to_string().parse().unwrap());
        });
    });
    assert_eq!(costs.len(), width * height);

    // Dual planes of nodes. Each node has a list of targets. A target has a location (target.x and
    //  target.y) on the other plane which can be moved to for the provided cost (target.cost).
    let mut nodes: Vec<Vec<Target>> = vec![vec![]; width * height * 2];
    let to_index = |x: usize, y: usize| y * width + x;
    let to_hor_index = |x: usize, y: usize| to_index(x, y);
    let to_ver_index = |x: usize, y: usize| to_hor_index(x, y) + (width * height);
    for y in 0..height {
        for x in 0..width {
            let mut hor_targets = vec![];
            let mut ver_targets = vec![];
            for delta in range_from..(range_to + 1) {
                // hor plane
                if x >= delta {
                    hor_targets.push(Target {
                        x: x - delta,
                        y,
                        cost: (1..(delta + 1)).map(|x_delta| costs[to_index(x - x_delta, y)]).sum(),
                    });
                }
                if x + delta < width {
                    hor_targets.push(Target {
                        x: x + delta,
                        y,
                        cost: (1..(delta + 1)).map(|x_delta| costs[to_index(x + x_delta, y)]).sum(),
                    });
                }
                // ver plane
                if y >= delta {
                    ver_targets.push(Target {
                        x,
                        y: y - delta,
                        cost: (1..(delta + 1)).map(|y_delta| costs[to_index(x, y - y_delta)]).sum(),
                    });
                }
                if y + delta < height {
                    ver_targets.push(Target {
                        x,
                        y: y + delta,
                        cost: (1..(delta + 1)).map(|y_delta| costs[to_index(x, y + y_delta)]).sum(),
                    });
                }
            }
            nodes[to_hor_index(x, y)] = hor_targets;
            nodes[to_ver_index(x, y)] = ver_targets;
        }
    }

    // Path finding
    let mut visited_costs = vec![usize::MAX; width * height * 2];
    let mut paths = BinaryHeap::new();
    visited_costs[0] = 0;
    paths.push(Path{ cost: 0, x: 0, y: 0, plane: false });
    visited_costs[width * height] = 0;
    paths.push(Path{ cost: 0, x: 0, y: 0, plane: true });
    while let Some(Path { cost, x, y, plane }) = paths.pop() {
        // We have found the goal
        if x == width - 1 && y == height - 1 {
            return cost;
        }
        // A shorter path here was already found, skip this
        let index = if plane { to_ver_index(x, y) } else { to_hor_index(x, y) };
        if visited_costs[index] < cost {
            continue;
        }
        // Find all targets
        for target in &nodes[index] {
            let path = Path {
                x: target.x,
                y: target.y,
                plane: !plane,
                cost: cost + target.cost,
            };
            let target_index = if path.plane { to_ver_index(path.x, path.y) } else { to_hor_index(path.x, path.y) };
            // See if the target is worth visiting
            if path.cost < visited_costs[target_index] {
                paths.push(path);
                visited_costs[target_index] = path.cost;
            }
        }
    }
    panic!("no path found");
}

#[aoc(day17, part1)]
pub fn part1(input: &str) -> usize {
    return solution(input, 1, 3);
}

#[aoc(day17, part2)]
pub fn part2(input: &str) -> usize {
    return solution(input, 4, 10);
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn samples() {
        let example = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        assert_eq!(part1(example), 102);
        assert_eq!(part2(example), 0);
    }
}
