use std::cmp;
use std::collections::HashMap;

#[derive(PartialEq)]
enum Tile {
    Forest,
    Path,
    Slope { direction: u8 },
}

#[derive(Debug)]
struct Section {
    length: usize,
    next: Vec<usize>,
}

#[aoc(day23, part1)]
pub fn part1(input: &str) -> usize {
    let width = input.lines().next().unwrap().chars().count();
    let height = input.lines().count();
    let to_index = |x: usize, y: usize| y * width + x;
    let from_index = |index: usize | -> (usize, usize) {
        let x = index % width;
        let y = (index - x) / width;
        (x, y)
    };
    let mut field = vec![];
    let mut begin = 0;
    let mut goal = 0;
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, char)| field.push(match char {
            '#' => Tile::Forest,
            '.' => {
                if y == 0 {
                    begin = to_index(x, y);
                } else if y == height - 1 {
                    goal = to_index(x, y);
                }
                Tile::Path
            },
            '^' => Tile::Slope { direction: 0 },
            '>' => Tile::Slope { direction: 1 },
            'v' => Tile::Slope { direction: 2 },
            '<' => Tile::Slope { direction: 3 },
            _ => panic!("unknown"),
        }));
    });

    // Based on the index of a tile, return all neighboring indices in the field
    let get_neighbors = |index: usize| -> Vec<usize> {
        let mut neighbors = vec![];
        let (x, y) = from_index(index);
        if x > 0 { neighbors.push(to_index(x - 1, y)) };
        if x < width - 1 { neighbors.push(to_index(x + 1, y)) };
        if y > 0 { neighbors.push(to_index(x, y - 1)) };
        if y < height - 1 { neighbors.push(to_index(x, y + 1)) };
        neighbors
    };

    // Assert that there are no three or four way intersections and that slides never join a section
    //  midway through.
    let mut path_types: Vec<usize> = vec![0; width * height];
    for index in 0..(width * height) {
        match field.get(index).unwrap() {
            Tile::Path => {}
            _ => { continue; }
        }
        path_types[index] = get_neighbors(index).iter().map(|neighbor_index| match field.get(*neighbor_index).unwrap() {
            Tile::Path => 1,
            Tile::Slope { .. } => 10,
            _ => 0,
        }).sum();
    }
    // assert that no other path types are possible except for:
    assert_eq!(path_types.iter().all(|path_type| [
        0, // a forest or slope tile
        1, // a dead end (only possible for begin and goal tile)
        2, // a path
        11, // a slide to or from a section without a choice
        30, // three sections meet, with 1 or 2 incoming and the opposite number of outgoing slides
        40, // four sections meet, with 1, 2, or 3 incoming and the opposite number of outgoing slides
    ].contains(path_type)), true);

    // Find where a path ends and what the length of that path is
    let find_path = |start_index: usize| -> (usize, usize) {
        let start_path_type = path_types[start_index];

        // If we start at a crossing, then the path also ends here
        if start_path_type == 30 || start_path_type == 40 {
            return (start_index, 1);
        }

        // We can only start at a dead end if this is the begin or goal
        if start_path_type == 1 && start_index != begin && start_index != goal {
            panic!("an unexpected dead end was found at {}", start_index);
        }

        // We can never start in the middle of a path
        if start_path_type == 2 {
            panic!("a section started middle of a path at {}", start_index);
        }

        // Traverse the path
        let mut index = start_index;
        let mut previous_index = start_index;
        let mut path_length = 1;
        while previous_index == index || path_types[index] == 2 {
            path_length += 1;
            let next_index = *get_neighbors(index).iter().filter(|n| **n != previous_index && field[**n] == Tile::Path).nth(0).unwrap();
            previous_index = index;
            index = next_index;
        }
        (index, path_length)
    };

    // Given an index, find all neighbors that are a slide away from this tile and for these: return the index of the tile after the slide
    let find_outward = |index: usize| -> Vec<usize> {
        let mut neighbors = vec![];
        let (x, y) = from_index(index);
        if x > 1 {
            if let Tile::Slope { direction } = field[to_index(x - 1, y)] { if direction == 3 { neighbors.push(to_index(x - 2, y)); } }
        };
        if x < width - 2 {
            if let Tile::Slope { direction } = field[to_index(x + 1, y)] { if direction == 1 { neighbors.push(to_index(x + 2, y)); } }
        };
        if y > 1 {
            if let Tile::Slope { direction } = field[to_index(x, y - 1)] { if direction == 0 { neighbors.push(to_index(x, y - 2)); } }
        };
        if y < height - 2 {
            if let Tile::Slope { direction } = field[to_index(x, y + 1)] { if direction == 2 { neighbors.push(to_index(x, y + 2)); } }
        };
        neighbors
    };

    // Find each section
    let mut sections: HashMap<usize, Section> = HashMap::new();
    let mut open: Vec<usize> = vec![begin];
    while let Some(start_index) = open.pop() {
        let (end_index, length) = find_path(start_index);
        let next: Vec<usize> = find_outward(end_index);
        next.iter().for_each(|i| { if !open.contains(i) && !sections.contains_key(i) { open.push(*i) } });
        sections.insert(start_index, Section {
            length,
            next,
        });
    }

    // Visualize the sections using graphviz
    println!("--graphviz sections--");
    println!("digraph G {{\n  begin -> {}", begin);
    sections.iter().for_each(|(k, v)| {
        if v.next.is_empty() {
            print!("  {} -> goal;", k);
        } else {
            print!("  {} -> {};", k, v.next.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(","));
        }
        println!("  {} [label=\"length {}\"];", k, v.length);
    });
    println!("}}");
    println!("--end graphviz sections--");

    // Find the longest path
    let mut open: Vec<(/*section*/usize, /*path_length*/usize)> = vec![];
    let mut length_to_section: HashMap<usize, usize> = HashMap::new();
    open.push((begin, 0));
    let mut longest_route = 0;
    while let Some((section_start, length)) = open.pop() {
        let section = sections.get(&section_start).unwrap();

        // if this section was already reached, through a route that was longer, then abort this route
        if let Some(other_length) = length_to_section.get(&section_start) {
            if *other_length > length {
                continue;
            }
        }

        // otherwise, mark this as the longest route so far
        length_to_section.insert(section_start, length);

        // then if there are no next routes then set it to the longest route if it is
        if section.next.is_empty() {
            longest_route = cmp::max(longest_route, length + section.length);
        } else {
            // for each next route follow it
            section.next.iter().for_each(|n| {
                let n_length = length + section.length + 1;
                open.push((*n, n_length));
            });
        }
    }
    return longest_route - 1;
}

#[aoc(day23, part2)]
pub fn part2(_input: &str) -> usize {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn samples() {
        let example = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
        assert_eq!(part1(example), 94);
        assert_eq!(part2(example), 0);
    }
}
