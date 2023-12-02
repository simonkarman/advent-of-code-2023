use std::cmp::max;

type CubeSet = (/*red*/ i32, /*green*/ i32, /*blue*/ i32);
type Game = (/*id*/ i32, /*reveals*/ Vec<CubeSet>);

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|l| {
            let mut game_splits = l.trim().split(':');
            let game_id: i32 = game_splits.next().unwrap()[5..].parse().unwrap();
            let reveal_splits = game_splits.next().unwrap().split(';');
            let reveals: Vec<CubeSet> = reveal_splits.map(|reveal| {
                let mut red = 0;
                let mut green = 0;
                let mut blue = 0;
                for cube in reveal.split(',') {
                    let amount = cube.trim().split(' ').next().unwrap().parse().unwrap();
                    if cube.ends_with("red") {
                        red = amount;
                    } else if cube.ends_with("green") {
                        green = amount;
                    } else if cube.ends_with("blue") {
                        blue = amount;
                    }
                }
                (red, green, blue)
            }).collect();
            let game: Game = (game_id, reveals);
            return game;
        }).collect()
}

#[aoc(day2, part1)]
pub fn part1(games: &[Game]) -> i32 {
    return games.iter()
        .filter(|(_,reveals)| reveals.iter().all(|(red, green, blue)| *red <= 12 && *green <= 13 && *blue <= 14))
        .fold(0, |acc, &(id, _)| acc + id);
}

#[aoc(day2, part2)]
pub fn part2(games: &[Game]) -> i32 {
    fn to_minimum_set(game: &Game) -> CubeSet {
        let (_, reveals) = game;
        let (mut red, mut green, mut blue) = reveals[0];
        for i in 0..reveals.iter().count() {
            let (p_red, p_green, p_blue) = reveals.iter().nth(i).unwrap();
            red = max(red, *p_red);
            green = max(green, *p_green);
            blue = max(blue, *p_blue);
        }
        return (red, green, blue);
    }
    fn to_power(cube_set: CubeSet) -> i32 {
        let (red, green, blue) = cube_set;
        return red * green * blue;
    }
    return games.iter()
        .map(to_minimum_set)
        .map(to_power)
        .sum();
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    // part 1
    #[test]
    fn sample1() {
        assert_eq!(part1(input_generator("").iter().as_ref()), 0);
        assert_eq!(part1(input_generator("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green").iter().as_ref()), 8);

    }

    // part 2
    #[test]
    fn sample2() {
        assert_eq!(part2(input_generator("").iter().as_ref()), 0);
        assert_eq!(part2(input_generator("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green").iter().as_ref()), 0);
    }

}
