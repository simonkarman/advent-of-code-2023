use std::collections::VecDeque;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
struct Block {
    x_from: usize,
    y_from: usize,

    x_to: usize,
    y_to: usize,

    z_from: usize,
    height: usize,
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{},{}~{},{} ({} +{})",
            self.x_from, self.y_from,
            self.x_to, self.y_to,
            self.z_from, self.height,
        ).as_str())
    }
}

impl Block {
    fn new(line: &str) -> Block {
        let splits = line.split('~').collect::<Vec<&str>>();
        let get_xyz = |split_index: usize| -> (usize, usize, usize) {
            match splits.get(split_index).unwrap().split(',').collect::<Vec<&str>>()[..] {
                [x, y, z] => (x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap()),
                _ => panic!("cannot create block from '{}' coordinates", splits.get(split_index).unwrap()),
            }
        };
        let from = get_xyz(0);
        let to = get_xyz(1);
        Block {
            x_from: from.0, x_to: to.0,
            y_from: from.1, y_to: to.1,
            z_from: from.2, height: (to.2 - from.2) + 1,
        }
    }

    fn overlaps_xy(&self, other: &Block) -> bool {
        !(  self.x_to < other.x_from || self.x_from > other.x_to
         || self.y_to < other.y_from || self.y_from > other.y_to)
    }
}

#[aoc(day22, part1)]
pub fn part1(input: &str) -> usize {
    let mut blocks = input.lines().map(Block::new).collect::<Vec<Block>>();
    blocks.sort_by_key(|b| b.z_from);
    blocks.reverse();

    let mut block_configuration: Vec<(/*resting on*/ Vec<usize>, /*supporting*/ Vec<usize>)> = vec![];
    let mut fallen_blocks: VecDeque<(usize, (usize, Block))> = VecDeque::new();
    while let Some(block) = blocks.pop() {
        let height = block.height;
        let mut could_have_fallen_on = fallen_blocks.iter().rev()
            .filter(|(_, (_, b))| b.overlaps_xy(&block))
            .collect::<Vec<&(usize, (usize, Block))>>();
        could_have_fallen_on.sort_by_key(|v| v.0);
        let z_from = match could_have_fallen_on.last() {
            None => 0,
            Some((z, _)) => *z,
        };

        // add this block configuration
        let block_index = block_configuration.len();
        block_configuration.push((vec![], vec![]));

        // find all touching blocks aka block configuration
        fallen_blocks.iter().for_each(|(other_z_to, (other_block_index, other_block))| {
            if *other_z_to == z_from && other_block.overlaps_xy(&block) {
                // this block is resting on the other block
                block_configuration[block_index].0.push(*other_block_index);
                // so the other block is support this block
                block_configuration[*other_block_index].1.push(block_index);
            }
        });

        // add this block
        fallen_blocks.push_back((z_from + height, (block_index, block)));
    }
    let removable_blocks = block_configuration.iter().enumerate().map(|(block_index, (_, supporting))| {
        // if all blocks that this block is supporting
        supporting.iter().all(|supporting_block_index| {
            // also rest on another block than block_index
            let (resting_on, _) = block_configuration.get(*supporting_block_index).unwrap();
            resting_on.iter().any(|resting_block_index| *resting_block_index != block_index)
        })
    }).collect::<Vec<bool>>();

    // println!("-- debug information --");
    // block_configuration.iter().enumerate().for_each(|(block_index, configuration)| {
    //     let block = fallen_blocks.get(block_index).unwrap();
    //     println!("block {} (to z{} +{}) rests on {:?} and supports {:?} (removable={})", block_index, block.0, block.1.1.height, configuration.0, configuration.1, removable_blocks.get(block_index).unwrap());
    // });

    return removable_blocks.iter().filter(|v| **v).count();
    // too high: 456
}

#[aoc(day22, part2)]
pub fn part2(_input: &str) -> usize {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn samples() {
        let example1 = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
        let example2 = "1,0,1~1,2,1
0,2,5~2,2,5
0,2,4~2,2,4
0,0,2~2,0,3
1,0,6~1,2,6";
        let example3 = "1,0,1~1,2,1
0,0,2~2,0,4
0,2,4~2,2,4
0,2,5~2,2,5
1,0,6~1,2,6";
        assert_eq!(part1(example1), 5);
        assert_eq!(part1(example2), 3);
        assert_eq!(part1(example3), 2);
        assert_eq!(part2(example1), 0);
    }
}
