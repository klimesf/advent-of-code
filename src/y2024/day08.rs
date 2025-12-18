use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::ops::{Add, Sub};

pub fn day08(print: fn(usize)) {
    print(part_a(fs::read_to_string("input/2024/day08/input.txt").unwrap()));
    print(part_b(fs::read_to_string("input/2024/day08/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    let map = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let mut antennae: HashMap<char, Vec<Point>> = HashMap::new();
    map.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().filter(|(_, c)| **c != '.').for_each(|(j, c)| {
            antennae.entry(*c).or_insert(vec![]).push(Point(i as i32, j as i32));
        })
    });

    let mut antinodes: HashSet<Point> = HashSet::new();
    antennae.values().for_each(|&ref positions| {
        positions.iter().tuple_combinations().for_each(|(a1, a2)| {
            let vec = a2 - a1;

            let an1 = a2 + &vec;
            if an1.in_bounds(map.len(), map[0].len()) {
                antinodes.insert(an1);
            }

            let an2 = a1 - &vec;
            if an2.in_bounds(map.len(), map[0].len()) {
                antinodes.insert(an2);
            }
        });
    });
    antinodes.len()
}

fn part_b(input: String) -> usize {
    let map = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let mut antennae: HashMap<char, Vec<Point>> = HashMap::new();
    map.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().filter(|(_, c)| **c != '.').for_each(|(j, c)| {
            antennae.entry(*c).or_insert(vec![]).push(Point(i as i32, j as i32));
        })
    });

    let mut antinodes: HashSet<Point> = HashSet::new();
    antennae.values().for_each(|&ref positions| {
        positions.iter().tuple_combinations().for_each(|(a1, a2)| {
            let vec = a2 - a1;

            let mut inc = 0;
            loop {
                let an = a2 + &vec.mul(inc);
                if !an.in_bounds(map.len(), map[0].len()) {
                    break;
                }
                antinodes.insert(an);
                inc += 1;
            }

            let mut inc = 0;
            loop {
                let an = a2 - &vec.mul(inc);
                if !an.in_bounds(map.len(), map[0].len()) {
                    break;
                }
                antinodes.insert(an);
                inc += 1;
            }
        });
    });
    antinodes.len()
}

#[derive(Debug, PartialEq, Hash, Eq)]
struct Point(i32, i32);

impl Sub for &Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Add for &Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Point {
    fn in_bounds(&self, max_x: usize, max_y: usize) -> bool {
        self.0 >= 0 && self.0 < max_x as i32 && self.1 >= 0 && self.1 < max_y as i32
    }

    fn mul(&self, times: i32) -> Point {
        Point(self.0 * times, self.1 * times)
    }
}

#[cfg(test)]
mod day08_tests {
    use std::fs;

    use crate::y2024::day08::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(14, part_a(fs::read_to_string("input/2024/day08/test.txt").unwrap()));
        assert_eq!(34, part_b(fs::read_to_string("input/2024/day08/test.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(351, part_a(fs::read_to_string("input/2024/day08/input.txt").unwrap()));
        assert_eq!(1259, part_b(fs::read_to_string("input/2024/day08/input.txt").unwrap()));
    }
}
