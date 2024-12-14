use std::fs;

pub(crate) fn day15() {
    println!("{}", part_a(fs::read_to_string("input/2024/day15/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2024/day15/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    let (sm, sd) = input.split_once("\n\n").unwrap();
    let mut map: Vec<Vec<char>> = sm.lines().map(|line| line.chars().collect()).collect();
    let instructions: Vec<char> = sd.lines().map(|line| line.chars().collect::<Vec<char>>()).flatten().collect();
    let mut start = (0, 0);
    'outer: for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == '@' {
                start = (i, j);
                break 'outer;
            }
        }
    }

    let mut robot_pos = start;
    'outer: for c in instructions {
        match c {
            '^' => {
                if map[robot_pos.0 - 1][robot_pos.1] == '#' { continue; }
                if map[robot_pos.0 - 1][robot_pos.1] == '.' {
                    map[robot_pos.0][robot_pos.1] = '.';
                    map[robot_pos.0 - 1][robot_pos.1] = '@';
                    robot_pos = (robot_pos.0 - 1, robot_pos.1);
                    continue;
                }

                let mut x = robot_pos.0 - 1;
                let y = robot_pos.1;
                while x > 0 && map[x][y] == 'O' {
                    x -= 1;
                    if map[x][y] == '#' { continue 'outer; }
                }

                map[robot_pos.0][y] = '.';
                map[robot_pos.0 - 1][y] = '@';
                map[x][y] = 'O';

                robot_pos = (robot_pos.0 - 1, y);
            },
            '>' => {
                if map[robot_pos.0][robot_pos.1 + 1] == '#' { continue; }
                if map[robot_pos.0][robot_pos.1 + 1] == '.' {
                    map[robot_pos.0][robot_pos.1] = '.';
                    map[robot_pos.0][robot_pos.1 + 1] = '@';
                    robot_pos = (robot_pos.0, robot_pos.1 + 1);
                    continue;
                }

                let x = robot_pos.0;
                let mut y = robot_pos.1 + 1;
                while y < map[0].len() - 1 && map[x][y] == 'O' {
                    y += 1;
                    if map[x][y] == '#' { continue 'outer; }
                }

                map[x][robot_pos.1] = '.';
                map[x][robot_pos.1 + 1] = '@';
                map[x][y] = 'O';

                robot_pos = (x, robot_pos.1 + 1);
            },
            'v' => {
                if map[robot_pos.0 + 1][robot_pos.1] == '#' { continue; }
                if map[robot_pos.0 + 1][robot_pos.1] == '.' {
                    map[robot_pos.0][robot_pos.1] = '.';
                    map[robot_pos.0 + 1][robot_pos.1] = '@';
                    robot_pos = (robot_pos.0 + 1, robot_pos.1);
                    continue;
                }

                let mut x = robot_pos.0 + 1;
                let y = robot_pos.1;
                while x < map.len() - 1 && map[x][y] == 'O' {
                    x += 1;
                    if map[x][y] == '#' { continue 'outer; }
                }

                map[robot_pos.0][y] = '.';
                map[robot_pos.0 + 1][y] = '@';
                map[x][y] = 'O';

                robot_pos = (robot_pos.0 + 1, y);
            },
            '<' => {
                if map[robot_pos.0][robot_pos.1 - 1] == '#' { continue; }
                if map[robot_pos.0][robot_pos.1 - 1] == '.' {
                    map[robot_pos.0][robot_pos.1] = '.';
                    map[robot_pos.0][robot_pos.1 - 1] = '@';
                    robot_pos = (robot_pos.0, robot_pos.1 - 1);
                    continue;
                }

                let x = robot_pos.0;
                let mut y = robot_pos.1 - 1;
                while y > 0 && map[x][y] == 'O' {
                    y -= 1;
                    if map[x][y] == '#' { continue 'outer; }
                }

                map[x][robot_pos.1] = '.';
                map[x][robot_pos.1 - 1] = '@';
                map[x][y] = 'O';

                robot_pos = (x, robot_pos.1 - 1);
            },
            _ => panic!("{}", c)
        }
    }

    let mut ans = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 'O' {
                ans += 100 * i + j;
            }
        }
    }
    ans
}

fn part_b(input: String) -> usize {
    let (sm, sd) = input.split_once("\n\n").unwrap();
    let initial_map: Vec<Vec<char>> = sm.lines().map(|line| line.chars().collect()).collect();
    let instructions: Vec<char> = sd.lines().map(|line| line.chars().collect::<Vec<char>>()).flatten().collect();


    let mut map = vec![vec!['.'; initial_map[0].len() * 2]; initial_map.len()];
    for i in 0..initial_map.len() {
        for j in 0..initial_map[i].len() {
            match initial_map[i][j] {
                '#' => {
                    map[i][2 * j] = '#';
                    map[i][2 * j + 1] = '#';
                }
                'O' => {
                    map[i][2 * j] = '[';
                    map[i][2 * j + 1] = ']';
                }
                '.' => {
                    map[i][2 * j] = '.';
                    map[i][2 * j + 1] = '.';
                }
                '@' => {
                    map[i][2 * j] = '@';
                    map[i][2 * j + 1] = '.';
                }
                _ => panic!()
            }
        }
    }


    let mut start = (0, 0);
    'outer: for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == '@' {
                start = (i, j);
                break 'outer;
            }
        }
    }


    let mut robot_pos = start;
    'outer: for c in instructions {
        match c {
            '^' => {
                if map[robot_pos.0 - 1][robot_pos.1] == '#' { continue; }
                if map[robot_pos.0 - 1][robot_pos.1] == '.' {
                    map[robot_pos.0][robot_pos.1] = '.';
                    map[robot_pos.0 - 1][robot_pos.1] = '@';
                    robot_pos = (robot_pos.0 - 1, robot_pos.1);
                    continue;
                }

                let mut ranges = vec![(usize::MAX, 0); map.len()];
                if map[robot_pos.0 - 1][robot_pos.1] == ']' {
                    ranges[robot_pos.0 - 1] = (robot_pos.1 - 1, robot_pos.1);
                } else if map[robot_pos.0 - 1][robot_pos.1] == '[' {
                    ranges[robot_pos.0 - 1] = (robot_pos.1, robot_pos.1 + 1);
                }

                let mut x = robot_pos.0 - 1;
                loop {
                    if ranges[x] == (usize::MAX, 0) {
                        for y in ranges[x + 1].0..=ranges[x + 1].1 {
                            if map[x][y] == '#' { continue 'outer; }
                        }
                        break;
                    }
                    for y in ranges[x].0..=ranges[x].1 {
                        if map[x][y] == '#' { continue 'outer; }
                        if map[x - 1][y] == '[' {
                            ranges[x - 1] = (ranges[x - 1].0.min(y), ranges[x].1.max(y + 1));
                        }
                        if map[x - 1][y] == ']' {
                            ranges[x - 1] = (ranges[x - 1].0.min(y - 1), ranges[x].1.max(y));
                        }
                    }
                    x -= 1;
                }

                ranges[robot_pos.0] = (robot_pos.1, robot_pos.1);
                for dx in x..=robot_pos.0 - 1 {
                    for dy in ranges[dx + 1].0..=ranges[dx + 1].1 {
                        map[dx][dy] = map[dx + 1][dy];
                        map[dx + 1][dy] = '.';
                    }
                }

                robot_pos = (robot_pos.0 - 1, robot_pos.1);
            },
            '>' => {
                if map[robot_pos.0][robot_pos.1 + 1] == '#' { continue; }
                if map[robot_pos.0][robot_pos.1 + 1] == '.' {
                    map[robot_pos.0][robot_pos.1] = '.';
                    map[robot_pos.0][robot_pos.1 + 1] = '@';
                    robot_pos = (robot_pos.0, robot_pos.1 + 1);
                    continue;
                }

                let x = robot_pos.0;
                let mut y = robot_pos.1 + 1;
                while y < map[0].len() - 1 && (map[x][y] == '[' || map[x][y] == ']') {
                    y += 1;
                    if map[x][y] == '#' { continue 'outer; }
                }

                for dy in (robot_pos.1..=y).rev() {
                    map[x][dy] = map[x][dy - 1];
                }
                map[x][robot_pos.1] = '.';
                map[x][robot_pos.1 + 1] = '@';

                robot_pos = (x, robot_pos.1 + 1);
            },
            'v' => {
                if map[robot_pos.0 + 1][robot_pos.1] == '#' { continue; }
                if map[robot_pos.0 + 1][robot_pos.1] == '.' {
                    map[robot_pos.0][robot_pos.1] = '.';
                    map[robot_pos.0 + 1][robot_pos.1] = '@';
                    robot_pos = (robot_pos.0 + 1, robot_pos.1);
                    continue;
                }

                let mut ranges = vec![(usize::MAX, 0); map.len()];
                if map[robot_pos.0 + 1][robot_pos.1] == ']' {
                    ranges[robot_pos.0 + 1] = (robot_pos.1 - 1, robot_pos.1);
                } else if map[robot_pos.0 + 1][robot_pos.1] == '[' {
                    ranges[robot_pos.0 + 1] = (robot_pos.1, robot_pos.1 + 1);
                }

                let mut x = robot_pos.0 + 1;
                while x < map.len() {
                    if ranges[x] == (usize::MAX, 0) {
                        for y in ranges[x - 1].0..=ranges[x - 1].1 {
                            if map[x][y] == '#' { continue 'outer; }
                        }
                        break;
                    }
                    for y in ranges[x].0..=ranges[x].1 {
                        if map[x][y] == '#' { continue 'outer; }
                        if map[x + 1][y] == '[' {
                            ranges[x + 1] = (ranges[x + 1].0.min(y), ranges[x].1.max(y + 1));
                        }
                        if map[x + 1][y] == ']' {
                            ranges[x + 1] = (ranges[x + 1].0.min(y - 1), ranges[x].1.max(y));
                        }
                    }
                    x += 1;
                }

                ranges[robot_pos.0] = (robot_pos.1, robot_pos.1);
                for dx in (robot_pos.0 + 1..=x).rev() {
                    for dy in ranges[dx - 1].0..=ranges[dx - 1].1 {
                        map[dx][dy] = map[dx - 1][dy];
                        map[dx - 1][dy] = '.';
                    }
                }

                robot_pos = (robot_pos.0 + 1, robot_pos.1);
            },
            '<' => {
                if map[robot_pos.0][robot_pos.1 - 1] == '#' { continue; }
                if map[robot_pos.0][robot_pos.1 - 1] == '.' {
                    map[robot_pos.0][robot_pos.1] = '.';
                    map[robot_pos.0][robot_pos.1 - 1] = '@';
                    robot_pos = (robot_pos.0, robot_pos.1 - 1);
                    continue;
                }

                let x = robot_pos.0;
                let mut y = robot_pos.1 - 1;
                while y > 0 && (map[x][y] == '[' || map[x][y] == ']') {
                    y -= 1;
                    if map[x][y] == '#' { continue 'outer; }
                }

                for dy in y..=robot_pos.1 - 1 {
                    map[x][dy] = map[x][dy + 1];
                }
                map[x][robot_pos.1] = '.';
                map[x][robot_pos.1 - 1] = '@';

                robot_pos = (x, robot_pos.1 - 1);
            },
            _ => panic!("{}", c)
        }
    }

    let mut ans = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == '[' {
                ans += 100 * i + j;
            }
        }
    }
    ans
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
