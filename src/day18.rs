use std::cmp;

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Copy, Clone)]
pub struct Instruction {
    direction: Direction,
    meters: usize,
}

fn get_dimensions(instructions: &Vec<Instruction>) -> (/*width*/ usize, /*height*/ usize, /*start_x*/ usize, /*start_y*/ usize) {
    let mut x = 0;
    let mut y = 0;
    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    for instruction in instructions {
        let (x_diff, y_diff) = get_diff(&instruction.direction);
        x += x_diff * (instruction.meters as i32);
        y += y_diff * (instruction.meters as i32);
        min_x = cmp::min(x, min_x);
        min_y = cmp::min(y, min_y);
        max_x = cmp::max(x, max_x);
        max_y = cmp::max(y, max_y);
    }
    return ((max_x - min_x + 1) as usize, (max_y - min_y + 1) as usize, (-min_x) as usize, (-min_y) as usize);
}

fn get_diff(direction: &Direction) -> (i32, i32) {
    let x_diff = match direction {
        Direction::Right => 1,
        Direction::Left => -1,
        _ => 0,
    };
    let y_diff = match direction {
        Direction::Down => 1,
        Direction::Up => -1,
        _ => 0,
    };
    return (x_diff, y_diff);
}

#[aoc(day18, part1)]
pub fn part1(input: &str) -> usize {
    let instructions = &input
        .lines()
        .map(|l| {
            let splits: Vec<&str> = l.split(' ').collect();
            Instruction {
                direction: match *splits.get(0).unwrap() {
                    "U" => Direction::Up,
                    "R" => Direction::Right,
                    "D" => Direction::Down,
                    "L" => Direction::Left,
                    _ => panic!("unknown direction {}", splits.get(0).unwrap()),
                },
                meters: splits.get(1).unwrap().parse().unwrap(),
            }
        }).collect();

    let (width, height, start_x, start_y) = get_dimensions(instructions);
    let mut holes = vec![false; width * height];
    let mut x = start_x;
    let mut y = start_y;
    let to_index = |x: usize, y: usize| y * width + x;
    holes[to_index(x, y)] = true;
    for instruction in instructions {
        let (x_diff, y_diff) = get_diff(&instruction.direction);
        for _ in 0..instruction.meters {
            x = (x as i32 + x_diff) as usize;
            y = (y as i32 + y_diff) as usize;
            holes[to_index(x, y)] = true;
        }
    }

    // Flood fill the inside
    let mut open: Vec<(usize, usize)> = vec![(start_x + 1, start_y + 1)];
    while let Some((x, y)) = open.pop() {
        let index = to_index(x, y);
        holes[index] = true;
        let mut try_visit = |x, y| {
            let location = (x, y);
            let index = to_index(x, y);
            if !holes[index] && !open.contains(&location) {
                open.push(location);
            }
        };
        if x > 0 { try_visit(x - 1, y) };
        if x < width - 1 { try_visit(x + 1, y) };
        if y > 0 { try_visit(x, y - 1) };
        if y < height - 1 { try_visit(x, y + 1) };
    }
    return holes.iter().filter(|h| **h).count();
}

#[aoc(day18, part2)]
pub fn part2(input: &str) -> f64 {
    let instructions: Vec<Instruction> = input.lines().map(|l| {
        let splits: Vec<&str> = l.split(' ').collect();
        let color = splits.get(2).unwrap();
        Instruction {
            direction: match color.chars().nth(color.len() - 2).unwrap() {
                '0' => Direction::Right,
                '1' => Direction::Down,
                '2' => Direction::Left,
                '3' => Direction::Up,
                _ => panic!("unknown direction {}", color.chars().nth(color.len() - 2).unwrap()),
            },
            meters: usize::from_str_radix(&color[2..(color.len() - 2)], 16).unwrap(),
        }
    }).collect();
    let mut x = 0;
    let mut y = 0;
    let mut points: Vec<(f64, f64)> = vec![(x as f64, y as f64)];
    instructions.iter().for_each(|instruction| {
        let (x_diff, y_diff) = get_diff(&instruction.direction);
        x += x_diff * (instruction.meters as i32);
        y += y_diff * (instruction.meters as i32);
        points.push((x as f64, y as f64));
    });
    assert_eq!(points[0], points[points.len() - 1]);

    // Calculate area of a polygon
    let area = 0.5f64 * (0..(points.len() - 1)).map(|i| {
        let (ax, ay) = points[i];
        let (bx, by) = points[i + 1];
        return ax * by - bx * ay;
    }).sum::<f64>();
    let circumference = 1f64 + (0..(points.len() - 1)).map(|i| {
        let (ax, ay) = points[i];
        let (bx, by) = points[i + 1];
        let x_diff = ax - bx;
        let y_diff = ay - by;
        return (if x_diff < 0f64 {-x_diff} else {x_diff}) + (if y_diff < 0f64 {-y_diff} else {y_diff});
    }).sum::<f64>() / 2f64;
    return area + circumference;
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn samples() {
        let example = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        assert_eq!(part1(example), 62);
        assert_eq!(part2(example), 952408144115f64);
    }
}
