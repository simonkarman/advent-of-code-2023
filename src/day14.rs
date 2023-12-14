#[derive(Debug, Copy, Clone)]
enum Tile {
    Empty,
    Fixed,
    Sliding,
}

#[aoc(day14, part1)]
pub fn part1(input: &str) -> usize {
    let width = input.lines().next().unwrap().chars().count();
    let height = input.lines().count();
    let mut field = vec![];
    input.lines().for_each(|line| line.chars().for_each(|char| field.push(match char {
        '.' => Tile::Empty,
        '#' => Tile::Fixed,
        'O' => Tile::Sliding,
        _ => panic!("unknown tile with type {}", char)
    })));
    let to_index = |x: usize, y: usize| -> Option<usize> {
        if x < 0 || x >= width || y < 0 || y >= height { return None }
        return Some(width * y + x);
    };
    let mut sum = 0;
    for x in 0..width {
        let mut begin_y = 0;
        let mut number_of_sliders = 0;
        let mut curr_sum = 0;
        for y in 0..(height + 1) {
            let index = to_index(x, y);
            let mut tile = Tile::Fixed;
            if !index.is_none() {
                tile = field[index.unwrap()];
            }
            match tile {
                Tile::Sliding => {
                    number_of_sliders += 1;
                    curr_sum += (height - begin_y) - (number_of_sliders - 1);

                },
                Tile::Fixed => {
                    sum += curr_sum;
                    number_of_sliders = 0;
                    curr_sum = 0;
                    begin_y = y + 1;
                }
                _ => {},
            }
        }
    }
    return sum;
}

#[aoc(day14, part2)]
pub fn part2(_input: &str) -> usize {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn samples() {
        let example = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        assert_eq!(part1(example), 136);
        assert_eq!(part2(example), 0);
    }
}
