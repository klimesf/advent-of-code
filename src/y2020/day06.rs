use std::collections::HashSet;
use std::fs;

pub(crate) fn day06() {
    println!(
        "{}",
        part_a(fs::read_to_string("input/2020/day06/input.txt").unwrap())
    );
    println!(
        "{}",
        part_b(fs::read_to_string("input/2020/day06/input.txt").unwrap())
    );
}

fn part_a(input: String) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            group
                .chars()
                .filter(|c| *c != '\n')
                .collect::<HashSet<char>>()
                .len()
        })
        .sum()
}

fn part_b(input: String) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|line| line.chars().collect::<HashSet<char>>())
                .reduce(|a, b| a.intersection(&b).copied().collect())
                .unwrap()
                .len()
        })
        .sum()
}

#[cfg(test)]
mod day06_tests {
    use std::fs;

    use crate::y2020::day06::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(
            11,
            part_a(fs::read_to_string("input/2020/day06/test.txt").unwrap())
        );
        assert_eq!(
            6,
            part_b(fs::read_to_string("input/2020/day06/test.txt").unwrap())
        );
    }

    #[test]
    fn input_works() {
        assert_eq!(
            6680,
            part_a(fs::read_to_string("input/2020/day06/input.txt").unwrap())
        );
        assert_eq!(
            3117,
            part_b(fs::read_to_string("input/2020/day06/input.txt").unwrap())
        );
    }
}
