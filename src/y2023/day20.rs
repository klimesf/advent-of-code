use std::collections::{HashMap, VecDeque};
use std::fs;
use crate::utils::toolbox::lcm_64;

pub(crate) fn day20() {
    println!("{}", part_a(fs::read_to_string("input/2023/day20/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2023/day20/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    let mut modules: HashMap<&str, Type> = HashMap::new();
    let mut destinations: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut flip_flop_states: HashMap<&str, bool> = HashMap::new();
    let mut conjunction_states: HashMap<&str, HashMap<&str, bool>> = HashMap::new();
    input.lines().for_each(|line| {
        let (name, destinations_str) = line.split_once(" -> ").unwrap();
        let tp = match name.chars().nth(0).unwrap() {
            '%' => Type::FlipFlop,
            '&' => Type::Conjunction,
            'b' => Type::Broadcaster,
            c => panic!("unknown type {}", c)
        };

        let name_fr = &name[1..name.len()];
        modules.insert(name_fr, tp);

        let dests: Vec<&str> = destinations_str.split(", ").collect();
        destinations.insert(name_fr, dests);
    });

    // Pre-populate conjunction states
    destinations.iter().for_each(|(from, dests)| {
        dests.iter().for_each(|dest| {
            if let Some(tp) = modules.get(dest) {
                if *tp == Type::Conjunction {
                    conjunction_states.entry(dest).or_insert(HashMap::new()).insert(from, false);
                }
            }
        })
    });

    let mut ans = (0, 0); // High, low
    for _ in 0..1000 {
        let mut stack = VecDeque::new();
        stack.push_back((false, "roadcaster", "button"));
        while let Some((high, key, from)) = stack.pop_front() {
            if high { ans = (ans.0 + 1, ans.1) } else { ans = (ans.0, ans.1 + 1) }

            if !modules.contains_key(key) { continue }
            let send_high: bool;
            match *modules.get(key).unwrap() {
                Type::FlipFlop => {
                    if high { continue; }
                    let state = *flip_flop_states.entry(key).or_insert(false);
                    flip_flop_states.insert(key, !state);
                    send_high = !state;
                }
                Type::Conjunction => {
                    let state = conjunction_states.entry(key).or_insert(HashMap::new());
                    state.insert(from, high);
                    if state.values().all(|v| *v) { send_high = false } else { send_high = true }
                }
                Type::Broadcaster => { send_high = high }
            }
            for dest in destinations.get(key).unwrap() {
                stack.push_back((send_high, dest, key));
            }
        }
    }
    return ans.0 * ans.1;
}

fn part_b(input: String) -> i64 {
    let mut modules: HashMap<&str, Type> = HashMap::new();
    let mut destinations: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut flip_flop_states: HashMap<&str, bool> = HashMap::new();
    let mut conjunction_states: HashMap<&str, HashMap<&str, bool>> = HashMap::new();
    input.lines().for_each(|line| {
        let (name, destinations_str) = line.split_once(" -> ").unwrap();
        let tp = match name.chars().nth(0).unwrap() {
            '%' => Type::FlipFlop,
            '&' => Type::Conjunction,
            'b' => Type::Broadcaster,
            c => panic!("unknown type {}", c)
        };

        let name_fr = &name[1..name.len()];
        modules.insert(name_fr, tp);

        let dests: Vec<&str> = destinations_str.split(", ").collect();
        destinations.insert(name_fr, dests);
    });

    // Pre-populate conjunction states
    destinations.iter().for_each(|(from, dests)| {
        dests.iter().for_each(|dest| {
            if let Some(tp) = modules.get(dest) {
                if *tp == Type::Conjunction {
                    conjunction_states.entry(dest).or_insert(HashMap::new()).insert(from, false);
                }
            }
        })
    });

    let mut ans = 0;
    let mut lcms = HashMap::new();
    loop {
        let mut stack = VecDeque::new();
        ans += 1;
        stack.push_back((false, "roadcaster", "button"));
        while let Some((high, key, from)) = stack.pop_front() {
            // &vt -> &tj -> rx
            // &sk -> &tj -> rx
            // &xc -> &tj -> rx
            // &kk -> &tj -> rx
            if key == "vt" && !high {
                lcms.insert("vt", ans);
            }
            if key == "sk" && !high {
                lcms.insert("sk", ans);
            }
            if key == "xc" && !high {
                lcms.insert("xc", ans);
            }
            if key == "kk" && !high {
                lcms.insert("kk", ans);
            }
            if lcms.contains_key("vt") && lcms.contains_key("sk") && lcms.contains_key("xc") && lcms.contains_key("kk") {
                let vt = *lcms.get("vt").unwrap();
                let sk = *lcms.get("sk").unwrap();
                let xc = *lcms.get("xc").unwrap();
                let kk = *lcms.get("kk").unwrap();
                return lcm_64(lcm_64(lcm_64(vt, sk), xc), kk);
            }

            if key == "rx" && !high {
                return ans;
            }

            if !modules.contains_key(key) { continue }
            let send_high: bool;
            match *modules.get(key).unwrap() {
                Type::FlipFlop => {
                    if high { continue; }
                    let state = *flip_flop_states.entry(key).or_insert(false);
                    flip_flop_states.insert(key, !state);
                    send_high = !state;
                }
                Type::Conjunction => {
                    let state = conjunction_states.entry(key).or_insert(HashMap::new());
                    state.insert(from, high);
                    if state.values().all(|v| *v) { send_high = false } else { send_high = true }
                }
                Type::Broadcaster => { send_high = high }
            }
            for dest in destinations.get(key).unwrap() {
                stack.push_back((send_high, dest, key));
            }
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Type {
    FlipFlop,
    Conjunction,
    Broadcaster,
}

#[cfg(test)]
mod day20_tests {
    use std::fs;

    use crate::y2023::day20::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(32000000, part_a(fs::read_to_string("input/2023/day20/test.txt").unwrap()));
        assert_eq!(11687500, part_a(fs::read_to_string("input/2023/day20/test_2.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(818649769, part_a(fs::read_to_string("input/2023/day20/input.txt").unwrap()));
        assert_eq!(246313604784977, part_b(fs::read_to_string("input/2023/day20/input.txt").unwrap()));
    }
}
