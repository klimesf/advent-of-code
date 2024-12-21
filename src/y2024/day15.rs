use std::collections::HashSet;
use std::fs;
use crate::utils::grid::{Grid, DOWN, LEFT, P, RIGHT, UP};

pub fn day15(print: fn(i32)) {
    print(part_a(fs::read_to_string("input/2024/day15/input.txt").unwrap()));
    print(part_b(fs::read_to_string("input/2024/day15/input.txt").unwrap()));
}

fn part_a(input: String) -> i32 {
    let (sm, sd) = input.split_once("\n\n").unwrap();
    let mut map = Grid::parse(sm);
    let mut robot = map.find_first(b'@').unwrap();
    let instructions: Vec<char> = sd.lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .flatten().collect();

    for inst in instructions {
        let dir = match inst {
            '^' => UP,
            '>' => RIGHT,
            'v' => DOWN,
            '<' => LEFT,
            _ => panic!(),
        };
        if push(map[robot], robot + dir, dir, &mut map) {
            map[robot] = b'.';
            robot = robot + dir;
        }
    }

    map.into_iter()
        .filter(|(_, _, c)| *c == b'O')
        .map(|(x, y, _)| 100 * x + y)
        .sum()
}

fn push(c: u8, p: P, dir: P, map: &mut Grid<u8>) -> bool {
    match map[p] {
        b'#' => false,
        b'.' => {
            map[p] = c as u8;
            true
        }
        _ => {
            if push(map[p], p + dir, dir, map) {
                map[p] = c;
                true
            } else { false }
        }
    }
}

fn part_b(input: String) -> i32 {
    let (sm, sd) = input.split_once("\n\n").unwrap();
    let initial_map = Grid::parse(sm);
    let instructions: Vec<char> = sd.lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .flatten().collect();

    let mut map = Grid::new(initial_map.x_len, initial_map.y_len * 2, b'.');
    for x in 0..initial_map.x_len {
        for y in 0..initial_map.y_len {
            match initial_map[(x, y)] {
                b'#' => {
                    map[(x, 2 * y)] = b'#';
                    map[(x, 2 * y + 1)] = b'#';
                }
                b'O' => {
                    map[(x, 2 * y)] = b'[';
                    map[(x, 2 * y + 1)] = b']';
                }
                b'.' => {
                    map[(x, 2 * y)] = b'.';
                    map[(x, 2 * y + 1)] = b'.';
                }
                b'@' => {
                    map[(x, 2 * y)] = b'@';
                    map[(x, 2 * y + 1)] = b'.';
                }
                _ => panic!()
            }
        }
    }

    let mut robot = map.find_first(b'@').unwrap();
    for inst in instructions {
        let dir = match inst {
            '^' => UP,
            '>' => RIGHT,
            'v' => DOWN,
            '<' => LEFT,
            _ => panic!(),
        };

        let mut visited = HashSet::new();
        let object = find_obj(robot, dir, &map, &mut visited);
        if check_move(&object, dir, &map) {
            push_object(&object, dir, &mut map);
            robot = robot + dir;
        }
    }

    map.into_iter()
        .filter(|(_, _, c)| *c == b'[')
        .map(|(x, y, _)| 100 * x + y)
        .sum()
}

fn find_obj(p: P, dir: P, map: &Grid<u8>, visited: &mut HashSet<P>) -> Vec<(P, u8)> {
    if !visited.insert(p) { return vec!(); } // Prevent visiting same nodes twice
    let mut ans = vec!();
    match map[p] {
        b'@' => {
            ans.push((p, map[p]));
            ans.extend(find_obj(p + dir, dir, map, visited));
        }
        b'.' => { }
        b'#' => { }
        b'[' => {
            ans.push((p, map[p]));
            if dir == LEFT || dir == RIGHT {
                ans.extend(find_obj(p + dir, dir, map, visited));
            } else {
                ans.extend(find_obj(p + dir, dir, map, visited));
                ans.extend(find_obj(p + RIGHT, dir, map, visited));
            }
        }
        b']' => {
            ans.push((p, map[p]));
            if dir == LEFT || dir == RIGHT {
                ans.extend(find_obj(p + dir, dir, map, visited));
            } else {
                ans.extend(find_obj(p + dir, dir, map, visited));
                ans.extend(find_obj(p + LEFT, dir, map, visited));
            }
        }
        _ => panic!()
    }
    ans
}

fn check_move(obj: &Vec<(P, u8)>, dir: P, map: &Grid<u8>) -> bool {
    obj.iter().all(|(p, _)| map[*p + dir] != b'#')
}

fn push_object(obj: &Vec<(P, u8)>, dir: P, map: &mut Grid<u8>) {
    // Push to new positions
    obj.iter().rev().for_each(|(p, c)| {
        map[*p + dir] = *c;
    });

    // Cleanup
    let new_positions = obj.iter()
        .map(|(p, _)| *p + dir)
        .collect::<HashSet<P>>();
    obj.iter()
        .filter(|(p, _)| !new_positions.contains(p))
        .for_each(|(p, _)| map[*p] = b'.');
}

#[cfg(test)]
mod day15_tests {
    use std::fs;

    use crate::y2024::day15::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(2028, part_a(fs::read_to_string("input/2024/day15/test.txt").unwrap()));
        assert_eq!(10092, part_a(fs::read_to_string("input/2024/day15/test_2.txt").unwrap()));
        assert_eq!(618, part_b(fs::read_to_string("input/2024/day15/test_3.txt").unwrap()));
        assert_eq!(9021, part_b(fs::read_to_string("input/2024/day15/test_2.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(1492518, part_a(fs::read_to_string("input/2024/day15/input.txt").unwrap()));
        assert_eq!(1512860, part_b(fs::read_to_string("input/2024/day15/input.txt").unwrap()));
    }
}
