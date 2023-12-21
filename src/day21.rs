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
    let mut visited: Vec<bool> = vec![false; width * height * parity];
    let to_visited_index = |index: usize, depth: usize| {
        return (width * height * (depth % parity)) + index;
    };
    let mut open: Vec<(usize, usize)> = vec![(start_index, 0)];
    while let Some((index, depth)) = open.pop() {
        // Mark this index at this depth as visited
        let visited_index = to_visited_index(index, depth);
        visited[visited_index] = true;

        // Stop if we're now at the max depth
        if depth >= steps {
            continue;
        }

        // When visiting a neighbor
        let mut try_visit = |x: usize, y: usize| {
            let target_index = y * width + x;
            let target_depth = depth + 1;
            let target_visited_index = to_visited_index(target_index, target_depth);
            if tiles[target_index] && !visited[target_visited_index] && !open.contains(&(target_index, target_depth)) {
                open.push((target_index, target_depth));
            }
        };

        // Try and visit all neighors
        let (x, y) = from_index(index);
        if x > 0 { try_visit(x - 1, y) };
        if x < width - 1 { try_visit(x + 1, y) };
        if y > 0 { try_visit(x, y - 1) };
        if y < height - 1 { try_visit(x, y + 1) };
    }

    for y in 0..height {
        for x in 0..width {
            let index = y * width + x;
            if visited[index] {
                print!("O")
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
    return visited[0..(width*height)].iter().filter(|v| **v).count();
}

#[aoc(day21, part1)]
pub fn part1(input: &str) -> usize {
    return solution(input, 64);
}

#[aoc(day21, part2)]
pub fn part2(_input: &str) -> usize {
    return 0;
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
        assert_eq!(solution(example, 6), 16);
        assert_eq!(part2(example), 0);
    }
}
