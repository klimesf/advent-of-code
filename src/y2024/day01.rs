use std::collections::HashMap;
use std::fs;

pub fn day01(print: fn(u32)) {
    print(part_a(fs::read_to_string("input/2024/day01/input.txt").unwrap()));
    print(part_b(fs::read_to_string("input/2024/day01/input.txt").unwrap()));
}

fn part_a(input: String) -> u32 {
    let (mut left_list, mut right_list) = parse_input(input);

    left_list.sort();
    right_list.sort();

    (0..left_list.len()).map(|i| {
        let l = left_list[i];
        let r = right_list[i];
        l.max(r) - l.min(r)
    }).sum()
}

fn part_b(input: String) -> u32 {
    let (left_list, right_list) = parse_input(input);

    let mut similarities: HashMap<u32, u32> = HashMap::new();
    right_list
        .iter()
        .for_each(|r| {
            similarities.entry(*r).and_modify(|v| *v += 1).or_insert(1);
        });

    (0..left_list.len()).map(|i| {
        let l = left_list[i];
        let similarity = *similarities.get(&l).unwrap_or(&0);
        l * similarity
    }).sum()
}

fn parse_input(input: String) -> (Vec<u32>, Vec<u32>) {
    let mut left_list: Vec<u32> = vec!();
    let mut right_list: Vec<u32> = vec!();
    input.lines().for_each(|line| {
        let (l, r) = line.split_once("   ").unwrap();
        left_list.push(l.parse().unwrap());
        right_list.push(r.parse().unwrap());
    });
    (left_list, right_list)
}

#[cfg(test)]
mod day01_tests {
    use std::fs;

    use crate::y2024::day01::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(11, part_a(fs::read_to_string("input/2024/day01/test.txt").unwrap()));
        assert_eq!(31, part_b(fs::read_to_string("input/2024/day01/test.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(2164381, part_a(fs::read_to_string("input/2024/day01/input.txt").unwrap()));
        assert_eq!(20719933, part_b(fs::read_to_string("input/2024/day01/input.txt").unwrap()));
    }
}
