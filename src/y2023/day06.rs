use std::{fs};
use itertools::Itertools;

pub(crate) fn day06() {
    println!("{}", part_a(fs::read_to_string("input/2023/day06/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2023/day06/input.txt").unwrap()));
}

fn part_a(input: String) -> i32 {
    let (time_str, distance_str) = input.split_once("\n").unwrap();

    let times: Vec<i32> = time_str.split_whitespace().enumerate()
        .filter(|(i, _)| *i > 0)
        .map(|(_, s)| s.parse::<i32>().unwrap())
        .collect();

    let distances: Vec<i32> = distance_str.split_whitespace().enumerate()
        .filter(|(i, _)| *i > 0)
        .map(|(_, s)| s.parse::<i32>().unwrap())
        .collect();

    (0..times.len()).map(|i| {
        let time = times[i];
        let record = distances[i];

        let mut ans = 0;
        for t in 0..time {
            let travel_time = time - t;
            let travel_speed = t;

            if travel_time * travel_speed > record {
                ans += 1
            }
        }

        ans
    }).product()
}

fn part_b(input: String) -> u64 {
    let (time_str, distance_str) = input.split_once("\n").unwrap();

    let time = time_str.split_whitespace().enumerate()
        .filter(|(i, _)| *i > 0)
        .map(|(_, s)| s)
        .into_iter().join("")
        .parse::<i64>().unwrap();

    let record = distance_str.split_whitespace().enumerate()
        .filter(|(i, _)| *i > 0)
        .map(|(_, s)| s)
        .into_iter().join("")
        .parse::<i64>().unwrap();

    let mut ans = 0;
    for t in 0..time {
        let travel_time = time - t;
        let travel_speed = t;

        if travel_time * travel_speed > record {
            ans += 1
        }
    }
    ans
}

#[cfg(test)]
mod day06_tests {
    use std::fs;

    use crate::y2023::day06::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(288, part_a(fs::read_to_string("input/2023/day06/test.txt").unwrap()));
        assert_eq!(71503, part_b(fs::read_to_string("input/2023/day06/test.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(227850, part_a(fs::read_to_string("input/2023/day06/input.txt").unwrap()));
        assert_eq!(42948149, part_b(fs::read_to_string("input/2023/day06/input.txt").unwrap()));
    }
}
