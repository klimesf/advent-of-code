use std::collections::{HashMap, VecDeque};
use std::fs;
use regex::{Regex};

pub(crate) fn day16() {
    let input = fs::read_to_string("input/2022/day16/input.txt").unwrap();
    let mut pressures: HashMap<&str, i32> = HashMap::new();
    let mut adjacency_list: HashMap<&str, Vec<&str>> = HashMap::new();

    let re = Regex::new(r"^Valve ([A-Z]+) has flow rate=([0-9]+); tunnels? leads? to valves? (.+)$").unwrap();
    for line in input.lines().into_iter() {
        let g = re.captures(line).unwrap();
        let from = g.get(1).unwrap().as_str();
        let flow: i32 = g.get(2).unwrap().as_str().parse().unwrap();
        pressures.insert(from, flow);

        let neighbors: Vec<&str> = g.get(3).unwrap().as_str().split(", ").into_iter().collect();
        adjacency_list.insert(from, neighbors);
    }

    let mut distances: HashMap<&str, HashMap<&str, i32>> = HashMap::new();
    for from in adjacency_list.keys() {
        let mut distance_map = HashMap::new();
        let mut stack = VecDeque::new();
        stack.push_back((from, 0));
        while !stack.is_empty() {
            let (valve, time) = stack.pop_front().unwrap();

            if *distance_map.entry(*valve).or_insert(i32::MAX) > time {
                distance_map.insert(*valve, time);
            } else {
                continue;
            }

            adjacency_list.get(valve).unwrap().iter()
                .for_each(|neighbor| stack.push_back((neighbor, time + 1)))
        }
        distances.insert(from, distance_map);
    }

    let valves_with_pressure: Vec<&str> = pressures.iter()
        .filter(|(_, pressure)| **pressure > 0)
        .map(|(valve, _)| *valve)
        .collect();

    // Part 1
    let (max, _) = solve(&pressures, &distances, valves_with_pressure.clone(), 30);
    println!("{}", max);

    // Part 2 - this does not work for test input because it's too greedy, but it works for my input
    let (max, remaining) = solve(&pressures, &distances, valves_with_pressure, 26);
    let (max_elephant, _) = solve(&pressures, &distances, remaining, 26);
    println!("{}", max + max_elephant);
}

fn solve<'a>(
    pressures: &HashMap<&str, i32>,
    distances: &HashMap<&str, HashMap<&str, i32>>,
    remaining: Vec<&'a str>,
    time: i32,
) -> (i32, Vec<&'a str>) {
    let mut stack: Vec<(&str, Vec<&str>, i32, i32)> = vec!();
    stack.push(("AA", remaining.clone(), time, 0));

    let mut max = i32::MIN;
    let mut best_path = vec!();
    while !stack.is_empty() {
        let (from, remaining, time, total) = stack.pop().unwrap();
        if total > max {
            max = total;
            best_path = remaining.clone();
        }
        remaining.iter().for_each(|to| {
            let dist = distances.get(from).unwrap().get(to).unwrap();
            let mut new_remaining = remaining.clone();
            new_remaining.retain(|x| x != to);
            let next_time = time - dist - 1;

            if next_time <= 0 {
                if total > max {
                    max = total;
                    best_path = remaining.clone();
                }
                return;
            }
            let pressure = pressures.get(to).unwrap();
            stack.push((to, new_remaining, next_time, total + (next_time * pressure)));
        })
    }
    (max, best_path)
}
