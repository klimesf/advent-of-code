use std::collections::HashSet;
use std::fs;

pub(crate) fn day06() {
    let (part_a, part_b) = solve(fs::read_to_string("input/2024/day06/input.txt").unwrap());
    println!("{}", part_a);
    println!("{}", part_b);
}

fn solve(input: String) -> (usize, usize) {
    let mut map = input.lines().map(|line| {
        line.chars().collect()
    }).collect::<Vec<Vec<char>>>();

    let mut starting_pos: (i32, i32) = (0, 0);
    for i in 0..map[0].len() {
        for j in 0..map.len() {
            if map[i][j] == '^' {
                starting_pos = (i as i32, j as i32);
                break;
            }
        }
    }

    let mut pos = starting_pos.clone();
    let mut dir = (-1, 0);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    loop {
        visited.insert((pos.0 as usize, pos.1 as usize));
        let mut next_pos = (pos.0 + dir.0, pos.1 + dir.1);
        if next_pos.0 < 0 || next_pos.0 >= map.len() as i32
            || next_pos.1 < 0 || next_pos.1 >= map[0].len() as i32 {
            break;
        }
        loop {
            if map[next_pos.0 as usize][next_pos.1 as usize] == '#' {
                dir = rotate_right(&dir);
                next_pos = (pos.0 + dir.0, pos.1 + dir.1);
            } else { break }
        }
        pos = next_pos;
    }
    let ans_a = visited.len();

    let mut ans_b = 0;
    for (i, j) in visited {
        if map[i][j] == '^' { continue; }
        map[i][j] = '#';

        let mut pos = starting_pos.clone();
        let mut dir = (-1, 0);
        let mut visited_b: HashSet<(i32, i32, i32, i32)> = HashSet::new();

        loop {
            if !visited_b.insert((pos.0, pos.1, dir.0, dir.1)) {
                ans_b += 1;
                break;
            }
            let mut next_pos = (pos.0 + dir.0, pos.1 + dir.1);
            if next_pos.0 < 0 || next_pos.0 >= map.len() as i32
                || next_pos.1 < 0 || next_pos.1 >= map[0].len() as i32 {
                break;
            }
            loop {
                if map[next_pos.0 as usize][next_pos.1 as usize] == '#' {
                    dir = rotate_right(&dir);
                    next_pos = (pos.0 + dir.0, pos.1 + dir.1);
                } else { break }
            }
            pos = next_pos;
        }

        // clean
        map[i][j] = '.';
    }

    (ans_a, ans_b)
}

fn rotate_right(dir: &(i32, i32)) -> (i32, i32) {
    match dir {
        (-1, 0) => { (0, 1) }
        (0, 1) => { (1, 0) }
        (1, 0) => { (0, -1) }
        (0, -1) => { (-1, 0) }
        _ => panic!()
    }
}

#[cfg(test)]
mod day06_tests {
    use std::fs;

    use crate::y2024::day06::{solve};

    #[test]
    fn test_works() {
        assert_eq!((41, 6), solve(fs::read_to_string("input/2024/day06/test.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!((4722, 1602), solve(fs::read_to_string("input/2024/day06/input.txt").unwrap()));
    }
}
