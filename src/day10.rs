const NORTH: u8 = 0b0001;
const EAST: u8 = 0b0010;
const SOUTH: u8 = 0b0100;
const WEST: u8 = 0b1000;

#[aoc(day10, part1)]
pub fn part1(input: &str) -> i32 {
    // Find dimensions
    let width = input.lines().next().unwrap().chars().count();
    let height = input.lines().count();
    let index_at = |x: i32, y: i32| -> Option<usize> {
        if x < 0 || x >= (width as i32) || y < 0 || y >= (height as i32) {
            return None
        }
        return Some((y as usize) * width + (x as usize));
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
    let mut direction = SOUTH;
    let mut distance = 0;
    let mut x = start_x as i32;
    let mut y = start_y as i32;
    while distance == 0 || (x != (start_x as i32) || y != (start_y as i32)) {
        distance += 1;
        x += dir_x(direction);
        y += dir_y(direction);
        let next_index = index_at(x, y);
        if next_index.is_none() {
            panic!("cannot move outside the map");
        }
        direction = inverse(direction) ^ maze[next_index.unwrap()];
    }
    return (distance + 1) / 2;
}

#[aoc(day10, part2)]
pub fn part2(_input: &str) -> i32 {
    return 0;
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
        assert_eq!(part2(""), 0);
    }

}
