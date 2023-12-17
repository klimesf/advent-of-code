use std::collections::HashSet;
use std::fs;
use itertools::Itertools;
use regex::{Regex};
use crate::utils::toolbox::parse_i32;

pub(crate) fn day18() {
    println!("{}", part_a(fs::read_to_string("input/2023/day18/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2023/day18/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    let re = Regex::new(r"^(U|D|R|L) (\d+) \((.+)\)$").unwrap();
    let mut pos: (i32, i32) = (0, 0);
    let mut rect: HashSet<(i32, i32)> = HashSet::new();
    let mut stack = vec!();
    let mut prev_dir = "X";

    input.lines().for_each(|line| {
        let caps = re.captures(line).unwrap();
        let dir = caps.get(1).unwrap().as_str();
        let num = parse_i32(caps.get(2));
        let _color = caps.get(3).unwrap().as_str();

        match dir {
            "U" => {
                for i in 1..=num {
                    rect.insert((pos.0 - i, pos.1));
                }
                pos = (pos.0 - num, pos.1);
            }
            "D" => {
                if prev_dir == "R" {
                    stack.push((pos.0 + 1, pos.1 - 1));
                }

                for i in 1..=num {
                    rect.insert((pos.0 + i, pos.1));
                }
                pos = (pos.0 + num, pos.1);
            }
            "R" => {
                for i in 1..=num {
                    rect.insert((pos.0, pos.1 + i));
                }
                pos = (pos.0, pos.1 + num);
            }
            "L" => {
                for i in 1..=num {
                    rect.insert((pos.0, pos.1 - i));
                }
                pos = (pos.0, pos.1 - num);
            }
            _ => panic!("Unknown dir {}", dir)
        }

        prev_dir = dir;
    });

    while let Some(pos) = stack.pop() {
        if !rect.insert(pos) { continue; }

        stack.push((pos.0 - 1, pos.1));
        stack.push((pos.0 + 1, pos.1));
        stack.push((pos.0, pos.1 - 1));
        stack.push((pos.0, pos.1 + 1));
    }

    rect.len()
}

fn part_b(input: String) -> i128 {
    let mut pos: (i128, i128) = (0, 0);
    let mut rect: Vec<(i128, i128)> = vec! {pos};

    let mut sides = 0;

    input.lines().for_each(|line| {
        let (_, hex) = line.split_once("#").unwrap();
        let num = i128::from_str_radix(hex[0..5].to_string().as_str(), 16).unwrap();
        sides += num;

        match hex[5..6].to_string().as_str() {
            // UP
            "3" => {
                pos = (pos.0 - num, pos.1);
                rect.push(pos);
            }
            // DOWN
            "1" => {
                pos = (pos.0 + num, pos.1);
                rect.push(pos);
            }
            // RIGHT
            "0" => {
                pos = (pos.0, pos.1 + num);
                rect.push(pos);
            }
            // LEFT
            "2" => {
                pos = (pos.0, pos.1 - num);
                rect.push(pos);
            }
            dir => panic!("Unknown dir {}", dir)
        }
    });

    // https://www.mathopenref.com/coordpolygonarea.html
    let mut ans = 0;
    rect.iter().tuple_windows().for_each(|((x1, y1), (x2, y2))| {
        ans += (x1 * y2) - (y1 * x2);
    });
    (-ans / 2) + (sides / 2) + 1 // Add (0,0)
}

#[cfg(test)]
mod day18_tests {
    use std::fs;

    use crate::y2023::day18::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(62, part_a(fs::read_to_string("input/2023/day18/test.txt").unwrap()));
        assert_eq!(952408144115, part_b(fs::read_to_string("input/2023/day18/test.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(33491, part_a(fs::read_to_string("input/2023/day18/input.txt").unwrap()));
        assert_eq!(87716969654406, part_b(fs::read_to_string("input/2023/day18/input.txt").unwrap()));
    }
}
