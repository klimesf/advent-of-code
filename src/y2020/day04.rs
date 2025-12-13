use std::collections::HashSet;
use std::fs;

pub(crate) fn day04() {
    println!(
        "{}",
        part_a(fs::read_to_string("input/2020/day04/input.txt").unwrap())
    );
    println!(
        "{}",
        part_b(fs::read_to_string("input/2020/day04/input.txt").unwrap())
    );
}

fn part_a(input: String) -> usize {
    input
        .split("\n\n")
        .filter(|p| {
            let fields = p.split_whitespace().collect::<Vec<&str>>();
            let keys = fields
                .iter()
                .map(|value| {
                    let (key, _) = value.split_once(":").unwrap();
                    key
                })
                .collect::<HashSet<&str>>();

            ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
                .iter()
                .all(|k| keys.contains(k))
        })
        .count()
}

fn part_b(_: String) -> usize {
    0
}

#[cfg(test)]
mod day04_tests {
    use std::fs;

    use crate::y2020::day04::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(
            2,
            part_a(fs::read_to_string("input/2020/day04/test.txt").unwrap())
        );
        assert_eq!(
            0,
            part_b(fs::read_to_string("input/2020/day04/test.txt").unwrap())
        );
    }

    #[test]
    fn input_works() {
        assert_eq!(
            254,
            part_a(fs::read_to_string("input/2020/day04/input.txt").unwrap())
        );
        assert_eq!(
            0,
            part_b(fs::read_to_string("input/2020/day04/input.txt").unwrap())
        );
    }
}
