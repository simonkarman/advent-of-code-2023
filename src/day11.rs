#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn distance(a: &Point, b: &Point) -> usize {
        fn diff(x: usize, y: usize) -> usize { if x < y { y - x } else { x - y } }
        return diff(a.x, b.x) + diff(a.y, b.y);
    }
}

fn solution(input: &str, expansion: usize) -> usize {
    let width = input.lines().next().unwrap().chars().count();
    let height = input.lines().count();
    let mut empty_cols = vec![true; width];
    let mut empty_rows = vec![true; height];
    let mut points: Vec<Point> = vec![];
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, char)| {
            match char {
                '#' => {
                    points.push(Point { x, y });
                    empty_cols[x] = false;
                    empty_rows[y] = false;
                }
                _ => {}
            }
        });
    });
    fn before(empty: &Vec<bool>) -> Vec<usize> {
        return empty.iter()
            .fold((0, vec![]), |(mut count, mut array), is_empty| {
                if *is_empty {
                    count += 1;
                }
                array.push(count);
                return (count, array);
            }).1;
    }
    let empty_cols_before = before(&empty_cols);
    let empty_rows_before = before(&empty_rows);
    let points: Vec<Point> = points.iter().map(|point| Point {
        x: point.x + (empty_cols_before[point.x] * (expansion - 1)),
        y: point.y + (empty_rows_before[point.y] * (expansion - 1)),
    }).collect();

    let mut sum = 0;
    for (i, a) in points.iter().enumerate() {
        for b in points.as_slice()[i + 1..].iter() {
            let distance = Point::distance(a, b);
            sum += distance;
        }
    }
    return sum;
}

#[aoc(day11, part1)]
pub fn part1(input: &str) -> usize {
    return solution(input, 2);
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> usize {
    return solution(input, 1_000_000);
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    // part 1
    #[test]
    fn sample1() {
        assert_eq!(part1("...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."), 374);
    }

    // part 2
    #[test]
    fn sample2() {
        assert_eq!(part2("...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."), 82000210);
    }

}
