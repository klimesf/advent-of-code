use std::fs;

pub(crate) fn day06() {
    println!(
        "{}",
        part_a(fs::read_to_string("input/2015/day06/input.txt").unwrap())
    );
    println!(
        "{}",
        part_b(fs::read_to_string("input/2015/day06/input.txt").unwrap())
    );
}

fn part_a(input: String) -> usize {
    let mut lights = vec![vec! {false; 1000}; 1000];
    input.lines().for_each(|line| {
        if line.starts_with("turn off") {
            let (l, r) = line[9..].split_once(" through ").unwrap();
            let (ll, lr) = l.split_once(",").unwrap();
            let (rl, rr) = r.split_once(",").unwrap();

            for r in ll.parse::<usize>().unwrap()..=rl.parse::<usize>().unwrap() {
                for c in lr.parse::<usize>().unwrap()..=rr.parse::<usize>().unwrap() {
                    lights[r][c] = false;
                }
            }
        } else if line.starts_with("turn on") {
            let (l, r) = line[8..].split_once(" through ").unwrap();
            let (ll, lr) = l.split_once(",").unwrap();
            let (rl, rr) = r.split_once(",").unwrap();

            for r in ll.parse::<usize>().unwrap()..=rl.parse::<usize>().unwrap() {
                for c in lr.parse::<usize>().unwrap()..=rr.parse::<usize>().unwrap() {
                    lights[r][c] = true;
                }
            }
        } else if line.starts_with("toggle") {
            let (l, r) = line[7..].split_once(" through ").unwrap();
            let (ll, lr) = l.split_once(",").unwrap();
            let (rl, rr) = r.split_once(",").unwrap();

            for r in ll.parse::<usize>().unwrap()..=rl.parse::<usize>().unwrap() {
                for c in lr.parse::<usize>().unwrap()..=rr.parse::<usize>().unwrap() {
                    lights[r][c] = !lights[r][c];
                }
            }
        }
    });

    let mut ans = 0;
    for r in 0..lights.len() {
        for c in 0..lights.len() {
            if lights[r][c] {
                ans += 1
            }
        }
    }
    ans
}

fn part_b(input: String) -> i32 {
    let mut lights = vec![vec! {0; 1000}; 1000];
    input.lines().for_each(|line| {
        if line.starts_with("turn off") {
            let (l, r) = line[9..].split_once(" through ").unwrap();
            let (ll, lr) = l.split_once(",").unwrap();
            let (rl, rr) = r.split_once(",").unwrap();

            for r in ll.parse::<usize>().unwrap()..=rl.parse::<usize>().unwrap() {
                for c in lr.parse::<usize>().unwrap()..=rr.parse::<usize>().unwrap() {
                    lights[r][c] = 0.max(lights[r][c] - 1);
                }
            }
        } else if line.starts_with("turn on") {
            let (l, r) = line[8..].split_once(" through ").unwrap();
            let (ll, lr) = l.split_once(",").unwrap();
            let (rl, rr) = r.split_once(",").unwrap();

            for r in ll.parse::<usize>().unwrap()..=rl.parse::<usize>().unwrap() {
                for c in lr.parse::<usize>().unwrap()..=rr.parse::<usize>().unwrap() {
                    lights[r][c] += 1;
                }
            }
        } else if line.starts_with("toggle") {
            let (l, r) = line[7..].split_once(" through ").unwrap();
            let (ll, lr) = l.split_once(",").unwrap();
            let (rl, rr) = r.split_once(",").unwrap();

            for r in ll.parse::<usize>().unwrap()..=rl.parse::<usize>().unwrap() {
                for c in lr.parse::<usize>().unwrap()..=rr.parse::<usize>().unwrap() {
                    lights[r][c] += 2;
                }
            }
        }
    });

    let mut ans = 0;
    for r in 0..lights.len() {
        for c in 0..lights.len() {
            ans += lights[r][c]
        }
    }
    ans
}

#[cfg(test)]
mod day06_tests {
    use std::fs;

    use crate::y2015::day06::{part_a, part_b};

    #[test]
    fn input_works() {
        assert_eq!(
            377891,
            part_a(fs::read_to_string("input/2015/day06/input.txt").unwrap())
        );
        assert_eq!(
            14110788,
            part_b(fs::read_to_string("input/2015/day06/input.txt").unwrap())
        );
    }
}
