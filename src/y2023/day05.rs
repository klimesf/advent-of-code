use std::{fs};
use itertools::Itertools;

pub(crate) fn day05() {
    println!("{}", part_a(fs::read_to_string("input/2023/day05/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2023/day05/input.txt").unwrap()));
}

fn parse_input(input: String) -> (Vec<usize>, Vec<Vec<(usize, usize, usize, usize)>>) {
    let mut seeds: Vec<usize> = vec!{};
    let mut current_map: Vec<(usize, usize, usize, usize)> = vec!{};
    let mut maps: Vec<Vec<(usize, usize, usize, usize)>> = vec!{};
    input.lines().for_each(|line| {
        if line.starts_with("seeds: ") {
            seeds = (&line[7..line.len()]).split_whitespace().map(|n| n.parse::<usize>().unwrap()).collect();
            return;
        }

        if line.trim().is_empty() {
            return;
        }

        if line.starts_with("seed-to-soil") {
            return;
        } else if line.starts_with("soil-to-fertilizer") {
            maps.push(current_map.clone());
            current_map = vec!{};
            return;
        } else if line.starts_with("fertilizer-to-water") {
            maps.push(current_map.clone());
            current_map = vec!{};
            return;
        } else if line.starts_with("water-to-light") {
            maps.push(current_map.clone());
            current_map = vec!{};
            return;
        } else if line.starts_with("light-to-temperature") {
            maps.push(current_map.clone());
            current_map = vec!{};
            return;
        } else if line.starts_with("temperature-to-humidity") {
            maps.push(current_map.clone());
            current_map = vec!{};
            return;
        } else if line.starts_with("humidity-to-location") {
            maps.push(current_map.clone());
            current_map = vec!{};
            return;
        }

        // parse numbers
        let nums: Vec<usize> = line.split_whitespace().map(|n| n.parse::<usize>().unwrap()).collect();
        let destination_start = nums[0];
        let destination_end = nums[0] + nums[2];
        let source_start = nums[1];
        let source_end = nums[1] + nums[2];
        current_map.push((source_start, source_end, destination_start, destination_end));
        // println!("{}-{} {}-{}", source_start, source_end, destination_start, destination_end);
    });
    maps.push(current_map.clone());

    (seeds, maps)
}

fn part_a(input: String) -> usize {
    let (seeds, maps) = parse_input(input);

    seeds.iter().map(|seed| {
        let mut ptr = *seed;
        for map in &maps {
            for (source_start, source_end, destination_start, _destination_end) in map {
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

fn part_b(input: String) -> usize {
    let (seeds, maps) = parse_input(input);

    seeds.into_iter().tuples().map(|(seed_start, seed_range)| {
        (seed_start..seed_start + seed_range).map(|seed| {
            let mut ptr = seed;
            for map in &maps {
                for (source_start, source_end, destination_start, _destination_end) in map {
                    if *source_start <= ptr && ptr < *source_end {
                        ptr = ptr + (destination_start - source_start);
                        break;
                    }
                }
                // let the seed be the same
            }

            ptr
        }).min().unwrap()
    }).min().unwrap()
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
        assert_eq!(0, part_b(fs::read_to_string("input/2023/day05/input.txt").unwrap()));
    }
}
