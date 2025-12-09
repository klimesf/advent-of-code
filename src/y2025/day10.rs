use std::cmp::Ordering;
use std::collections::{BinaryHeap};
use std::fs;
use rayon::iter::ParallelIterator;
use rayon::iter::IntoParallelRefIterator;

pub(crate) fn day10(print: fn(usize)) {
    // print(part_a(fs::read_to_string("input/2025/day10/input.txt").unwrap()));
    print(part_b(fs::read_to_string("input/2025/day10/test.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    let mut total = 0;
    input.lines().for_each(|line| {
        let parts = line.split_whitespace().collect::<Vec<&str>>();

        let target_str = parts[0][1..parts[0].len() - 1].to_string();
        let target = target_str.chars().map(|c| {
            if c == '.' { false } else { true }
        }).collect::<Vec<bool>>();

        let presses = parts[1..parts.len() - 1].iter().map(|part| {
            part[1..part.len() - 1].split(",").map(|part| part.parse::<usize>().unwrap()).collect::<Vec<usize>>()
        }).collect::<Vec<Vec<usize>>>();

        let min = solve_a(target, presses);
        total += min;
    });
    total
}

fn solve_a(target: Vec<bool>, presses: Vec<Vec<usize>>) -> usize {
    let mut heap = BinaryHeap::new();
    heap.push(StateA { state: vec![false; target.len()], presses: 0 });
    
    while let Some(next_state) = heap.pop() {
        if next_state.state == target { return next_state.presses }
        
        for press in presses.iter() {
            let mut new_state = next_state.state.clone();
            for i in 0..press.len() {
                let pos = press[i];
                new_state[pos] = !new_state[pos];
            }
            let bump = StateA { state: new_state.clone(), presses: next_state.presses + 1 };
            heap.push(bump);
        }
    }
    0
}

#[derive(Eq, PartialEq)]
struct StateA {
    state: Vec<bool>,
    presses: usize,
}

impl Ord for StateA {
    fn cmp(&self, other: &Self) -> Ordering {
        other.presses.cmp(&self.presses)
    }
}

impl PartialOrd for StateA {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part_b(input: String) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    lines.par_iter().map(|line| {
        let parts = line.split_whitespace().collect::<Vec<&str>>();

        let presses = parts[1..parts.len() - 1].iter().map(|part| {
            part[1..part.len() - 1].split(",").map(|part| part.parse::<usize>().unwrap()).collect::<Vec<usize>>()
        }).collect::<Vec<Vec<usize>>>();
        
        let joltage_str = parts[parts.len() - 1][1..parts[parts.len() - 1].len() - 1].to_string();
        let joltage = joltage_str.split(",").map(|part| part.parse::<usize>().unwrap()).collect::<Vec<usize>>();

        let min = solve_b(joltage, presses);
        min
    }).sum()
}

// Solved by Z3 first
fn solve_b(joltage: Vec<usize>, presses: Vec<Vec<usize>>) -> usize {
    let mut heap = BinaryHeap::new();
    heap.push(StateB {
        joltage: vec![0; joltage.len()],
        presses: 0,
        dist: calc_dist(&joltage, &vec![0; joltage.len()], 0),
    });

    'outer: while let Some(next_state) = heap.pop() {
        if next_state.joltage == joltage { return next_state.presses }
        for i in 0..joltage.len() {
            if next_state.joltage[i] > joltage[i] { continue 'outer; }
        }

        for press in presses.iter() {
            let mut new_joltage = next_state.joltage.clone();
            for i in 0..press.len() {
                let pos = press[i];
                new_joltage[pos] += 1;
            }
            let bump = StateB {
                joltage: new_joltage.clone(),
                presses: next_state.presses + 1,
                dist: calc_dist(&joltage, &new_joltage, next_state.presses + 1),
            };
            heap.push(bump);
        }
    }
    0
}

fn calc_dist(joltage: &Vec<usize>, joltage_state: &Vec<usize>, _presses: usize) -> usize {
    let mut dist = 0;
    for i in 0..joltage.len() {
        let a = joltage_state[i];
        let b = joltage[i];
        dist += a.max(b) - a.min(b);
    }
    dist
}

#[derive(Eq, PartialEq)]
struct StateB {
    joltage: Vec<usize>,
    presses: usize,
    dist: usize,
}

impl Ord for StateB {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl PartialOrd for StateB {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod day10_tests {
    use std::fs;

    use crate::y2025::day10::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(7, part_a(fs::read_to_string("input/2025/day10/test.txt").unwrap()));
        assert_eq!(33, part_b(fs::read_to_string("input/2025/day10/test.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(498, part_a(fs::read_to_string("input/2025/day10/input.txt").unwrap()));
        assert_eq!(0, part_b(fs::read_to_string("input/2025/day10/input.txt").unwrap()));
    }
}
