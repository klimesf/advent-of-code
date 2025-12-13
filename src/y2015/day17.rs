use itertools::Itertools;
use std::fs;

pub(crate) fn day17() {
    println!(
        "{}",
        part_a(
            fs::read_to_string("input/2015/day17/input.txt").unwrap(),
            150
        )
    );
    println!(
        "{}",
        part_b(
            fs::read_to_string("input/2015/day17/input.txt").unwrap(),
            150
        )
    );
}

fn part_a(input: String, liters: usize) -> usize {
    let containers: Vec<usize> = input.lines().map(|s| s.parse().unwrap()).collect();

    let mut ans = 0;
    for i in 2..containers.len() {
        containers.iter().combinations(i).for_each(|c| {
            if c.iter().map(|n| *n).sum::<usize>() == liters {
                ans += 1
            }
        });
    }
    ans
}

fn part_b(input: String, liters: usize) -> usize {
    let containers: Vec<usize> = input.lines().map(|s| s.parse().unwrap()).collect();

    let mut ans = 0;
    for i in 2..containers.len() {
        containers.iter().combinations(i).for_each(|c| {
            if c.iter().map(|n| *n).sum::<usize>() == liters {
                ans += 1
            }
        });
        if ans > 0 {
            break;
        }
    }
    ans
}

#[cfg(test)]
mod day17_tests {
    use std::fs;

    use crate::y2015::day17::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(
            4,
            part_a(fs::read_to_string("input/2015/day17/test.txt").unwrap(), 25)
        );
        assert_eq!(
            3,
            part_b(fs::read_to_string("input/2015/day17/test.txt").unwrap(), 25)
        );
    }

    #[test]
    fn input_works() {
        assert_eq!(
            1638,
            part_a(
                fs::read_to_string("input/2015/day17/input.txt").unwrap(),
                150
            )
        );
        assert_eq!(
            17,
            part_b(
                fs::read_to_string("input/2015/day17/input.txt").unwrap(),
                150
            )
        );
    }
}
