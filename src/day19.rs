use std::collections::HashMap;

#[derive(Debug)]
struct Rule {
    fragment: char,
    operator_smaller: bool,
    value: usize,
    target: String,
}

#[derive(Debug)]
struct Workflow {
    rules: Vec<Rule>,
    fallback_target: String,
}

fn solution(input: &str) -> (usize, u64) {
    let mut parts = vec![];
    let mut workflows = HashMap::new();
    input.lines().for_each(|line| {
        if line.is_empty() {
            return;
        }
        if line.starts_with('{') {
            // a part
            let values = line
                .replace(&['{', 'x', '=', 'm', 'a', 's', '}'][..], "")
                .split(',')
                .map(|v| v.parse().unwrap())
                .collect::<Vec<usize>>();
            let mut part = HashMap::new();
            part.insert('x', values.get(0).unwrap().clone());
            part.insert('m', values.get(1).unwrap().clone());
            part.insert('a', values.get(2).unwrap().clone());
            part.insert('s', values.get(3).unwrap().clone());
            parts.push(part)
        } else {
            // a workflow
            let splits: Vec<&str> = line.split('{').collect();
            let name = splits.get(0).unwrap();
            let second_split = splits.get(1).unwrap();
            let rule_strings: Vec<&str> = second_split[0..(second_split.len() - 1)].split(',').collect();
            let mut rules = vec![];
            rule_strings[0..(rule_strings.len() - 1)].iter().for_each(|rule_string| {
                let splits: Vec<&str> = rule_string.split(':').collect();
                let expression = splits.get(0).unwrap();
                rules.push(Rule {
                    fragment: expression.chars().nth(0).unwrap(),
                    operator_smaller: expression.chars().nth(1).unwrap() == '<',
                    value: expression[2..].parse().unwrap(),
                    target: splits.get(1).unwrap().to_string(),
                })
            });
            workflows.insert(name.to_string(), Workflow {
                rules,
                fallback_target: rule_strings[rule_strings.len() - 1].to_string(),
            });
        }
    });

    // Part 1
    let next_target = |part: &HashMap<char, usize>, workflow: &str| {
        let workflow = workflows.get(workflow).unwrap();
        for rule_index in 0..workflow.rules.len() {
            let rule = workflow.rules.get(rule_index).unwrap();
            let fragment_value = *part.get(&rule.fragment).unwrap();
            if (rule.operator_smaller && fragment_value < rule.value)
                || (!rule.operator_smaller && fragment_value > rule.value) {
                return rule.target.as_str();
            }
        }
        return workflow.fallback_target.as_str();
    };
    let mut sum = 0;
    for part in parts {
        let mut target = "in";
        while target != "A" && target != "R" {
            target = next_target(&part, target);
        }
        if target == "A" {
            sum += part.values().sum::<usize>();
        }
    }

    // Part 2
    fn process(workflows: &HashMap<String, Workflow>, workflow: &str, ranges: &HashMap<char, (usize, usize)>) -> u64 {
        if workflow == "R" {
            return 0;
        }
        if workflow == "A" {
            return ranges.values().map(|(f, t)| (*t - *f + 1) as u64).product();
        }
        let workflow = workflows.get(workflow).unwrap();
        let mut remaining_ranges = ranges.clone();
        let mut sum = 0;
        for rule_index in 0..workflow.rules.len() {
            let rule = workflow.rules.get(rule_index).unwrap();
            let range_start = remaining_ranges.get(&rule.fragment).unwrap().0;
            let range_end = remaining_ranges.get(&rule.fragment).unwrap().1;
            if rule.operator_smaller {
                if rule.value <= range_start {
                    continue;
                } else if rule.value <= range_end {
                    let mut next_ranges = remaining_ranges.clone();
                    *next_ranges.get_mut(&rule.fragment).unwrap() = (range_start, rule.value - 1);
                    *remaining_ranges.get_mut(&rule.fragment).unwrap() = (rule.value, range_end);
                    sum += process(workflows, rule.target.as_str(), &next_ranges);
                } else {
                    return sum + process(workflows, rule.target.as_str(), &remaining_ranges);
                }
            } else {
                if rule.value >= range_end {
                    continue;
                } else if rule.value >= range_start {
                    let mut next_ranges = remaining_ranges.clone();
                    *remaining_ranges.get_mut(&rule.fragment).unwrap() = (range_start, rule.value);
                    *next_ranges.get_mut(&rule.fragment).unwrap() = (rule.value + 1, range_end);
                    sum += process(workflows, rule.target.as_str(), &next_ranges);
                } else {
                    return sum + process(workflows, rule.target.as_str(), &remaining_ranges);
                }
            }
        }
        return sum + process(workflows, workflow.fallback_target.as_str(), &remaining_ranges);
    }
    let mut ranges: HashMap<char, (usize, usize)> = HashMap::new();
    ranges.insert('x', (1, 4000));
    ranges.insert('m', (1, 4000));
    ranges.insert('a', (1, 4000));
    ranges.insert('s', (1, 4000));
    let options = process(&workflows, "in", &ranges);

    return (sum, options);
}

#[aoc(day19, part1)]
pub fn part1(input: &str) -> usize {
    return solution(input).0;
}

#[aoc(day19, part2)]
pub fn part2(input: &str) -> u64 {
    return solution(input).1;
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn samples() {
        let example = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
        assert_eq!(part1(example), 19114);
        assert_eq!(part2(example), 167409079868000);
    }
}
