use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use std::collections::HashSet;
use std::fs;

const DIRS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

pub fn day06(print: fn(usize)) {
    let (part_a, part_b) = solve(fs::read_to_string("input/2024/day06/input.txt").unwrap());
    print(part_a);
    print(part_b);
}

fn solve(input: String) -> (usize, usize) {
    let map = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

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
    let mut dir = 0;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    loop {
        visited.insert((pos.0 as usize, pos.1 as usize));
        let next_pos = (pos.0 + DIRS[dir].0, pos.1 + DIRS[dir].1);
        if next_pos.0 < 0
            || next_pos.0 >= map.len() as i32
            || next_pos.1 < 0
            || next_pos.1 >= map[0].len() as i32
        {
            break;
        }
        if map[next_pos.0 as usize][next_pos.1 as usize] == '#' {
            dir = (dir + 1) % 4;
        } else {
            pos = next_pos;
        }
    }
    let ans_a = visited.len();

    // Parallelism speeds this up by a factor of ~3 on M1
    let ans_b = visited
        .par_iter()
        .map(|&(i, j)| {
            if map[i][j] == '^' {
                return 0;
            }

            let mut ans = 0;
            let mut pos = starting_pos.clone();
            let mut dir = 0;
            let mut visited_b: HashSet<(i32, i32, usize)> = HashSet::new();

            'outer: loop {
                if !visited_b.insert((pos.0, pos.1, dir)) {
                    ans += 1;
                    break;
                }
                loop {
                    let next_pos = (pos.0 + DIRS[dir].0, pos.1 + DIRS[dir].1);
                    if next_pos.0 < 0
                        || next_pos.0 >= map.len() as i32
                        || next_pos.1 < 0
                        || next_pos.1 >= map[0].len() as i32
                    {
                        break 'outer;
                    }
                    if map[next_pos.0 as usize][next_pos.1 as usize] == '#'
                        || (next_pos.0 as usize == i && next_pos.1 as usize == j)
                    {
                        // Replaced item
                        pos = (next_pos.0 - DIRS[dir].0, next_pos.1 - DIRS[dir].1);
                        dir = (dir + 1) % 4;
                        continue 'outer;
                    }
                    pos = next_pos;
                }
            }
            ans
        })
        .sum();

    (ans_a, ans_b)
}

#[cfg(test)]
mod day06_tests {
    use std::fs;

    use crate::y2024::day06::solve;

    #[test]
    fn test_works() {
        assert_eq!(
            (41, 6),
            solve(fs::read_to_string("input/2024/day06/test.txt").unwrap())
        );
    }

    #[test]
    fn input_works() {
        assert_eq!(
            (4722, 1602),
            solve(fs::read_to_string("input/2024/day06/input.txt").unwrap())
        );
    }
}
