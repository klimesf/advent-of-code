use itertools::Itertools;
use std::fs;

pub(crate) fn day09() {
    println!(
        "{}",
        part_a(fs::read_to_string("input/2023/day09/input.txt").unwrap())
    );
    println!(
        "{}",
        part_b(fs::read_to_string("input/2023/day09/input.txt").unwrap())
    );
}

fn part_a(input: String) -> i32 {
    input
        .lines()
        .map(|line| {
            let mut nums: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect();
            let mut last = vec![nums.last().unwrap().clone()];
            while nums.iter().any(|a| *a > 0) {
                let new_nums: Vec<i32> = nums
                    .into_iter()
                    .tuple_windows()
                    .map(|(a, b)| b - a)
                    .collect();
                last.push(new_nums.last().unwrap().clone());
                nums = new_nums;
            }

            let mut ans = 0;
            for i in (0..last.len() - 1).rev() {
                ans = ans + last[i];
            }
            ans
        })
        .sum()
}

fn part_b(input: String) -> i32 {
    input
        .lines()
        .map(|line| {
            let mut nums: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect();
            let mut first = vec![nums.first().unwrap().clone()];
            while nums.iter().any(|a| *a > 0) {
                let new_nums: Vec<i32> = nums
                    .into_iter()
                    .tuple_windows()
                    .map(|(a, b)| b - a)
                    .collect();
                first.push(new_nums.first().unwrap().clone());
                nums = new_nums;
            }

            let mut ans = 0;
            for i in (0..first.len() - 1).rev() {
                ans = first[i] - ans;
            }
            ans
        })
        .sum()
}

#[cfg(test)]
mod day09_tests {
    use std::fs;

    use crate::y2023::day09::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(
            114,
            part_a(fs::read_to_string("input/2023/day09/test.txt").unwrap())
        );
        assert_eq!(
            2,
            part_b(fs::read_to_string("input/2023/day09/test.txt").unwrap())
        );
    }

    #[test]
    fn input_works() {
        assert_eq!(
            1974913025,
            part_a(fs::read_to_string("input/2023/day09/input.txt").unwrap())
        );
        assert_eq!(
            884,
            part_b(fs::read_to_string("input/2023/day09/input.txt").unwrap())
        );
    }
}
