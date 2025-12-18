use std::collections::HashMap;
use std::fs;

pub fn day19(print: fn(usize)) {
    print(part_a(fs::read_to_string("input/2024/day19/input.txt").unwrap()));
    print(part_b(fs::read_to_string("input/2024/day19/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    let (sa, sb) = input.split_once("\n\n").unwrap();
    let towels: Vec<Vec<char>> = sa.split(", ").map(|s| s.chars().collect()).collect();
    let patterns: Vec<Vec<char>> = sb.lines().map(|s| s.chars().collect()).collect();

    patterns
        .iter()
        .filter(|pattern| is_possible(pattern, 0, &towels))
        .count()
}

fn is_possible(pattern: &Vec<char>, pos: usize, towels: &Vec<Vec<char>>) -> bool {
    if pos >= pattern.len() {
        return true;
    }
    towels.iter().any(|towel| {
        for i in 0..towel.len() {
            if pos + i >= pattern.len() {
                return false;
            }
            if pattern[pos + i] != towel[i] {
                return false;
            }
        }
        return is_possible(pattern, pos + towel.len(), &towels);
    })
}

fn part_b(input: String) -> usize {
    let (sa, sb) = input.split_once("\n\n").unwrap();
    let towels: Vec<Vec<char>> = sa.split(", ").map(|s| s.chars().collect()).collect();
    let patterns: Vec<Vec<char>> = sb.lines().map(|s| s.chars().collect()).collect();

    let mut cache = HashMap::new();
    patterns
        .iter()
        .map(|pattern| count_possible_ways(pattern, 0, &towels, &mut cache))
        .sum()
}

fn count_possible_ways(
    pattern: &Vec<char>,
    pos: usize,
    towels: &Vec<Vec<char>>,
    cache: &mut HashMap<(Vec<char>, usize), usize>,
) -> usize {
    if pos >= pattern.len() {
        return 1;
    }
    if cache.contains_key(&(pattern.clone(), pos)) {
        return cache[&(pattern.clone(), pos)];
    }
    let mut arrangements = 0;
    towels.iter().for_each(|towel| {
        for i in 0..towel.len() {
            if pos + i >= pattern.len() {
                return;
            }
            if pattern[pos + i] != towel[i] {
                return;
            }
        }
        arrangements += count_possible_ways(pattern, pos + towel.len(), &towels, cache);
    });
    cache.insert((pattern.clone(), pos), arrangements);
    arrangements
}

#[cfg(test)]
mod day19_tests {
    use std::fs;

    use crate::y2024::day19::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(6, part_a(fs::read_to_string("input/2024/day19/test.txt").unwrap()));
        assert_eq!(16, part_b(fs::read_to_string("input/2024/day19/test.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(209, part_a(fs::read_to_string("input/2024/day19/input.txt").unwrap()));
        assert_eq!(777669668613191, part_b(fs::read_to_string("input/2024/day19/input.txt").unwrap()));
    }
}
