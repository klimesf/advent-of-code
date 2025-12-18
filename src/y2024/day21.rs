use itertools::Itertools;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fs;

pub fn day21(print: fn(usize)) {
    let (part_a, part_b) = solve(fs::read_to_string("input/2024/day21/input.txt").unwrap());
    print(part_a);
    print(part_b);
}

fn solve(input: String) -> (usize, usize) {
    let codes: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    // +---+---+---+
    // | 7 | 8 | 9 |
    // +---+---+---+
    // | 4 | 5 | 6 |
    // +---+---+---+
    // | 1 | 2 | 3 |
    // +---+---+---+
    //     | 0 | A |
    //     +---+---+
    let numeric_keypad = [['7', '8', '9'], ['4', '5', '6'], ['1', '2', '3'], [' ', '0', 'A']];
    let arm_pos_num = |c: char| -> (usize, usize) {
        match c {
            '7' => (0, 0),
            '8' => (0, 1),
            '9' => (0, 2),
            '4' => (1, 0),
            '5' => (1, 1),
            '6' => (1, 2),
            '1' => (2, 0),
            '2' => (2, 1),
            '3' => (2, 2),
            '0' => (3, 1),
            'A' => (3, 2),
            _ => panic!(),
        }
    };

    // Pre-calculate shortest paths between each position on the keypad using Dijkstra
    let mut numeric_keypad_map: HashMap<(char, char), Vec<Vec<char>>> = HashMap::new();
    for from in ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A'] {
        for to in ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A'] {
            let mut stack = BinaryHeap::new();

            let start_arm_pos = arm_pos_num(from);
            stack.push(Pos {
                x: start_arm_pos.0,
                y: start_arm_pos.1,
                dist: 0,
                path: vec![],
            });
            let mut min = usize::MAX;

            let mut paths = vec![];
            while let Some(pos) = stack.pop() {
                if pos.dist > min {
                    break;
                }
                if numeric_keypad[pos.x][pos.y] == to {
                    paths.push(pos.path.clone());
                    min = pos.dist;
                    continue;
                }

                if pos.x > 0 && (pos.x - 1, pos.y) != (3, 0) {
                    let mut newpath = pos.path.clone();
                    newpath.push('^');
                    stack.push(Pos {
                        x: pos.x - 1,
                        y: pos.y,
                        dist: pos.dist + 1,
                        path: newpath,
                    });
                }
                if pos.x < 3 && (pos.x + 1, pos.y) != (3, 0) {
                    let mut newpath = pos.path.clone();
                    newpath.push('v');
                    stack.push(Pos {
                        x: pos.x + 1,
                        y: pos.y,
                        dist: pos.dist + 1,
                        path: newpath,
                    });
                }
                if pos.y > 0 && (pos.x, pos.y - 1) != (3, 0) {
                    let mut newpath = pos.path.clone();
                    newpath.push('<');
                    stack.push(Pos {
                        x: pos.x,
                        y: pos.y - 1,
                        dist: pos.dist + 1,
                        path: newpath,
                    });
                }
                if pos.y < 2 && (pos.x, pos.y + 1) != (3, 0) {
                    let mut newpath = pos.path.clone();
                    newpath.push('>');
                    stack.push(Pos {
                        x: pos.x,
                        y: pos.y + 1,
                        dist: pos.dist + 1,
                        path: newpath,
                    });
                }
            }
            numeric_keypad_map.insert((from, to), paths.iter().unique().map(|c| c.clone()).collect());
        }
    }

    //     +---+---+
    //     | ^ | A |
    // +---+---+---+
    // | < | v | > |
    // +---+---+---+
    let arrow_keypad = [[' ', '^', 'A'], ['<', 'v', '>']];
    let arm_pos_key = |c: char| -> (usize, usize) {
        match c {
            '^' => (0, 1),
            'A' => (0, 2),
            '<' => (1, 0),
            'v' => (1, 1),
            '>' => (1, 2),
            _ => panic!(),
        }
    };

    // Pre-calculate shortest paths between each position on the keypad using Dijkstra
    let mut arrow_keypad_map: HashMap<(char, char), Vec<Vec<char>>> = HashMap::new();
    for from in ['^', 'A', '<', 'v', '>'] {
        for to in ['^', 'A', '<', 'v', '>'] {
            let mut stack = BinaryHeap::new();

            let start_arm_pos = arm_pos_key(from);
            stack.push(Pos {
                x: start_arm_pos.0,
                y: start_arm_pos.1,
                dist: 0,
                path: vec![],
            });
            let mut min = usize::MAX;

            let mut paths = vec![];
            while let Some(pos) = stack.pop() {
                if pos.dist > min {
                    break;
                }
                if arrow_keypad[pos.x][pos.y] == to {
                    paths.push(pos.path.clone());
                    min = pos.dist;
                    continue;
                }

                if pos.x > 0 && (pos.x - 1, pos.y) != (0, 0) {
                    let mut newpath = pos.path.clone();
                    newpath.push('^');
                    stack.push(Pos {
                        x: pos.x - 1,
                        y: pos.y,
                        dist: pos.dist + 1,
                        path: newpath,
                    });
                }
                if pos.x < 1 && (pos.x + 1, pos.y) != (0, 0) {
                    let mut newpath = pos.path.clone();
                    newpath.push('v');
                    stack.push(Pos {
                        x: pos.x + 1,
                        y: pos.y,
                        dist: pos.dist + 1,
                        path: newpath,
                    });
                }
                if pos.y > 0 && (pos.x, pos.y - 1) != (0, 0) {
                    let mut newpath = pos.path.clone();
                    newpath.push('<');
                    stack.push(Pos {
                        x: pos.x,
                        y: pos.y - 1,
                        dist: pos.dist + 1,
                        path: newpath,
                    });
                }
                if pos.y < 2 && (pos.x, pos.y + 1) != (0, 0) {
                    let mut newpath = pos.path.clone();
                    newpath.push('>');
                    stack.push(Pos {
                        x: pos.x,
                        y: pos.y + 1,
                        dist: pos.dist + 1,
                        path: newpath,
                    });
                }
            }
            arrow_keypad_map.insert((from, to), paths.iter().unique().map(|c| c.clone()).collect());
        }
    }

    codes
        .par_iter()
        .map(|code| {
            let mut min_a = 0;
            let mut min_b = 0;
            let mut pos = 'A';
            let mut cache = HashMap::new();

            let mut shortest_numeric = |from: char, to: char, max_level: usize| -> usize {
                let mut min = usize::MAX;
                for path in numeric_keypad_map.get(&(from, to)).unwrap() {
                    let mut new_path = path.clone();
                    new_path.push('A');

                    let mut pos = 'A';
                    let mut ans = 0;
                    for c in new_path {
                        ans += shortest_arrow(pos, c, &arrow_keypad_map, 0, max_level, &mut cache);
                        pos = c;
                    }
                    min = ans.min(min);
                }
                min
            };

            for c in code {
                min_a += shortest_numeric(pos, *c, 2 - 2);
                min_b += shortest_numeric(pos, *c, 25 - 2);
                pos = *c;
            }

            // --- Calc ans
            let val: usize = (0..code.len() - 1)
                .rev()
                .map(|i| {
                    let digit = code[i].to_digit(10).unwrap() as usize;
                    digit * 10_usize.pow((code.len() - i - 2) as u32)
                })
                .sum();
            (val * min_a, val * min_b)
        })
        .reduce(|| (0, 0), |a, b| (a.0 + b.0, a.1 + b.1))
}

