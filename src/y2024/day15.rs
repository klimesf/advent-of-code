use std::collections::HashSet;
use std::fs;
use crate::utils::grid::{Grid, DOWN, LEFT, P, RIGHT, UP};

pub(crate) fn day15() {
    println!("{}", part_a(fs::read_to_string("input/2024/day15/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2024/day15/input.txt").unwrap()));
}

fn part_a(input: String) -> i32 {
    let (sm, sd) = input.split_once("\n\n").unwrap();
    let mut map = Grid::parse(sm);
    let mut robot = map.find_first('@').unwrap();
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
            map[robot] = '.';
            robot = robot + dir;
        }
    }

    map.into_iter()
        .filter(|(_, _, c)| *c == 'O')
        .map(|(x, y, _)| 100 * x + y)
        .sum()
}

fn push(c: char, p: P, dir: P, map: &mut Grid<char>) -> bool {
    match map[p] {
        '#' => false,
        '.' => {
            map[p] = c;
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

    let mut map = Grid::new(initial_map.x_len, initial_map.y_len * 2, '.');
    for x in 0..initial_map.x_len {
        for y in 0..initial_map.y_len {
            match initial_map[(x, y)] {
                '#' => {
                    map[(x, 2 * y)] = '#';
                    map[(x, 2 * y + 1)] = '#';
                }
                'O' => {
                    map[(x, 2 * y)] = '[';
                    map[(x, 2 * y + 1)] = ']';
                }
                '.' => {
                    map[(x, 2 * y)] = '.';
                    map[(x, 2 * y + 1)] = '.';
                }
                '@' => {
                    map[(x, 2 * y)] = '@';
                    map[(x, 2 * y + 1)] = '.';
                }
                _ => panic!()
            }
        }
    }

    let mut robot = map.find_first('@').unwrap();
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
        .filter(|(_, _, c)| *c == '[')
        .map(|(x, y, _)| 100 * x + y)
        .sum()
}

fn find_obj(p: P, dir: P, map: &Grid<char>, visited: &mut HashSet<P>) -> Vec<(P, char)> {
    if !visited.insert(p) { return vec!(); } // Prevent visiting same nodes twice
    let mut ans = vec!();
    match map[p] {
        '@' => {
            ans.push((p, map[p]));
            ans.extend(find_obj(p + dir, dir, map, visited));
        }
        '.' => { }
        '#' => { }
        '[' => {
            ans.push((p, map[p]));
            if dir == LEFT || dir == RIGHT {
                ans.extend(find_obj(p + dir, dir, map, visited));
            } else {
                ans.extend(find_obj(p + dir, dir, map, visited));
                ans.extend(find_obj(p + RIGHT, dir, map, visited));
            }
        }
        ']' => {
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

fn check_move(obj: &Vec<(P, char)>, dir: P, map: &Grid<char>) -> bool {
    obj.iter().all(|(p, _)| map[*p + dir] != '#')
}

fn push_object(obj: &Vec<(P, char)>, dir: P, map: &mut Grid<char>) {
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
        .for_each(|(p, _)| map[*p] = '.');
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
