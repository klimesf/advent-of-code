use itertools::Itertools;
use std::fs;

pub(crate) fn day12() {
    println!(
        "{}",
        part_a(fs::read_to_string("input/2020/day12/input.txt").unwrap())
    );
    println!(
        "{}",
        part_b(fs::read_to_string("input/2020/day12/input.txt").unwrap())
    );
}

fn part_a(input: String) -> isize {
    let mut pos = (0, 0);
    let mut dir = 90;
    input.lines().for_each(|line| {
        let chars = line.chars().collect::<Vec<char>>();
        let action = chars[0];
        let value = chars[1..].iter().join("").parse::<isize>().unwrap();
        match action {
            'N' => pos = (pos.0, pos.1 - value),
            'E' => pos = (pos.0 + value, pos.1),
            'S' => pos = (pos.0, pos.1 + value),
            'W' => pos = (pos.0 - value, pos.1),
            'L' => {
                if value % 90 != 0 {
                    panic!("rotate by non-divisible by 90: {}", value)
                }
                dir = (dir - value).rem_euclid(360);
            }
            'R' => {
                if value % 90 != 0 {
                    panic!("rotate by non-divisible by 90: {}", value)
                }
                dir = (dir + value).rem_euclid(360);
            }
            'F' => match dir {
                0 => pos = (pos.0, pos.1 - value),
                90 => pos = (pos.0 + value, pos.1),
                180 => pos = (pos.0, pos.1 + value),
                270 => pos = (pos.0 - value, pos.1),
                _ => panic!("unknown dir: {}", dir),
            },
            _ => panic!("Unknown action: {}", action),
        }
    });
    pos.0.abs() + pos.1.abs()
}

fn part_b(input: String) -> isize {
    let mut waypoint = (10, -1);
    let mut pos = (0, 0);
    input.lines().for_each(|line| {
        let chars = line.chars().collect::<Vec<char>>();
        let action = chars[0];
        let value = chars[1..].iter().join("").parse::<isize>().unwrap();
        match action {
            'N' => waypoint = (waypoint.0, waypoint.1 - value),
            'E' => waypoint = (waypoint.0 + value, waypoint.1),
            'S' => waypoint = (waypoint.0, waypoint.1 + value),
            'W' => waypoint = (waypoint.0 - value, waypoint.1),
            'L' => match value {
                90 => waypoint = (waypoint.1, -waypoint.0),
                180 => waypoint = (-waypoint.0, -waypoint.1),
                270 => waypoint = (-waypoint.1, waypoint.0),
                _ => panic!("unknown value: {}", value),
            },
            'R' => match value {
                90 => waypoint = (-waypoint.1, waypoint.0),
                180 => waypoint = (-waypoint.0, -waypoint.1),
                270 => waypoint = (waypoint.1, -waypoint.0),
                _ => panic!("unknown value: {}", value),
            },
            'F' => pos = (pos.0 + value * waypoint.0, pos.1 + value * waypoint.1),
            _ => panic!("Unknown action: {}", action),
        }
    });
    pos.0.abs() + pos.1.abs()
}

#[cfg(test)]
mod day12_tests {
    use std::fs;

    use crate::y2020::day12::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(
            25,
            part_a(fs::read_to_string("input/2020/day12/test.txt").unwrap())
        );
        assert_eq!(
            286,
            part_b(fs::read_to_string("input/2020/day12/test.txt").unwrap())
        );
    }

    #[test]
    fn input_works() {
        assert_eq!(
            1106,
            part_a(fs::read_to_string("input/2020/day12/input.txt").unwrap())
        );
        assert_eq!(
            107281,
            part_b(fs::read_to_string("input/2020/day12/input.txt").unwrap())
        );
    }
}
