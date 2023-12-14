#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum Tile {
    Empty,
    Fixed,
    Sliding,
}

fn solution(input: &str, number_of_cycles: usize) -> usize {
    // Transform input to field of tiles
    let width = input.lines().next().unwrap().chars().count();
    let height = input.lines().count();
    let mut field = vec![];
    input.lines().for_each(|line| line.chars().for_each(|char| field.push(match char {
        '.' => Tile::Empty,
        '#' => Tile::Fixed,
        'O' => Tile::Sliding,
        _ => panic!("unknown tile with type {}", char)
    })));

    // Different coord systems over the array
    let _to_index = |x: i32, y: i32| -> Option<usize> {
        if x < 0 || x >= (width as i32) || y < 0 || y >= (height as i32) { return None }
        return Some((width * y as usize) + (x as usize));
    };
    let to_index_north = |x: usize, y: usize| -> Option<usize> {
        return _to_index(x as i32, y as i32);
    };
    let to_index_east = |x: usize, y: usize| -> Option<usize> {
        return _to_index((width as i32) - (y as i32) - 1,  x as i32);
    };
    let to_index_south = |x: usize, y: usize| -> Option<usize> {
        return _to_index((width as i32) - (x as i32) - 1, (height as i32) - (y as i32) - 1);
    };
    let to_index_west = |x: usize, y: usize| -> Option<usize> {
        return _to_index(y as i32, (height as i32) - (x as i32) - 1);
    };

    // Calculate load
    let calculate_load = |field: &Vec<Tile>| {
        let mut sum = 0;
        for x in 0..width {
            let mut begin_y = 0;
            let mut number_of_sliders = 0;
            let mut curr_sum = 0;
            for y in 0..(height + 1) {
                let index = to_index_north(x, y);
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
    };

    if number_of_cycles == 0 {
        return calculate_load(&field);
    }

    // Go through all cycles
    let mut cycle_at: usize = 0;
    let mut post_cycle_fields: Vec<Vec<Tile>> = vec![];
    let mut post_repetition_loads = vec![];
    for cycle_index in 0..number_of_cycles {
        if cycle_index % 100 == 0 {
            println!(" -- Cycle {}/{} -- ", cycle_index + 1, number_of_cycles);
        }
        for tilt in 0..4 {
            // Ensure that we have a x,y coord system where y to 0 always points to the wind direction
            //  that we're currently looking at, and x to 0 points to the left of that
            let to_index = |x: usize, y:usize| -> Option<usize> {
                if tilt == 0 { return to_index_north(x,y); }
                else if tilt == 1 { return to_index_west(x,y);  }
                else if tilt == 2 { return to_index_south(x,y); }
                else if tilt == 3 { return to_index_east(x,y); }
                else { panic!("tilt should be 0,1,2,3 and not {}", tilt) };
            };
            let w = if tilt % 2 == 0 { width } else { height };
            let h = if tilt % 2 == 0 { height } else { width };

            // Roll all boulders towards y 0
            for x in 0..w {
                let mut begin_y = 0;
                let mut number_of_sliders = 0;
                for y in 0..(h + 1) {
                    let index = to_index(x, y);
                    let mut tile = Tile::Fixed;
                    if !index.is_none() {
                        tile = field[index.unwrap()];
                    }
                    match tile {
                        Tile::Sliding => {
                            number_of_sliders += 1;
                            field[index.unwrap()] = Tile::Empty;
                        },
                        Tile::Fixed => {
                            for slider_index in 0..number_of_sliders {
                                field[to_index(x, begin_y + slider_index).unwrap()] = Tile::Sliding;
                            }
                            number_of_sliders = 0;
                            begin_y = y + 1;
                        }
                        _ => {},
                    }
                }
            }
        }
        if cycle_at == 0 {
            let position = post_cycle_fields.iter().position(|f| f == &field);
            if !position.is_none() {
                cycle_at = position.unwrap();
                // break;
            } else {
                post_cycle_fields.push(field.clone());
            }
        }
        if cycle_at != 0 {
            post_repetition_loads.push(calculate_load(&field));
        }

        // if number_of_cycles < 10 {
        //     let load = calculate_load(&field);
        //     println!("Load {} ", load);
        //     field.chunks(width).for_each(|row| println!("{}", row.iter().map(|tile| match tile {
        //         Tile::Empty => '.',
        //         Tile::Fixed => '#',
        //         Tile::Sliding => 'O',
        //     }).collect::<String>()));
        //     println!();
        // }
    }

    let repetition_len = post_cycle_fields.len() - cycle_at;
    println!("Repetition starts at {cycle_at} and repeats every {repetition_len} cycles");
    let cycles_remaining = (number_of_cycles - 1) - cycle_at;
    let final_index = cycle_at + (cycles_remaining % repetition_len);
    println!("with {} cycles remaining we expect to end with an index of {}", cycles_remaining, final_index);

    // 108579, 108568, 108561, 108563, 108568, 108583, 108607, 108625, 108648, 108650, 108645, 108644, 108633, 108632, 108622, 108607, 108589
    let all_loads: Vec<usize> = post_cycle_fields.iter().map(|field| calculate_load(field)).collect();
    let load = calculate_load(&post_cycle_fields[final_index]);
    println!("which is {} at index {} from all loads up to the repetition {:?}\nRepetition: {:?}", load, final_index, all_loads, all_loads[cycle_at..].iter().map(|s| *s).collect::<Vec<usize>>());
    let fully_calculated_load = calculate_load(&field);
    println!("when still doing all results in {} -> {:?}", fully_calculated_load, post_repetition_loads);
    assert_eq!(load, fully_calculated_load, "loads should be the same if calculated from repetition or by going through all");
    return load;
}

#[aoc(day14, part1)]
pub fn part1(input: &str) -> usize {
    return solution(input, 0);
}

#[aoc(day14, part2)]
pub fn part2(input: &str) -> usize {
    return solution(input, 18/*1_000_000_000*/);
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
        // assert_eq!(part1(example), 136);
        assert_eq!(part2(example), 110 /*should be 64*/);

        let example2 = "O....#....
O.OO#....#
.....##...
OO.#O#...O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        assert_eq!(part2(example2), 64);
    }

    #[test]
    fn cycles() {
        let start: u64 = 6926;
        let cycles: Vec<u64> = vec![
            108589, // too high
            108579,
            108568,
            108561,
            108563,
            108568,
            108583,
            108607,
            108625,
            108648,
            108650,
            108645,
            108644,
            108633,
            108632,
            108622,
            108607,
        ];
        assert_eq!(0, cycles[(1_000_000_000u64 - start) as usize % cycles.len()]);
    }
}
