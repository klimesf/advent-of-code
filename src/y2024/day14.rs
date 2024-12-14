use std::collections::HashMap;
use std::fs;
use regex::Regex;
use crate::utils::toolbox::parse_i32;

pub(crate) fn day14() {
    println!("{}", part_a(fs::read_to_string("input/2024/day14/input.txt").unwrap(), 101, 103));
    println!("{}", part_b(fs::read_to_string("input/2024/day14/input.txt").unwrap(), 101, 103));
}

fn part_a(input: String, max_x: i32, max_y: i32) -> i32 {
    let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
    let quadrants = input
        .lines()
        .map(|line| re.captures(line).unwrap() )
        .map(|caps| (
            parse_i32(caps.get(1)),
            parse_i32(caps.get(2)),
            parse_i32(caps.get(3)),
            parse_i32(caps.get(4))
        ))
        .map(|(px, py, vx, vy)| (
            (px + 100 * vx).rem_euclid(max_x),
            (py + 100 * vy).rem_euclid(max_y)
        ))
        .map(|(x, y)| (
            (max_x / 2 - x).signum(),
            (max_y / 2 - y).signum()
        ))
        .fold(HashMap::new(), |mut acc, (sx, sy)| {
            *acc.entry((sx, sy)).or_insert(0) += 1;
            acc
        });
    quadrants[&(-1, -1)] * quadrants[&(-1, 1)] * quadrants[&(1, -1)] * quadrants[&(1, 1)]
}

fn part_b(input: String, max_x: i32, max_y: i32) -> usize {
    let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
    let mut robots: Vec<(i32, i32, i32, i32)> = input.lines().map(|line| {
        let caps = re.captures(line).unwrap();
        let px = parse_i32(caps.get(1));
        let py = parse_i32(caps.get(2));
        let vx = parse_i32(caps.get(3));
        let vy = parse_i32(caps.get(4));
        (px, py, vx, vy)
    }).collect();

    let mut ans = 0;
    let mut avg_variance_x = 0;
    let mut avg_variance_y = 0;
    loop {
        for i in 0..robots.len() {
            let (px, py, vx, vy) = robots[i];
            let new_px = (px + vx).rem_euclid(max_x);
            let new_py = (py + vy).rem_euclid(max_y);
            robots[i] = (new_px, new_py, vx, vy);
        }
        ans += 1;

        let avg_x = robots.iter()
            .map(|(x, _, _, _)| x)
            .sum::<i32>() / robots.len() as i32;
        let avg_y = robots.iter()
            .map(|(_, y, _, _)| y)
            .sum::<i32>() / robots.len() as i32;
        let variance_x = robots.iter()
            .map(|(x, _, _, _)| (x - avg_x).pow(2))
            .sum::<i32>() / robots.len() as i32;
        let variance_y = robots.iter()
            .map(|(_, y, _, _)| (y - avg_y).pow(2))
            .sum::<i32>() / robots.len() as i32;

        if variance_x <= avg_variance_x / 2 && variance_y <= avg_variance_y / 2 {
            return ans;
        }

        avg_variance_x = (avg_variance_x * (ans as i32 - 1) + variance_x) / ans as i32;
        avg_variance_y = (avg_variance_y * (ans as i32 - 1) + variance_y) / ans as i32;
    }
}

#[cfg(test)]
mod day14_tests {
    use std::fs;

    use crate::y2024::day14::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(12, part_a(fs::read_to_string("input/2024/day14/test.txt").unwrap(), 11, 7));
    }

    #[test]
    fn input_works() {
        assert_eq!(222901875, part_a(fs::read_to_string("input/2024/day14/input.txt").unwrap(), 101, 103));
        assert_eq!(6243, part_b(fs::read_to_string("input/2024/day14/input.txt").unwrap(), 101, 103));
    }
}
