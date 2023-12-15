use std::collections::HashMap;

fn hash(value: &str) -> usize {
    let mut curr = 0;
    for char in value.chars() {
        let ascii = char as u8;
        curr += ascii as usize;
        curr *= 17;
        curr %= 256;
    }
    return curr;
}

#[aoc(day15, part1)]
pub fn part1(input: &str) -> usize {
    return input.split(',').map(|v| hash(v)).sum();
}

#[aoc(day15, part2)]
pub fn part2(input: &str) -> usize {
    let mut boxes = HashMap::new();
    input.split(',').enumerate().for_each(|(i, v)| {
        let is_minus_operation = v.chars().last().unwrap() == '-';
        let mut label_len = v.chars().count() - 1;
        if !is_minus_operation { label_len -= 1; }

        let lens_name: String = v.chars().take(label_len).collect();
        let box_id = hash(lens_name.as_str());

        let _box = boxes.entry(box_id).or_insert(HashMap::new());
        if is_minus_operation {
            _box.remove(&lens_name);
        } else {
            let lens_value: usize = v.chars().last().unwrap().to_string().parse().unwrap();
            let lens = _box.entry(lens_name).or_insert_with(|| (i, lens_value));
            lens.1 = lens_value;
        }
    });
    return boxes.iter().map(|(box_id, lenses)| {
        let mut lens_array: Vec<(usize, usize)> = lenses.iter().map(|(_, (order, lens_value))| {
            return (*order, (box_id + 1) * lens_value);
        }).collect();
        lens_array.sort_by_key(|v| v.0);
        lens_array.iter().enumerate().map(|(i, (_, v))| (i + 1) * v).sum::<usize>()
    }).sum();
}

#[cfg(test)]
mod tests {
    use super::{hash, part1, part2};

    #[test]
    fn samples() {
        assert_eq!(hash("HASH"), 52);
        let example = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(part1(example), 1320);
        assert_eq!(part2(example), 145);
    }
}
