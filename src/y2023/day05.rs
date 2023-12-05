use std::{cmp, fs};

use itertools::Itertools;

pub(crate) fn day05() {
    println!("{}", part_a(fs::read_to_string("input/2023/day05/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2023/day05/input.txt").unwrap()));
}

fn parse_input(input: String) -> (Vec<i64>, Vec<Vec<(i64, i64, i64)>>) {
    let mut seeds: Vec<i64> = vec!();
    let mut current_map: Vec<(i64, i64, i64)> = vec!();
    let mut maps: Vec<Vec<(i64, i64, i64)>> = vec!();
    input.lines().for_each(|line| {
        if line.starts_with("seeds: ") {
            seeds = (&line[7..line.len()]).split_whitespace().map(|n| n.parse::<i64>().unwrap()).collect();
            return;
        }

        if line.trim().is_empty() {
            return;
        }

        if line.contains("map") {
            if current_map.len() > 0 {
                maps.push(current_map.clone());
                current_map = vec!();
            }
            return;
        }

        // parse numbers
        let nums: Vec<i64> = line.split_whitespace().map(|n| n.parse::<i64>().unwrap()).collect();
        let destination_start = nums[0];
        let source_start = nums[1];
        let source_end = nums[1] + nums[2];
        current_map.push((source_start, source_end, destination_start));
    });
    maps.push(current_map.clone());

    (seeds, maps)
}

fn part_a(input: String) -> i64 {
    let (seeds, maps) = parse_input(input);

    seeds.iter().map(|seed| {
        let mut ptr = *seed;
        for map in &maps {
            for (source_start, source_end, destination_start) in map {
                if *source_start <= ptr && ptr < *source_end {
                    ptr = ptr + (destination_start - source_start);
                    break;
                }
            }
            // let the seed be the same
        }
        ptr
    }).min().unwrap()
}

fn part_b(input: String) -> i64 {
    let (seeds, maps) = parse_input(input);
    let mut carryover_ranges: Vec<(i64, i64)> = seeds.into_iter().tuples().map(|(seed_start, seed_range)| {
        (seed_start, seed_start + seed_range - 1)
    }).collect();

    for map in &maps {
        let mut new_ranges: Vec<(i64, i64)> = vec!();

        for range in carryover_ranges {
            let mut to_match: Vec<(i64, i64)> = vec!{range};

            for (source_start, source_end, destination_start) in map {
                if to_match.is_empty() { break }
                let mut next_to_match = vec!();
                for (range_start, range_end) in &to_match {
                    let start = cmp::max(range_start, source_start);
                    let end = cmp::min(range_end, source_end);
                    if start > end {
                        next_to_match.push((*range_start, *range_end));
                        continue;
                    }

                    new_ranges.push((
                        start + (destination_start - source_start),
                        end + (destination_start - source_start),
                    ));

                    if range_start < start {
                        next_to_match.push((*range_start, start - 1));
                    }
                    if end < range_end {
                        next_to_match.push((end + 1, *range_end));
                    }
                }
                to_match = next_to_match;
            }

            if !to_match.is_empty() { // Some ranges did not match anything
                for range in &to_match {
                    new_ranges.push(*range);
                }
            }
        }

        carryover_ranges = new_ranges;
    }

    carryover_ranges.iter().map(|(s, _)| *s).min().unwrap()
}

#[cfg(test)]
mod day05_tests {
    use std::fs;

    use crate::y2023::day05::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(35, part_a(fs::read_to_string("input/2023/day05/test.txt").unwrap()));
        assert_eq!(46, part_b(fs::read_to_string("input/2023/day05/test.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(484023871, part_a(fs::read_to_string("input/2023/day05/input.txt").unwrap()));
        assert_eq!(46294175, part_b(fs::read_to_string("input/2023/day05/input.txt").unwrap()));
    }
}
