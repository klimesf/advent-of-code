use std::collections::{HashSet, VecDeque};
use std::fs;

use itertools::Itertools;

use crate::y2017::day10::knot_hash;

pub(crate) fn day14() {
    let input = fs::read_to_string("input/2017/day14/input.txt").unwrap();
    let data = parse(input.trim());
    println!("{}", count_used(&data));
    println!("{}", count_regions(&data));
}

fn parse(input: &str) -> Vec<Vec<bool>> {
    (0..128)
        .into_iter()
        .map(|i| {
            let hash = knot_hash(format!("{}-{}", input, i).as_str());
            hash.chars()
                .into_iter()
                .tuples()
                .map(|(a, b)| u8::from_str_radix(format!("{}{}", a, b).as_str(), 16).unwrap())
                .flat_map(|mut num| {
                    let mut cell = vec![];
                    for _ in 0..8 {
                        cell.push(num & 1 == 1);
                        num = num >> 1;
                    }
                    cell.reverse();
                    cell
                })
                .collect()
        })
        .collect()
}

fn count_used(data: &Vec<Vec<bool>>) -> usize {
    data.iter().map(|row| row.iter().filter(|c| **c).count()).sum()
}

fn count_regions(data: &Vec<Vec<bool>>) -> usize {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    visited.reserve(128 * 128);
    let mut group_ctr = 0;

    for x in 0..128 {
        for y in 0..128 {
            if data[x][y] && !visited.contains(&(x, y)) {
                group_ctr += 1;
                let mut stack = VecDeque::new();
                stack.push_back((x, y));

                // Run BFS to each adjacent used cell
                while let Some(next) = stack.pop_front() {
                    if !visited.insert(next) {
                        continue;
                    }
                    if next.0 > 0 && data[next.0 - 1][next.1] {
                        stack.push_back((next.0 - 1, next.1))
                    }
                    if next.0 < 127 && data[next.0 + 1][next.1] {
                        stack.push_back((next.0 + 1, next.1))
                    }
                    if next.1 > 0 && data[next.0][next.1 - 1] {
                        stack.push_back((next.0, next.1 - 1))
                    }
                    if next.1 < 127 && data[next.0][next.1 + 1] {
                        stack.push_back((next.0, next.1 + 1))
                    }
                }
            }
        }
    }

    group_ctr
}

#[cfg(test)]
mod day14_tests {
    use std::fs;

    use crate::y2017::day14::{count_regions, count_used, parse};

    #[test]
    fn test_works() {
        let data = parse("flqrgnkx");
        assert_eq!(8108, count_used(&data));
        assert_eq!(1242, count_regions(&data));
    }

    #[test]
    fn input_works() {
        let input = fs::read_to_string("input/2017/day14/input.txt").unwrap();
        let data = parse(input.trim());
        assert_eq!(8250, count_used(&data));
        assert_eq!(1113, count_regions(&data));
    }
}
