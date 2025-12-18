use itertools::Itertools;
use std::fs;

pub(crate) fn day06() {
    println!("{}", part_a(fs::read_to_string("input/2023/day06/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2023/day06/input.txt").unwrap()));
}

fn part_a(input: String) -> i32 {
    let (time_str, distance_str) = input.split_once("\n").unwrap();

    let times: Vec<i32> = time_str
        .split_whitespace()
        .enumerate()
        .filter(|(i, _)| *i > 0)
        .map(|(_, s)| s.parse::<i32>().unwrap())
        .collect();

    let distances: Vec<i32> = distance_str
        .split_whitespace()
        .enumerate()
        .filter(|(i, _)| *i > 0)
        .map(|(_, s)| s.parse::<i32>().unwrap())
        .collect();

    (0..times.len())
        .map(|i| {
            let time = times[i];
            let record = distances[i];

            // dist = x * (t - x)
            // dist = tx - x^2
            // x^2 - tx + dist = 0
            // x = (t +- sqrt(t^2 - 4 dist) / 2)

            let x1 = (time as f64 + ((time.pow(2) - 4 * record) as f64).sqrt()) / 2.0;
            let x2 = (time as f64 - ((time.pow(2) - 4 * record) as f64).sqrt()) / 2.0;

            (x1.ceil() - x2.floor()) as i32 - 1
        })
        .product()
}

fn part_b(input: String) -> i64 {
    let (time_str, distance_str) = input.split_once("\n").unwrap();

    let time = time_str
        .split_whitespace()
        .enumerate()
        .filter(|(i, _)| *i > 0)
        .map(|(_, s)| s)
        .into_iter()
        .join("")
        .parse::<i64>()
        .unwrap();

    let record = distance_str
        .split_whitespace()
        .enumerate()
        .filter(|(i, _)| *i > 0)
        .map(|(_, s)| s)
        .into_iter()
        .join("")
        .parse::<i64>()
        .unwrap();

    let x1 = (time as f64 + ((time.pow(2) - 4 * record) as f64).sqrt()) / 2.0;
    let x2 = (time as f64 - ((time.pow(2) - 4 * record) as f64).sqrt()) / 2.0;

    (x1.ceil() - x2.floor()) as i64 - 1
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
