const NORTH: u8 = 0b0001;
const EAST: u8 = 0b0010;
const SOUTH: u8 = 0b0100;
const WEST: u8 = 0b1000;

pub fn solution(input: &str) -> (i32, i32) {
    // Find dimensions
    let width = input.lines().next().unwrap().chars().count();
    let height = input.lines().count();
    let index_at = |x: i32, y: i32| -> Option<usize> {
        if x < 0 || x >= (width as i32) || y < 0 || y >= (height as i32) {
            return None
        }
        return Some((y as usize) * width + (x as usize));
    };
    let from_index = |index: usize| {
        let x = index % width;
        let y = (index - x) / width;
        return (x, y);
    };

    // Build maze
    let mut start_x = 0;
    let mut start_y = 0;
    let mut maze = vec![0u8; width * height];
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, char)| {
            let index = index_at(x as i32, y as i32).unwrap();
            maze[index] = match char {
                // S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
                'S' => { start_x = x; start_y = y; 0 },
                // | is a vertical pipe connecting north and south.
                '|' => NORTH | SOUTH,
                // - is a horizontal pipe connecting east and west.
                '-' => EAST | WEST,
                // L is a 90-degree bend connecting north and east.
                'L' => NORTH | EAST,
                // J is a 90-degree bend connecting north and west.
                'J' => NORTH | WEST,
                // 7 is a 90-degree bend connecting south and west.
                '7' => SOUTH | WEST,
                // F is a 90-degree bend connecting south and east.
                'F' => SOUTH | EAST,
                // . is ground; there is no pipe in this tile.
                _ => 0,
            };
        });
    });
    let start_x = start_x;
    let start_y = start_y;
    let maze = maze;

    // Find loop
    let dir_x = |direction: u8| -> i32 {
        if direction & EAST > 0 {
            return 1;
        }
        if direction & WEST > 0 {
            return -1;
        }
        return 0;
    };
    let dir_y = |direction: u8| -> i32 {
        if direction & SOUTH > 0 {
            return 1;
        }
        if direction & NORTH > 0 {
            return -1;
        }
        return 0;
    };
    let inverse = |direction: u8| -> u8 {
        match direction {
            NORTH => SOUTH,
            EAST => WEST,
            SOUTH => NORTH,
            WEST => EAST,
            _ => panic!("can only inverse direction with exactly one flag set")
        }
    };
    let side = |direction: u8| -> u8 {
        match direction {
            NORTH => EAST,
            EAST => SOUTH,
            SOUTH => WEST,
            WEST => NORTH,
            // NORTH => WEST,
            // EAST => NORTH,
            // SOUTH => EAST,
            // WEST => SOUTH,
            _ => panic!("can only find left_hand of direction with exactly one flag set")
        }
    };
    let mut direction = SOUTH;
    let mut distance = 0;
    let mut x = start_x as i32;
    let mut y = start_y as i32;
    let mut is_pipe = vec![false; width * height];
    let mut is_inside = vec![false; width * height];
    while distance == 0 || (x != (start_x as i32) || y != (start_y as i32)) {
        let curr_index = index_at(x, y).unwrap();
        is_pipe[curr_index] = true;
        if is_inside[curr_index] {
            is_inside[curr_index] = false;
        }
        let left_dir = side(direction);
        let left_index = index_at(x + dir_x(left_dir), y + dir_y(left_dir));
        if !left_index.is_none() && !is_pipe[left_index.unwrap()] && !is_inside[left_index.unwrap()] {
            is_inside[left_index.unwrap()] = true;
        }
        let left_index_2 = index_at(x + dir_x(left_dir) + dir_x(direction), y + dir_y(left_dir) + dir_y(direction));
        if !left_index_2.is_none() && !is_pipe[left_index_2.unwrap()] && !is_inside[left_index_2.unwrap()] {
            is_inside[left_index_2.unwrap()] = true;
        }

        // find next x, y, and direction
        distance += 1;
        let next_x = x + dir_x(direction);
        let next_y = y + dir_y(direction);
        let next_index = index_at(next_x, next_y);
        if next_index.is_none() {
            panic!("cannot move outside the map");
        }
        let next_direction = inverse(direction) ^ maze[next_index.unwrap()];

        x = next_x;
        y = next_y;
        direction = next_direction;
    }

    // Flood fill the inside
    let mut is_visited = vec![false; width * height];
    let mut visit_count = 0;
    let mut open: Vec<usize> = is_inside.iter().enumerate()
        .filter_map(|(index, value)| if *value {Some(index)} else {None}).collect();
    while !open.is_empty() {
        visit_count += 1;
        let index = open.pop().unwrap();
        is_visited[index] = true;
        let (x, y) = from_index(index);
        let x = x as i32;
        let y = y as i32;
        let mut try_visit = |x, y| {
            let index = index_at(x, y);
            if !index.is_none() && !is_pipe[index.unwrap()] && !is_visited[index.unwrap()] && !open.contains(&index.unwrap()) {
                open.push(index.unwrap());
            }
        };
        try_visit(x, y + 1);
        try_visit(x, y - 1);
        try_visit(x + 1, y);
        try_visit(x - 1, y);
    }
    return ((distance + 1) / 2, visit_count);
}

#[aoc(day10, part1)]
pub fn part1(input: &str) -> i32 {
    let (a, _) = solution(input);
    return a;
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> i32 {
    let (_, b) = solution(input);
    return b;
    // 660 too high
    // 471 apparently another answer
    // 1xx too low

}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    // part 1
    #[test]
    fn sample1() {
        assert_eq!(part1("..F7.
.FJ|.
SJ.L7
|F--J
LJ..."), 8);
    }

    // part 2
    #[test]
    fn sample2() {
        assert_eq!(part2("...........
.F-------S.
.|F-----7|.
.||..F--J|.
FJL-7|--J|.
|JL-||F-J|.
L7..|||..|.
.L--JL---J.
..........."), 14);
    }

}
