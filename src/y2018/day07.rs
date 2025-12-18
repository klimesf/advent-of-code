use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;

use itertools::Itertools;
use regex::Regex;

pub(crate) fn day07() {
    let input = fs::read_to_string("input/2018/day07/input.txt").unwrap();
    let re = Regex::new(r"^Step (\w) must be finished before step (\w) can begin\.$").unwrap();
    let graph = input
        .trim()
        .split("\n")
        .sorted()
        .map(|s| {
            let caps = re.captures(s).unwrap();
            let to = caps.get(1).unwrap().as_str().chars().nth(0).unwrap();
            let from = caps.get(2).unwrap().as_str().chars().nth(0).unwrap();
            (from, to)
        })
        .fold(HashMap::new(), |mut map, (from, to)| {
            map.entry(from).or_insert_with(|| HashSet::new()).insert(to);
            map
        });

    part_a(graph.clone());
    part_b(graph.clone());
}

fn part_a(mut graph: HashMap<char, HashSet<char>>) {
    let mut available = BinaryHeap::new();
    let mut visited = HashSet::new();
    let mut order: Vec<char> = vec![];

    ('A'..'Z')
        .filter(|c| graph.entry(*c).or_insert_with(|| HashSet::new()).is_empty())
        .for_each(|c| available.push(Reverse(c)));

    while !available.is_empty() {
        let next = available.pop().unwrap().0;
        if !visited.insert(next) {
            continue;
        }
        order.push(next);

        for (c, deps) in &mut graph {
            deps.remove(&next);
            if deps.is_empty() {
                available.push(Reverse(*c));
            }
        }
    }

    for o in order {
        print!("{}", o);
    }
    println!();
}

fn part_b(mut graph: HashMap<char, HashSet<char>>) {
    const TOTAL_WORKERS: i32 = 5;

    let mut available_at = HashMap::new();
    ('A'..'Z')
        .filter(|c| graph.entry(*c).or_insert_with(|| HashSet::new()).is_empty())
        .for_each(|c| {
            available_at
                .entry(0)
                .or_insert_with(|| BinaryHeap::new())
                .push(Reverse(c))
        });

    let mut finished_at: HashMap<usize, Vec<char>> = HashMap::new();
    let mut finished = HashSet::new();
    let mut available_workers = TOTAL_WORKERS;
    let mut time = BinaryHeap::new();
    time.push(Reverse(0));

    let mut final_time = 0;
    while !time.is_empty() {
        let current_time = time.pop().unwrap();

        if let Some(dones) = finished_at.get(&current_time.0) {
            for done in dones {
                available_workers += 1;
                for (c, deps) in &mut graph {
                    deps.remove(done);
                    if deps.is_empty() && !finished.contains(c) {
                        available_at
                            .entry(current_time.0)
                            .or_insert_with(|| BinaryHeap::new())
                            .push(Reverse(*c));
                    }
                }
            }
        }

        while available_workers > 0
            && !available_at
                .entry(current_time.0)
                .or_insert_with(|| BinaryHeap::new())
                .is_empty()
        {
            let todo = available_at
                .entry(current_time.0)
                .or_insert_with(|| BinaryHeap::new())
                .pop()
                .unwrap()
                .0;
            available_workers -= 1;
            let deadline = current_time.0 + duration(todo);
            finished_at.entry(deadline).or_insert_with(|| Vec::new()).push(todo);
            time.push(Reverse(deadline));
            finished.insert(todo);
        }

        if available_workers == TOTAL_WORKERS && finished.len() == 26 {
            final_time = current_time.0;
        }
    }

    println!("It will take {} workers {} seconds", TOTAL_WORKERS, final_time);
}

fn duration(task: char) -> usize {
    60 + (task as u8 - 64) as usize
}
