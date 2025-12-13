use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fs;

pub(crate) fn day08(print: fn(usize)) {
    print(part_a(
        fs::read_to_string("input/2025/day08/input.txt").unwrap(),
        1000,
    ));
    print(part_b(
        fs::read_to_string("input/2025/day08/input.txt").unwrap(),
    ));
}

fn part_a(input: String, iterations: usize) -> usize {
    let nums: Vec<(i64, i64, i64)> = input
        .lines()
        .map(|line| {
            let vec: Vec<i64> = line.split(',').map(|num| num.parse().unwrap()).collect();
            (vec[0], vec[1], vec[2])
        })
        .collect();

    let mut parents: Vec<usize> = vec![0; nums.len()];
    for i in 0..nums.len() {
        parents[i] = i;
    }
    let mut heap = BinaryHeap::new();
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if i == j {
                continue;
            }
            let num_a = nums[i].clone();
            let num_b = nums[j].clone();
            let distance = (num_a.0 - num_b.0).pow(2)
                + (num_a.1 - num_b.1).pow(2)
                + (num_a.2 - num_b.2).pow(2);
            heap.push(NodePair { i, j, distance })
        }
    }

    for _ in 0..iterations {
        let pair = heap.pop().unwrap();
        if parents[pair.i] == parents[pair.j] {
            continue;
        }
        union(pair.i, pair.j, &mut parents);
    }

    let mut circuit_sizes = HashMap::new();
    for i in 0..parents.len() {
        *circuit_sizes.entry(parents[i]).or_insert(0) += 1;
    }
    circuit_sizes.values().sorted().rev().take(3).product()
}

fn part_b(input: String) -> usize {
    let nums: Vec<(i64, i64, i64)> = input
        .lines()
        .map(|line| {
            let vec: Vec<i64> = line.split(',').map(|num| num.parse().unwrap()).collect();
            (vec[0], vec[1], vec[2])
        })
        .collect();

    let mut parents: Vec<usize> = vec![0; nums.len()];
    for i in 0..nums.len() {
        parents[i] = i;
    }
    let mut heap = BinaryHeap::new();
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if i == j {
                continue;
            }
            let num_a = nums[i].clone();
            let num_b = nums[j].clone();
            let distance = (num_a.0 - num_b.0).pow(2)
                + (num_a.1 - num_b.1).pow(2)
                + (num_a.2 - num_b.2).pow(2);
            heap.push(NodePair { i, j, distance })
        }
    }

    let mut last = 0;
    let mut circuit_count = nums.len();
    while circuit_count > 1 {
        let pair = heap.pop().unwrap();
        if parents[pair.i] == parents[pair.j] {
            continue;
        }
        union(pair.i, pair.j, &mut parents);
        circuit_count -= 1;
        last = (nums[pair.i].0 * nums[pair.j].0) as usize;
    }
    last
}

fn union(a: usize, b: usize, parents: &mut Vec<usize>) {
    let parent_a = parents[a];
    let parent_b = parents[b];
    for i in 0..parents.len() {
        if parents[i] == parent_a {
            parents[i] = parent_b;
        }
    }
    parents[parent_a] = parent_b;
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct NodePair {
    i: usize,
    j: usize,
    distance: i64,
}

impl Ord for NodePair {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for NodePair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod day08_tests {
    use std::fs;

    use crate::y2025::day08::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(
            40,
            part_a(fs::read_to_string("input/2025/day08/test.txt").unwrap(), 10)
        );
        assert_eq!(
            25272,
            part_b(fs::read_to_string("input/2025/day08/test.txt").unwrap())
        );
    }

    #[test]
    fn input_works() {
        assert_eq!(
            330786,
            part_a(
                fs::read_to_string("input/2025/day08/input.txt").unwrap(),
                1000
            )
        );
        assert_eq!(
            3276581616,
            part_b(fs::read_to_string("input/2025/day08/input.txt").unwrap())
        );
    }
}
