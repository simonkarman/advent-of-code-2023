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
        // lines are parallel (skip collinear case)
        let s = self.vx * other.vy - self.vy * other.vx;
        if s == 0f64 {
            return None;
        }

        let diff = (other.x - self.x, other.y - self.y);
        let t = (diff.0 * other.vy - diff.1 * other.vx) / s;
        let u = (diff.0 * self.vy - diff.1 * self.vx) / s;

        // intersection happened prior to either line
        if t < 0f64 || u < 0f64 {
            return None;
        }
        return Some((self.x + t * self.vx, self.y + t * self.vy));
    }

    fn find_xyz_intersection(&self, other: &Line) -> Option<(f64, f64, f64)> {
        // no intersection as lines are parallel (skip collinear case)
        let s = self.vx * other.vy - self.vy * other.vx;
        if s == 0f64 {
            return None;
        }

        // calculations
        let diff = (other.x - self.x, other.y - self.y);
        let t = (diff.0 * other.vy - diff.1 * other.vx) / s;
        let u = (diff.0 * self.vy - diff.1 * self.vx) / s;

        // no intersection as intersection would have happened prior to either line
        if t < 0f64 || u < 0f64 {
            return None;
        }

        // no intersection as intersection didn't happen in z coordinate
        let z_self = self.z + t * self.vz;
        let z_other = other.z + u * other.vz;
        if z_self != z_other {
            return None;
        }

        // return intersection point
        return Some((self.x + t * self.vx, self.y + t * self.vy, self.z + t * self.vz));
    }

    fn with_velocity(&self, dvx: f64, dvy: f64, dvz: f64) -> Line {
        Line {
            x: self.x,
            y: self.y,
            z: self.z,
            vx: self.vx + dvx,
            vy: self.vy + dvy,
            vz: self.vz + dvz,
        }
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
pub fn part2(input: &str) -> f64 {
    let lines: Vec<Line> = input.lines().map(Line::new).collect();
    for z in -499..500 {
        let z = z as f64;
        for y in -499..500 {
            let y = y as f64;
            for x in -499..500 {
                let x = x as f64;
                let line_a = lines[0].with_velocity(x, y, z);
                let line_b = lines[1].with_velocity(x, y, z);
                let line_c = lines[2].with_velocity(x, y, z);
                if let Some(intersection_ab) = line_a.find_xyz_intersection(&line_b) {
                    if let Some(intersection_ac) = line_a.find_xyz_intersection(&line_c) {
                        if intersection_ab == intersection_ac {
                            let (ix, iy, iz) = intersection_ab;
                            return ix + iy + iz;
                        }
                    }
                }
            }
        }
    }
    panic!("not found!");
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
        assert_eq!(part2(example), 47f64);
    }
}
