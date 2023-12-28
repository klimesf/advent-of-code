use std::fs;
use itertools::Itertools;

pub(crate) fn day24() {
    println!("{}", part_a(fs::read_to_string("input/2015/day24/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2015/day24/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    let items: Vec<usize> = input.lines().map(|line| { line.parse::<usize>().unwrap() }).collect();

    for group_a_size in 1..items.len() {
        for group_a in items.iter().combinations(group_a_size) {
            let remaining: Vec<usize> = items.iter()
                .filter(|n| !group_a.contains(n))
                .map(|n| *n)
                .collect();
            let sum = group_a.iter().map(|n| **n).sum();

            if sum != remaining.iter().sum::<usize>() / 2 {
                continue
            }

            if two_subsets_sum(remaining, sum) {
                // combinations are returned in order of the original vec, so the first split we found is already min
                return group_a.iter().map(|n| **n).product()
            }
        }
    }
    panic!("no item config found")
}

fn part_b(input: String) -> usize {
    let items: Vec<usize> = input.lines().map(|line| { line.parse::<usize>().unwrap() }).collect();

    for group_a_size in 1..items.len() {
        for group_a in items.iter().combinations(group_a_size) {
            let remaining: Vec<usize> = items.iter()
                .filter(|n| !group_a.contains(n))
                .map(|n| *n)
                .collect();
            let sum = group_a.iter().map(|n| **n).sum();

            if sum != remaining.iter().sum::<usize>() / 3 {
                continue
            }

            if three_subsets_sum(remaining, sum) {
                // combinations are returned in order of the original vec, so the first split we found is already min
                return group_a.iter().map(|n| **n).product()
            }
        }
    }
    panic!("no item config found")
}

fn two_subsets_sum(items: Vec<usize>, target: usize) -> bool {
    let mut stack = vec!();
    stack.push((0, 0, 0, items.iter().sum::<usize>()));

    while let Some((i, sum_b, sum_c, remaining)) = stack.pop() {
        if sum_b == target && sum_c == target && i == items.len() { return true }
        if sum_b > target || sum_c > target { continue }
        if i >= items.len() { continue }

        if sum_c + items[i] <= target {
            stack.push((i + 1, sum_b, sum_c + items[i], remaining - items[i]));
        }
        if sum_b + items[i] <= target {
            stack.push((i + 1, sum_b + items[i], sum_c, remaining - items[i]));
        }
    }
    false
}

fn three_subsets_sum(items: Vec<usize>, target: usize) -> bool {
    let mut stack = vec!();
    stack.push((0, 0, 0, 0, items.iter().sum::<usize>()));

    while let Some((i, sum_b, sum_c, sum_d, remaining)) = stack.pop() {
        if sum_b == target && sum_c == target && sum_d == target && i == items.len() { return true }
        if sum_b > target || sum_c > target || sum_d > target { continue }
        if i >= items.len() { continue }

        if sum_d + items[i] <= target {
            stack.push((i + 1, sum_b, sum_c, sum_d + items[i], remaining - items[i]));
        }
        if sum_c + items[i] <= target {
            stack.push((i + 1, sum_b, sum_c + items[i], sum_d, remaining - items[i]));
        }
        if sum_b + items[i] <= target {
            stack.push((i + 1, sum_b + items[i], sum_c, sum_d, remaining - items[i]));
        }
    }
    false
}

#[cfg(test)]
mod day24_tests {
    use std::fs;

    use crate::y2015::day24::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(99, part_a(fs::read_to_string("input/2015/day24/test.txt").unwrap()));
        assert_eq!(44, part_b(fs::read_to_string("input/2015/day24/test.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(10439961859, part_a(fs::read_to_string("input/2015/day24/input.txt").unwrap()));
        assert_eq!(72050269, part_b(fs::read_to_string("input/2015/day24/input.txt").unwrap()));
    }
}
