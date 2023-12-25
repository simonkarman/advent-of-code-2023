use std::collections::HashMap;

#[aoc(day25, part1)]
pub fn part1(input: &str) -> usize {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    input.lines().for_each(|line| {
        let splits = line.split(": ").collect::<Vec<&str>>();
        let from = splits[0];
        let tos = splits[1].split(" ").collect::<Vec<&str>>();
        tos.iter().for_each(|to| {
            graph.entry(from.to_string()).or_default().push(to.to_string());
            graph.entry(to.to_string()).or_default().push(from.to_string());
        });
    });
    let mut remove_connection = |from: String, to: String| {
        let neighbors = graph.entry(from).or_default();
        if let Some(index) = neighbors.iter().position(|v| *v == to) {
            neighbors.swap_remove(index);
        }
    };

    // from visualization you can clearly see you need to remove:
    // rcn -> xkf
    remove_connection(String::from("rcn"), String::from("xkf"));
    remove_connection(String::from("xkf"), String::from("rcn"));
    // cms -> thk
    remove_connection(String::from("cms"), String::from("thk"));
    remove_connection(String::from("thk"), String::from("cms"));
    // dht -> xmv
    remove_connection(String::from("dht"), String::from("xmv"));
    remove_connection(String::from("xmv"), String::from("dht"));

    let mut visited = vec![];
    let mut open = vec![];
    open.push(graph.keys().nth(0).unwrap());
    while let Some(current) = open.pop() {
        visited.push(current);
        graph.get(current).unwrap().iter().for_each(|neighbor| {
            if !visited.contains(&neighbor) && !open.contains(&neighbor) {
                open.push(neighbor);
            }
        })
    }
    return visited.len() * (graph.len() - visited.len());
}

#[aoc(day25, part2)]
pub fn part2(_input: &str) -> usize {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn samples() {
        let example = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";
        assert_eq!(part1(example), 20);
        assert_eq!(part2(example), 0);
    }
}
