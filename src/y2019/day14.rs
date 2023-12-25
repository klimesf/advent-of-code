use std::collections::{HashMap, HashSet};
use std::fs;

pub(crate) fn day14() {
    let input = fs::read_to_string("input/2019/day14/input.txt").unwrap();
    let mut chemicals = HashSet::new();
    let mut reactions: HashMap<String, (usize, Vec<(usize, String)>)> = HashMap::new();
    for line in input.trim().split('\n') {
        let (from, to) = line.split_once(" => ").unwrap();
        let (to_price, to_name) = to.split_once(" ").unwrap();
        let from_parts: Vec<(usize, String)> = from
            .split(", ")
            .map(|s| s.split_once(" ").unwrap())
            .map(|(price, name)| (price.parse().unwrap(), name.to_string()))
            .collect();

        chemicals.insert(to_name.to_string());
        from_parts.iter().for_each(|(_, name)| { chemicals.insert(name.to_string()); });

        reactions.insert(to_name.to_string(), (to_price.parse().unwrap(), from_parts));
    }

    // Create topological ordering of the graph of chemical reactions
    let mut ordering: Vec<String> = vec!();
    let mut visited: HashSet<String> = HashSet::new();

    for c in chemicals {
        if visited.contains(&c) { continue; }
        topological_order(c.to_string(), &mut ordering, &mut visited, &reactions);
    }
    println!("{:?}", ordering);

    // Now go through the graph in topological order and count how much of each chemical is required
    let mut required = HashMap::new();
    ordering.reverse();
    for c in &ordering {
        if !reactions.contains_key(c) { continue; }

        let (c_created, from) = reactions.get(c).unwrap();
        let c_required = *required.entry(c.to_string()).or_insert(1 as usize);
        let c_to_make = (c_required + c_created - 1) / c_created; // Division with rounding up

        for (from_needed, from_name) in from {
            *required.entry(from_name.to_string()).or_insert(0) += c_to_make * from_needed;
        }
    }

    println!("{:?}", required);
    let ore_per_fuel = required.get(&"ORE".to_string()).unwrap();
    println!("{}", ore_per_fuel);

    // Part B
    let mut i = 1000000000000 / ore_per_fuel;
    loop {
        let mut required = HashMap::new();
        required.insert("FUEL".to_string(), i);
        for c in &ordering {
            if !reactions.contains_key(c) { continue; }

            let (c_created, from) = reactions.get(c).unwrap();
            let c_required = required.get(c).unwrap();
            let c_to_make = (c_required + c_created - 1) / c_created; // Division with rounding up

            for (from_needed, from_name) in from {
                *required.entry(from_name.to_string()).or_insert(0) += c_to_make * from_needed;
            }
        }
        if *required.get(&"ORE".to_string()).unwrap() > 1000000000000 as usize {
            break;
        }
        // println!("{}: {}", i, *required.get(&"ORE".to_string()).unwrap());
        i += 1;
    }
    println!("{}", i - 1)
}

fn topological_order(c: String, ordering: &mut Vec<String>, visited: &mut HashSet<String>, reactions: &HashMap<String, (usize, Vec<(usize, String)>)>) {
    if visited.contains(&c) { return; }

    let edges = match reactions.get(&c) {
        Some((_, edges)) => edges.clone(),
        None => vec!(),
    };

    for (_, e) in edges {
        topological_order(e.to_string(), ordering, visited, reactions);
    }

    visited.insert(c.clone());
    ordering.push(c.clone());
}

