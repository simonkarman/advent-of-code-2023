fn solution(input: &str, steps: usize) -> usize {
    let original_width = input.lines().next().unwrap().chars().count();
    let original_height = input.lines().count();
    let mut start_x = 0;
    let mut start_y = 0;
    let mut tiles = vec![];
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, char)| {
            tiles.push(match char {
                '.' => true,
                '#' => false,
                'S' => {
                    start_x = x;
                    start_y = y;
                    true
                }
                _ => panic!("unknown tile {}", char)
            })
        });
    });

    let widths = 2 * (steps / original_width) + 1;
    let heights = 2 * (steps / original_height) + 1;
    println!("required {} and {} of {}x{} to fit {} steps", widths, heights, original_width, original_height, steps);
    let start_x = start_x + ((widths / 2) * original_width);
    let start_y = start_y + ((heights / 2) * original_height);
    let width = widths * original_width;
    let height = heights * original_height;
    let start_index = start_y * width + start_x;
    println!("{}x{} start at {},{}", width, height, start_x, start_y);

    let from_index = |index: usize| -> (usize, usize) {
        let x = index % width;
        let y = (index - x) / width;
        (x, y)
    };

    let parity = 2;
    let mut visited_per_depth: Vec<(bool, usize)> = vec![(false, usize::MAX); width * height * parity];
    let to_depth_index = |index: usize, depth: usize| {
        return (width * height * (depth % parity)) + index;
    };
    let mut open: Vec<(usize, usize)> = vec![(to_depth_index(start_index, 0), 0)];
    while open.len() > 0 {
        // Always pick the next with the smallest depth
        open.sort_by_key(|(index, depth)| steps - *depth);
        let (index, depth) = open.pop().unwrap();

        // Mark this index at this depth as visited
        let depth_index = to_depth_index(index, depth);
        visited_per_depth[depth_index] = (true, depth);

        // When visiting a neighbor
        let mut visit_neighbor = |x: usize, y: usize| {
            let target_index = y * width + x;
            let tile_index = (y % original_height) * original_width + (x % original_width);
            let target_depth = depth + 1;
            let target_depth_index = to_depth_index(target_index, target_depth);
            let previous_visit = visited_per_depth[target_depth_index];
            if target_depth <= steps // should be reachable within the given steps
                && tiles[tile_index] // should be a garden and not a rock
                && (!previous_visit.0 || previous_visit.1 > target_depth) // should not have already been visited at this depth UNLESS it was visited at a higher depth
                && !open.contains(&(target_index, target_depth)) // should not already be on our list of future visits
            {
                open.push((target_index, target_depth));
            }
        };

        // Try and visit all neighbors
        let (x, y) = from_index(index);
        if x > 0 { visit_neighbor(x - 1, y) };
        if x < width - 1 { visit_neighbor(x + 1, y) };
        if y > 0 { visit_neighbor(x, y - 1) };
        if y < height - 1 { visit_neighbor(x, y + 1) };
        // if x > 0 { visit_neighbor(x - 1, y) } else { visit_neighbor(width - 1, y); };
        // if x < width - 1 { visit_neighbor(x + 1, y) } else { visit_neighbor(0, y); };
        // if y > 0 { visit_neighbor(x, y - 1) } else { visit_neighbor(x, height - 1); };
        // if y < height - 1 { visit_neighbor(x, y + 1) } else { visit_neighbor(x, 0); };
    }

    println!("\nafter {} steps:", steps);
    let mut sum = 0;
    for y in 0..height {
        for x in 0..width {
            let index = y * width + x;
            let v = visited_per_depth[to_depth_index(index, steps)];
            if v.0 {
                print!("{:X}", v.1 % 16);
                sum += 1;
            } else {
                if tiles[index] {
                    print!(".");
                } else {
                    print!("#");
                }
            }
        }
        println!();
    }
    return sum;
}

#[aoc(day21, part1)]
pub fn part1(input: &str) -> usize {
    return solution(input, 64);
}

#[aoc(day21, part2)]
pub fn part2(input: &str) -> usize {
    // too low: 15427
    return solution(input, 26501365);
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, solution};

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
        assert_eq!(part2(example), 0);
    }
}
