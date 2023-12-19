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

fn solution(input: &str) -> usize {
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

    let mut sum = 0;
    for part in parts {
        let mut target = "in";
        'outer: while target != "A" && target != "R" {
            print!("{} -> ", target);
            let workflow = workflows.get(target).unwrap();
            for rule_index in 0..workflow.rules.len() {
                let rule = workflow.rules.get(rule_index).unwrap();
                let fragment_value = *part.get(&rule.fragment).unwrap();
                if (rule.operator_smaller && fragment_value < rule.value)
                    || (!rule.operator_smaller && fragment_value > rule.value) {
                    target = rule.target.as_str();
                    continue 'outer;
                }
            }
            target = workflow.fallback_target.as_str();
        }
        print!("{}", target);
        if target == "A" {
            sum += part.values().sum::<usize>();
        }
        println!();
    }
    return sum;
}

#[aoc(day19, part1)]
pub fn part1(input: &str) -> usize {
    return solution(input);
}

#[aoc(day19, part2)]
pub fn part2(_input: &str) -> usize {
    return 0;
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
        assert_eq!(part2(example), 0);
    }
}
