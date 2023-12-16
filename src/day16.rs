enum Tile {
    Empty,
    MirrorForward,
    MirrorBackward,
    SplitterHorizontal,
    SplitterVertical,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
struct Beam {
    depth: usize,
    x: i32,
    y: i32,
    direction: Direction,
}

#[aoc(day16, part1)]
pub fn part1(input: &str) -> usize {
    let width = input.lines().next().unwrap().chars().count();
    let height = input.lines().count();
    let mut tiles = vec![];
    input.lines().for_each(|row| {
        row.chars().for_each(|t| tiles.push(match t {
            '.' => Tile::Empty,
            '/' => Tile::MirrorForward,
            '\\' => Tile::MirrorBackward,
            '-' => Tile::SplitterHorizontal,
            '|' => Tile::SplitterVertical,
            _ => panic!("invalid tile found"),
        }));
    });
    assert_eq!(tiles.len(), width * height);
    let mut visited = vec![false; width * height * 4];
    let mut energized = vec![false; width * height];

    let to_diff_x = |direction: &Direction| match direction {
        Direction::East => 1,
        Direction::West => -1,
        _ => 0,
    };
    let to_diff_y = |direction: &Direction| match direction {
        Direction::South => 1,
        Direction::North => -1,
        _ => 0,
    };

    let mut beams = vec![Beam { depth: 0, x: -1, y: 0, direction: Direction::East }];
    while !beams.is_empty() {
        let last_beam = beams.len() - 1;
        let beam = beams.get(last_beam).unwrap();
        let next_depth = beam.depth + 1;

        let next_x = beam.x + to_diff_x(&beam.direction);
        let next_y = beam.y + to_diff_y(&beam.direction);
        if next_x < 0 || next_x >= (width as i32) || next_y < 0 || next_y >= (height as i32) {
            beams.pop();
            continue;
        }

        let tile_index = width * (next_y as usize) + (next_x as usize);
        energized[tile_index] = true;
        let visited_index = tile_index + (width * height * match beam.direction {
            Direction::North => 0,
            Direction::East => 1,
            Direction::South => 2,
            Direction::West => 3,
        });
        if visited[visited_index] {
            beams.pop();
            continue;
        }
        visited[visited_index] = true;

        let next_dir: Direction;
        let mut additional_dir: Option<Direction> = None;

        let next_tile = tiles.get(tile_index).unwrap();
        match next_tile {
            Tile::Empty => { next_dir = beam.direction; }
            Tile::MirrorForward => {
                match beam.direction {
                    Direction::North => { next_dir = Direction::East; }
                    Direction::East => { next_dir = Direction::North; }
                    Direction::South => { next_dir = Direction::West; }
                    Direction::West => { next_dir = Direction::South; }
                }
            }
            Tile::MirrorBackward => {
                match beam.direction {
                    Direction::North => { next_dir = Direction::West; }
                    Direction::East => { next_dir = Direction::South; }
                    Direction::South => { next_dir = Direction::East; }
                    Direction::West => { next_dir = Direction::North; }
                }
            }
            Tile::SplitterHorizontal => {
                match beam.direction {
                    Direction::North => {
                        next_dir = Direction::East;
                        additional_dir = Some(Direction::West);
                    }
                    Direction::South => {
                        next_dir = Direction::East;
                        additional_dir = Some(Direction::West);
                    }
                    _ => { next_dir = beam.direction; }
                }
            }
            Tile::SplitterVertical => {
                match beam.direction {
                    Direction::East => {
                        next_dir = Direction::North;
                        additional_dir = Some(Direction::South);
                    }
                    Direction::West => {
                        next_dir = Direction::North;
                        additional_dir = Some(Direction::South);
                    }
                    _ => { next_dir = beam.direction; }
                }
            }
        }
        beams[last_beam] = Beam {
            depth: next_depth,
            x: next_x,
            y: next_y,
            direction: next_dir,
        };
        if !additional_dir.is_none() {
            beams.push(Beam {
                depth: next_depth,
                x: next_x,
                y: next_y,
                direction: additional_dir.unwrap(),
            });
        }
    }
    energized.chunks(width).for_each(|line| println!("{}", line.iter().map(|v| if *v { '#' } else { '.' } ).collect::<String>()));
    return energized.iter().filter(|v| **v).count();
}

#[aoc(day16, part2)]
pub fn part2(_input: &str) -> usize {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn samples() {
        let example = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";
        assert_eq!(part1(example), 46); // too high: 7736
        assert_eq!(part2(example), 0);
    }
}
