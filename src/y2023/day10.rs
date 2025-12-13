use std::collections::{HashMap, HashSet};
use std::fs;

pub(crate) fn day10() {
    let ans = solve(fs::read_to_string("input/2023/day10/input.txt").unwrap());
    println!("{}", ans.0);
    println!("{}", ans.1);
}

fn solve(input: String) -> (usize, usize) {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut start = (0, 0);
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 'S' {
                start = (i, j);
                break;
            }
        }
    }

    let mut distances = HashMap::new();
    distances.insert(start, 0);

    let mut stack = vec![];
    stack.push((start.0, start.1, 0));

    while !stack.is_empty() {
        let (r, c, dist) = stack.pop().unwrap();
        distances.insert((r, c), dist);
        let curr = map[r][c];
        if (curr == 'S' || curr == '|' || curr == 'J' || curr == 'L')
            && r > 0
            && (map[r - 1][c] == '|' || map[r - 1][c] == '7' || map[r - 1][c] == 'F')
            && *distances.get(&(r - 1, c)).unwrap_or(&(dist + 2)) > dist + 1
        {
            stack.push((r - 1, c, dist + 1));
        } // up
        if (curr == 'S' || curr == '|' || curr == '7' || curr == 'F')
            && r < map.len() - 1
            && (map[r + 1][c] == '|' || map[r + 1][c] == 'J' || map[r + 1][c] == 'L')
            && *distances.get(&(r + 1, c)).unwrap_or(&(dist + 2)) > dist + 1
        {
            stack.push((r + 1, c, dist + 1));
        } // down
        if (curr == 'S' || curr == '-' || curr == 'J' || curr == '7')
            && c > 0
            && (map[r][c - 1] == '-' || map[r][c - 1] == 'L' || map[r][c - 1] == 'F')
            && *distances.get(&(r, c - 1)).unwrap_or(&(dist + 2)) > dist + 1
        {
            stack.push((r, c - 1, dist + 1));
        } // left
        if (curr == 'S' || curr == '-' || curr == 'F' || curr == 'L')
            && c < map[r].len() - 1
            && (map[r][c + 1] == '-' || map[r][c + 1] == '7' || map[r][c + 1] == 'J')
            && *distances.get(&(r, c + 1)).unwrap_or(&(dist + 2)) > dist + 1
        {
            stack.push((r, c + 1, dist + 1));
        } // right
    }

    let part_a = *distances.values().max().unwrap();

    let mut double_map = vec![];
    for i in 0..map.len() {
        let mut row_1 = vec![];
        let mut row_2 = vec![];
        for j in 0..map[i].len() {
            match map[i][j] {
                '.' => {
                    row_1.push('.');
                    row_1.push('.');
                    row_2.push('.');
                    row_2.push('.');
                }
                '-' => {
                    row_1.push('-');
                    row_1.push('-');
                    row_2.push('.');
                    row_2.push('.');
                }
                '|' => {
                    row_1.push('|');
                    row_1.push('.');
                    row_2.push('|');
                    row_2.push('.');
                }
                'J' => {
                    row_1.push('J');
                    row_1.push('.');
                    row_2.push('.');
                    row_2.push('.');
                }
                'L' => {
                    row_1.push('L');
                    row_1.push('-');
                    row_2.push('.');
                    row_2.push('.');
                }
                '7' => {
                    row_1.push('7');
                    row_1.push('.');
                    row_2.push('|');
                    row_2.push('.');
                }
                'F' => {
                    row_1.push('F');
                    row_1.push('-');
                    row_2.push('|');
                    row_2.push('.');
                }
                'S' => {
                    row_1.push('S');
                    row_1.push('S');
                    row_2.push('S');
                    row_2.push('S');
                }
                c => panic!("Unknown {}", c),
            }
        }
        double_map.push(row_1);
        double_map.push(row_2);
    }

    let mut reachable = HashSet::new();
    let mut visited = HashSet::new();
    let mut stack = vec![];

    for r in 0..double_map.len() {
        for c in 0..double_map[r].len() {
            if r == 0 || r == double_map.len() - 1 || c == 0 || c == double_map[r].len() - 1 {
                stack.push((r, c));
            }
        }
    }

    while !stack.is_empty() {
        let pos = stack.pop().unwrap();
        if visited.contains(&pos) {
            continue;
        }
        visited.insert(pos);

        let (r, c) = pos;
        if double_map[r][c] != '.' && distances.contains_key(&(r / 2, c / 2)) {
            continue;
        }

        if r % 2 == 0 && c % 2 == 0 {
            reachable.insert((r / 2, c / 2));
        }

        if r > 0 {
            stack.push((r - 1, c))
        }
        if r < double_map.len() - 1 {
            stack.push((r + 1, c))
        }
        if c > 0 {
            stack.push((r, c - 1))
        }
        if c < double_map[r].len() - 1 {
            stack.push((r, c + 1))
        }
    }

    let part_b = (map.len() * map[0].len()) - reachable.len() - distances.keys().len();

    (part_a, part_b)
}

#[cfg(test)]
mod day10_tests {
    use std::fs;

    use crate::y2023::day10::solve;

    #[test]
    fn test_works() {
        assert_eq!(
            8,
            solve(fs::read_to_string("input/2023/day10/test.txt").unwrap()).0
        );
        assert_eq!(
            4,
            solve(fs::read_to_string("input/2023/day10/test_b.txt").unwrap()).1
        );
        assert_eq!(
            8,
            solve(fs::read_to_string("input/2023/day10/test_c.txt").unwrap()).1
        );
        assert_eq!(
            10,
            solve(fs::read_to_string("input/2023/day10/test_d.txt").unwrap()).1
        );
    }

    #[test]
    fn input_works() {
        assert_eq!(
            (6820, 337),
            solve(fs::read_to_string("input/2023/day10/input.txt").unwrap())
        );
    }
}
