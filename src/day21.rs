use std::collections::HashMap;

fn solution(input: &str, steps: usize) -> usize {
    let width = input.lines().next().unwrap().chars().count();
    let height = input.lines().count();
    let from_index = |index: usize| -> (usize, usize) {
        let x = index % width;
        let y = (index - x) / width;
        (x, y)
    };
    let mut start_index = 0;
    let mut tiles = vec![];
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, char)| {
            tiles.push(match char {
                '.' => true,
                '#' => false,
                'S' => {
                    start_index = y * width + x;
                    true
                }
                _ => panic!("unknown tile {}", char)
            })
        });
    });

    let parity = 2;
    let mut visited_per_depth_per_plane: Vec<HashMap<i64, usize>> = vec![HashMap::new(); width * height * parity];
    let to_depth_index = |index: usize, depth: usize| {
        return (width * height * (depth % parity)) + index;
    };
    let to_plane_index = |plane_x: i32, plane_y: i32| -> i64 {
        return (plane_x as i64) * 1_000_000i64 + (plane_y as i64);
    };
    let mut open: Vec<(usize, usize, i32, i32)> = vec![(to_depth_index(start_index, 0), 0, 0, 0)];
    let mut visited_planes = HashMap::new();
    let mut max_depth_seen = 0;
    while open.len() > 0 {
        // Always pick the next with the smallest depth
        open.sort_by_key(|(_, depth, _, _)| steps - *depth);
        let (index, depth, plane_x, plane_y) = open.pop().unwrap();

        // Mark this index at this depth as visited
        let plane_index = to_plane_index(plane_x, plane_y);
        if !visited_planes.contains_key(&plane_index) {
            // println!("visiting plane {},{} index={}", plane_x, plane_y, plane_index);
            visited_planes.insert(plane_index, true);
        }
        if depth > max_depth_seen {
            max_depth_seen = depth;
            println!("depth {} reached", max_depth_seen);
        }
        let depth_index = to_depth_index(index, depth);
        let visited_per_plane = &mut visited_per_depth_per_plane[depth_index];
        let visited = visited_per_plane.entry(plane_index).or_default();
        *visited = depth;

        // When visiting a neighbor
        let mut visit_neighbor = |x: usize, y: usize, plane_x: i32, plane_y: i32| {
            let plane_index = to_plane_index(plane_x, plane_y);
            let target_index = y * width + x;
            let target_depth = depth + 1;
            let target_depth_index = to_depth_index(target_index, target_depth);
            let previous_visit_per_plane = &visited_per_depth_per_plane[target_depth_index];
            let previous_visit = previous_visit_per_plane.contains_key(&plane_index);
            if target_depth <= steps // should be reachable within the given steps
                && tiles[target_index] // should be a garden and not a rock
                && !previous_visit // should not have already been visited at this depth
                && !open.contains(&(target_index, target_depth, plane_x, plane_y)) // should not already be on our list of future visits
            {
                open.push((target_index, target_depth, plane_x, plane_y));
            }
        };

        // Try and visit all neighbors
        let (x, y) = from_index(index);
        if x > 0 { visit_neighbor(x - 1, y, plane_x, plane_y) } else { visit_neighbor(width - 1, y, plane_x - 1, plane_y); };
        if x < width - 1 { visit_neighbor(x + 1, y, plane_x, plane_y) } else { visit_neighbor(0, y, plane_x + 1, plane_y); };
        if y > 0 { visit_neighbor(x, y - 1, plane_x, plane_y) } else { visit_neighbor(x, height - 1, plane_x, plane_y - 1); };
        if y < height - 1 { visit_neighbor(x, y + 1, plane_x, plane_y) } else { visit_neighbor(x, 0, plane_x, plane_y + 1); };
    }

    println!("\nafter {} steps:", steps);
    let mut sums = vec![];
    let mut total_sum = 0;
    for plane_y in -4..5 {
        for plane_x in -4..5 {
            let print_plane = plane_x == 0 && (plane_y == 0 || plane_y == 1);
            if print_plane {
                println!("\nplane {},{}:", plane_x, plane_y);
            }
            let mut sum = 0;
            total_sum = 0;
            for y in 0..height {
                for x in 0..width {
                    let index = y * width + x;
                    let v = &visited_per_depth_per_plane[to_depth_index(index, steps)];
                    total_sum += v.len();
                    let plane_index = &to_plane_index(plane_x, plane_y);
                        if v.contains_key(plane_index) {
                            sum += 1;
                            // let depth = v.get(plane_index).unwrap();
                            // print!("{:X}", depth % 16);
                            if print_plane {
                                print!("O");
                            }
                        } else if print_plane {
                            if tiles[index] {
                                print!(".");
                            } else {
                                print!("#");
                            }
                        }
                }
                if print_plane {
                    println!();
                }
            }
            sums.push(sum);
        }
    }
    sums.chunks(9).for_each(|c| {
        println!("{:?}", c);
    });
    return total_sum;
}

#[aoc(day21, part1)]
pub fn part1(input: &str) -> usize {
    return solution(input, 64);
}

#[aoc(day21, part2)]
pub fn part2(input: &str) -> i64 {
    // Since solution is repeatable in the provided puzzle input, we can find three points and
    //  extrapolate the answer from those using a quadratic formula
    let width = 131;
    let steps = 26501365;

    // First let's find our three base points
    let remainder = steps % width;
    let one = solution(input, remainder) as i64;
    let two = solution(input, remainder + width) as i64;
    let three = solution(input, remainder + 2 * width) as i64;

    // Then, find the quadratic formula
    let a = (one - 2 * two + three) / 2;
    let b = (-3 * one + 4 * two - three) / 2;
    let c = one;
    println!("Formula(x) = {a}x^2 + {b}x + {c}");

    // Finally, print the result for x
    let x = (steps / width) as i64;
    let result = a * x * x + b * x + c;
    println!("Formula({x}) = {result}");
    return result;
}

#[cfg(test)]
mod tests {
    use super::{solution};

    #[test]
    fn samples() {
        let example = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
        assert_eq!(solution(example, 1), 2);
        assert_eq!(solution(example, 2), 4);
        assert_eq!(solution(example, 6), 16);
        assert_eq!(solution(example, 500), 167004);
    }
}