fn shortest_arrow(
    from: char,
    to: char,
    arrow_keypad_map: &HashMap<(char, char), Vec<Vec<char>>>,
    level: usize,
    max_level: usize,
    cache: &mut HashMap<(char, char, usize, usize), usize>,
) -> usize {
    if level > max_level {
        // All paths should be the same size, so just get the length of the first one, plus 'A'
        return arrow_keypad_map.get(&(from, to)).unwrap().first().unwrap().len() + 1;
    }

    if cache.contains_key(&(from, to, level, max_level)) {
        return cache[&(from, to, level, max_level)];
    }

    let mut min = usize::MAX;

    for path in arrow_keypad_map.get(&(from, to)).unwrap() {
        let mut new_path = path.clone();
        new_path.push('A');

        // The previous path will always end on 'A', and the initial position for each robot is also 'A'.
        // Thanks, Eric.
        let mut pos = 'A';
        let mut ans = 0;
        for c in new_path {
            ans += shortest_arrow(pos, c, &arrow_keypad_map, level + 1, max_level, cache);
            pos = c;
        }
        min = ans.min(min);
    }

    cache.insert((from, to, level, max_level), min);
    min
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Pos {
    x: usize,
    y: usize,
    dist: usize,
    path: Vec<char>,
}

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod day21_tests {
    use std::fs;

    use crate::y2024::day21::solve;

    #[test]
    fn test_works() {
        assert_eq!((68 * 29, 2379451789590), solve(fs::read_to_string("input/2024/day21/test.txt").unwrap()));
        assert_eq!((126384, 154115708116294), solve(fs::read_to_string("input/2024/day21/test_2.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!((202648, 248919739734728), solve(fs::read_to_string("input/2024/day21/input.txt").unwrap()));
    }
}
