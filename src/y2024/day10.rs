use std::collections::HashSet;
use std::fs;

pub fn day10(print: fn(usize)) {
    let (part_a, part_b) = solve(fs::read_to_string("input/2024/day10/input.txt").unwrap());
    print(part_a);
    print(part_b);
}

fn solve(input: String) -> (usize, usize) {
    let map = input.lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect::<Vec<Vec<u32>>>();

    let mut ans_a = 0;
    let mut ans_b = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] != 0 { continue; }
            let mut targets = HashSet::new(); // Fort part A, we are counting unique 9s

            let mut stack: Vec<(usize, usize, u32)> = vec!(); // x, y, last

            if i > 0 { stack.push((i - 1, j, 0)); }
            if j > 0 { stack.push((i, j - 1, 0)); }
            if i < map.len() - 1 { stack.push((i + 1, j, 0)); }
            if j < map[i].len() - 1 { stack.push((i, j + 1, 0)); }

            while let Some((x, y, last)) = stack.pop() {
                if last + 1 != map[x][y] { continue; }
                if map[x][y] == 9 {
                    targets.insert((x, y));
                    ans_b += 1;
                    continue;
                }

                if x > 0 { stack.push((x - 1, y, map[x][y])); }
                if y > 0 { stack.push((x, y - 1, map[x][y])); }
                if x < map.len() - 1 { stack.push((x + 1, y, map[x][y])); }
                if y < map[0].len() - 1 { stack.push((x, y + 1, map[x][y])); }
            }

            ans_a += targets.len();
        }
    }

    (ans_a, ans_b)
}

#[cfg(test)]
mod day10_tests {
    use std::fs;

    use crate::y2024::day10::{solve};

    #[test]
    fn test_works() {
        assert_eq!((36, 81), solve(fs::read_to_string("input/2024/day10/test.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!((552, 1225), solve(fs::read_to_string("input/2024/day10/input.txt").unwrap()));
    }
}
