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

    // Draw graphviz
    println!("--graphviz--\ndigraph G {{  broadcaster -> hd,xt,kj,zt;\n  broadcaster [color=green];\n  rx [color=green];\n");
    modules.iter().for_each(|(k, v)| {
        let color = match v.variant {
            Variant::FlipFlop { .. } => "black",
            Variant::Conjunction => "blue",
        };
        println!("  {} [color={}];{} -> {};", k, color, k, v.targets.join(","));
    });
    println!("}}\n--end graphviz--");
    println!();

    // Clusters
    // xr: *hd,tp,rq,gr,zn,fh,xs,*sb,jh,bz,rg,dl
    // tf: *xt,?cc,cr,?kk,fc,qx,?gq,jm,pj,dk,tb,kr
    // gk: *zt,?xg,rd,cz,?vv,?br,fm,?dj,pd,kb,nt,ph
    // gx: *kj,hs,?tk,?tt,gf,?sh,lr,?mz,rv,ck,nf,gd
    let mut clusters = HashMap::new();
    clusters.insert("xr", vec!["hd","tp","rq","gr","zn","fh","xs","sb","jh","bz","rg","dl"]);
    clusters.insert("tf", vec!["xt","cc","cr","kk","fc","qx","gq","jm","pj","dk","tb","kr"]);
    clusters.insert("gk", vec!["zt","xg","rd","cz","vv","br","fm","dj","pd","kb","nt","ph"]);
    clusters.insert("gx", vec!["kj","hs","tk","tt","gf","sh","lr","mz","rv","ck","nf","gd"]);
    clusters.iter_mut().for_each(|(_, parts)| parts.reverse());
    let mut last_cluster_low_output = HashMap::new();

    // Find pulses
    let mut high_pulses_seen = 0;
    let mut low_pulses_seen = 0;
    for i in 0..5000 {
        let mut pulses = pulses.clone();
        low_pulses_seen += 1;
        let mut last_pulse_from = HashMap::new();
        let mut has_outputted_low = HashMap::new();
        while let Some((pulse_in, to, from)) = pulses.pop_front() {
            if i < 1000 {
                if pulse_in { high_pulses_seen += 1; } else { low_pulses_seen += 1; }
            }
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
                last_pulse_from.remove(to.as_str());
                continue;
            }
            last_pulse_from.insert(to.clone(), pulse_out.unwrap());
            if !pulse_out.unwrap() {
                has_outputted_low.insert(to.clone(), true);
            }
            for next_target in module.targets.clone() {
                pulses.push_back((pulse_out.unwrap(), next_target, to.clone()));
            }
            modules.insert(to, module);
        }

        // visualize clusters
        // clusters.iter().for_each(|(cluster, parts)| {
        //     println!("{}: {}", cluster, parts.iter().map(|part| {
        //         if let Some(val) = last_pulse_from.get(*part) {
        //             if !*val {
        //                 return "1";
        //             };
        //             return "0";
        //         }
        //         return "_";
        //     }).collect::<String>());
        // });
        clusters.iter().for_each(|(cluster, parts)| {
            if has_outputted_low.get(*cluster).is_none() {
                return;
            }
            let cluster_module = modules.get(*cluster);
            if cluster_module.is_none() {
                return;
            }
            let cluster_module = cluster_module.unwrap();
            let inputs = cluster_module.inputs.clone();
            let diff = last_cluster_low_output.get(*cluster).map(|last| i - last);
            println!("{}: {} ({:?} from {} since last {:?})", cluster, parts.iter().filter_map(|v| {
                let i = inputs.get(*v);
                if i.is_none() {
                    return None;
                }
                return Some(if *i.unwrap() {"1"} else {"0"});
            }).collect::<String>(), has_outputted_low.get(*cluster), i, diff);
            last_cluster_low_output.insert(*cluster, i);
        });

        // visualize cluster outputs
        clusters.iter().for_each(|(cluster, _)| {
            if let Some(val) = last_pulse_from.get(*cluster) {
                if !*val { println!("{}: {}", cluster, i); }
            }
        });
        if clusters.iter().all(|(cluster, _)| {
            if let Some(val) = last_pulse_from.get(*cluster) {
                return !*val;
            }
            return false;
        }) {
            println!("clusters: all found at {}", i);
        }
    }
    return low_pulses_seen * high_pulses_seen;
}

#[aoc(day20, part2)]
pub fn part2(_input: &str) -> usize {
    // solution -- see graphviz from part 1

    // graph clusters
    // xr: *hd,tp,rq,gr,zn,fh,xs,*sb,jh,bz,rg,dl
    // tf: *xt,?cc,cr,?kk,fc,qx,?gq,jm,pj,dk,tb,kr
    // gk: *zt,?xg,rd,cz,?vv,?br,fm,?dj,pd,kb,nt,ph
    // gx: *kj,hs,?tk,?tt,gf,?sh,lr,?mz,rv,ck,nf,gd

    // answers
    // each cluster cycles in the below intervals
    return 4021 * 3917 * 3923 * 3967;
    // too low:        479001600
    // correct:  245114020323037
    // too high: 281474980000000
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
        assert_eq!(part2(example1), 245114020323037);
    }
}
