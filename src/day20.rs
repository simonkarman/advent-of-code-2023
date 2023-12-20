use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone)]
enum Variant {
    FlipFlop { status: bool },
    Conjunction,
}

#[derive(Debug, Clone)]
struct Module {
    inputs: HashMap<String, bool>,
    variant: Variant,
    targets: Vec<String>,
}

#[aoc(day20, part1)]
pub fn part1(input: &str) -> usize {
    let mut pulses = VecDeque::<(bool, String, String)>::new();
    let mut missing_targets_per_module = HashMap::<&str, Vec<&str>>::new();
    let mut modules = HashMap::<String, Module>::new();
    input.lines().for_each(|line| {
        let splits: Vec<&str> = line.split(" -> ").collect();
        let module = splits.get(0).unwrap();
        let targets: Vec<&str> = splits.get(1).unwrap().split(", ").collect();
        if *module == "broadcaster" {
            targets.iter().for_each(|t| pulses.push_back((false, t.to_string(), module.to_string())));
            return;
        }

        // find the name of this module
        let module_name = &module[1..];

        // insert all targets we missed before, as at that point this module didn't exist yet
        let mut inputs = HashMap::new();
        if let Some(missing_targets) = missing_targets_per_module.get(&module_name) {
            missing_targets.iter().for_each(|missing_target| {
                inputs.insert(missing_target.to_string(), false);
            });
        }
        missing_targets_per_module.remove(&module_name);

        // construct the module
        let variant = if module.starts_with('%') { Variant::FlipFlop { status: false } } else { Variant::Conjunction };
        modules.insert(module_name.to_string(), Module {
            inputs,
            variant,
            targets: targets.iter().map(|t| t.to_string()).collect(),
        });

        // register each target
        targets.iter().for_each(|target| {
            if modules.contains_key(&target.to_string()) {
                modules.get_mut(&target.to_string()).unwrap().inputs.insert(module_name.to_string(), false);
            } else {
                missing_targets_per_module.entry(target).or_default().push(&module_name);
            }
        })
    });
    let mut high_pulses_seen = 0;
    let mut low_pulses_seen = 0;
    for i in 0..1000 {
        let mut pulses = pulses.clone();
        low_pulses_seen += 1;
        while let Some((pulse_in, to, from)) = pulses.pop_front() {
            if pulse_in { high_pulses_seen += 1; } else { low_pulses_seen += 1; }
            let module = modules.get(to.as_str());
            if module.is_none() {
                continue;
            }
            let mut module = module.unwrap().clone();
            if from != "broadcaster" {
                *module.inputs.get_mut(from.as_str()).unwrap() = pulse_in;
            }
            let pulse_out = match module.variant {
                Variant::FlipFlop { status } => {
                    match pulse_in {
                        false => {
                            module.variant = Variant::FlipFlop { status: !status };
                            Some(!status)
                        }
                        true => { None }
                    }
                }
                Variant::Conjunction => { Some(!module.inputs.values().all(|pulse| *pulse)) }
            };
            if pulse_out.is_none() {
                continue;
            }
            for next_target in module.targets.clone() {
                pulses.push_back((pulse_out.unwrap(), next_target, to.clone()));
            }
            modules.insert(to, module);
        }
    }
    return low_pulses_seen * high_pulses_seen;
}

#[aoc(day20, part2)]
pub fn part2(_input: &str) -> usize {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn samples() {
        let example1 = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
        let example2 = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
        assert_eq!(part1(example1), 32000000);
        assert_eq!(part1(example2), 11687500);
        assert_eq!(part2(example1), 0);
    }
}
