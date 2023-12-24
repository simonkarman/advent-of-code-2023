struct Line {
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
}

impl Line {
    fn new(text: &str) -> Line {
        let splits = text.split(" @ ").collect::<Vec<&str>>();
        let get_xyz = |split_index: usize| -> (f64, f64, f64) {
            match splits.get(split_index).unwrap().split(", ").collect::<Vec<&str>>()[..] {
                [x, y, z] => (x.trim().parse().unwrap(), y.trim().parse().unwrap(), z.trim().parse().unwrap()),
                _ => panic!("cannot get xyz from '{}'", splits.get(split_index).unwrap()),
            }
        };
        let (x, y, z) = get_xyz(0);
        let (vx, vy, vz) = get_xyz(1);
        Line { x, y, z, vx, vy, vz }
    }

    fn find_xy_intersection(&self, other: &Line) -> Option<(f64, f64)> {
        // check parallel (skip collinear case)
        let s = self.vx * other.vy - self.vy * other.vx;
        if s == 0f64 {
            return None;
        }

        let diff = (other.x - self.x, other.y - self.y);
        let t = (diff.0 * other.vy - diff.1 * other.vx) / s;
        let u = (diff.0 * self.vy - diff.1 * self.vx) / s;

        // collision happened prior to either line
        if t < 0f64 || u < 0f64 {
            return None;
        }
        return Some((self.x + t * self.vx, self.y + t * self.vy));
    }
}

#[aoc(day24, part1)]
pub fn part1(input: &str) -> usize {
    let lines: Vec<Line> = input.lines().map(Line::new).collect();
    let boundary: (f64, f64) = if lines.len() == 5 {(7f64, 27f64)} else {(200000000000000f64, 400000000000000f64)};
    let mut intersections = 0;
    for i in 0..lines.len() {
        for j in (i + 1)..lines.len() {
            let intersection = lines[i].find_xy_intersection(&lines[j]);
            if let Some(point) = intersection  {
                if point.0 >= boundary.0 && point.0 <= boundary.1
                    && point.1 >= boundary.0 && point.1 <= boundary.1 {
                    intersections += 1;
                }
            }
        }
    }
    return intersections;
}

#[aoc(day24, part2)]
pub fn part2(_input: &str) -> usize {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn samples() {
        let example = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";
        assert_eq!(part1(example), 2);
        assert_eq!(part2(example), 0);
    }
}
