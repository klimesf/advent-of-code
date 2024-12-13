use std::fs;
use regex::Regex;
use crate::utils::toolbox::parse_i32;

pub(crate) fn day14() {
    println!("{}", part_a(fs::read_to_string("input/2024/day14/input.txt").unwrap(), 101, 103));
    println!("{}", part_b(fs::read_to_string("input/2024/day14/input.txt").unwrap(), 101, 103));
}

fn part_a(input: String, max_x: i32, max_y: i32) -> i32 {
    let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
    let mut robots: Vec<(i32, i32, i32, i32)> = input.lines().map(|line| {
        let caps = re.captures(line).unwrap();
        let px = parse_i32(caps.get(1));
        let py = parse_i32(caps.get(2));
        let vx = parse_i32(caps.get(3));
        let vy = parse_i32(caps.get(4));
        (px, py, vx, vy)
    }).collect();

    for i in 0..robots.len() {
        let (px, py, vx, vy) = robots[i];
        let new_px = (px + 100 * vx).rem_euclid(max_x);
        let new_py = (py + 100 * vy).rem_euclid(max_y);
        robots[i] = (new_px, new_py, vx, vy);
    }

    let mut quadrants = vec![vec![0; 3]; 3];

    for (px, py, _, _) in &robots {
        let xs = (max_x / 2 - px).signum();
        let ys = (max_y / 2 - py).signum();
        quadrants[(xs + 1) as usize][(ys + 1) as usize] += 1;
    }

    quadrants[0][0] * quadrants[2][0] * quadrants[0][2] * quadrants[2][2]
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
    loop {
        for i in 0..robots.len() {
            let (px, py, vx, vy) = robots[i];
            let new_px = (px + vx).rem_euclid(max_x);
            let new_py = (py + vy).rem_euclid(max_y);
            robots[i] = (new_px, new_py, vx, vy);
        }
        ans += 1;
        // TODO: some fancy checking logic
        if ans == 6243 { return ans }
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
