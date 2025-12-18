use std::collections::HashMap;
use std::fs;

pub(crate) fn day04() {
    println!("{}", part_a(fs::read_to_string("input/2023/day04/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2023/day04/input.txt").unwrap()));
}

fn part_a(cards: String) -> i32 {
    cards
        .lines()
        .map(|line| {
            let (_, numbers) = line.split_once(": ").unwrap();

            let (winning_s, right) = numbers.split_once(" | ").unwrap();

            let winning: Vec<i32> = winning_s
                .split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect();

            let matches: Vec<i32> = right
                .split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .filter(|n| winning.contains(n))
                .collect();

            if matches.len() < 1 {
                return 0;
            }

            let mut ans = 1;
            for _ in 1..matches.len() {
                ans *= 2;
            }

            ans
        })
        .sum()
}

fn part_b(cards: String) -> usize {
    let mut ctr: HashMap<usize, usize> = HashMap::new();
    cards
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let mult = *ctr.entry(i).or_insert(1);
            let (_, numbers) = line.split_once(": ").unwrap();

            let (winning_s, right) = numbers.split_once(" | ").unwrap();

            let winning: Vec<usize> = winning_s
                .split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect();

            let matches: Vec<usize> = right
                .split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .filter(|n| winning.contains(n))
                .collect();

            for j in 1..=matches.len() {
                *ctr.entry(i + j).or_insert(1) += mult;
            }

            mult
        })
        .sum()
}

#[cfg(test)]
mod day04_tests {
    use std::fs;

    use crate::y2023::day04::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(13, part_a(fs::read_to_string("input/2023/day04/test.txt").unwrap()));
        assert_eq!(30, part_b(fs::read_to_string("input/2023/day04/test_b.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(24175, part_a(fs::read_to_string("input/2023/day04/input.txt").unwrap()));
        assert_eq!(18846301, part_b(fs::read_to_string("input/2023/day04/input.txt").unwrap()));
    }
}
