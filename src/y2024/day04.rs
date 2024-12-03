use std::fs;

pub(crate) fn day04() {
    println!("{}", part_a(fs::read_to_string("input/2024/day04/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2024/day04/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    let map = input.lines().map(|line| {
        line.chars().collect::<Vec<char>>()
    }).collect::<Vec<Vec<char>>>();

    let mut ans = 0;
    for x in 0..map.len() {
        for y in 0..map[0].len() {
            if map[x][y] != 'X' { continue; }

            // Right
            if y <= map[x].len() - 4
                && map[x][y + 1] == 'M'
                && map[x][y + 2] == 'A'
                && map[x][y + 3] == 'S' {
                ans += 1;
            }

            // Left
            if y >= 3
                && map[x][y - 1] == 'M'
                && map[x][y - 2] == 'A'
                && map[x][y - 3] == 'S' {
                ans += 1;
            }

            // Up
            if x >= 3
                && map[x - 1][y] == 'M'
                && map[x - 2][y] == 'A'
                && map[x - 3][y] == 'S' {
                ans += 1;
            }

            // Down
            if x <= map[x].len() - 4
                && map[x + 1][y] == 'M'
                && map[x + 2][y] == 'A'
                && map[x + 3][y] == 'S' {
                ans += 1;
            }

            // Right + Down
            if y <= map[x].len() - 4
                && x <= map[x].len() - 4
                && map[x + 1][y + 1] == 'M'
                && map[x + 2][y + 2] == 'A'
                && map[x + 3][y + 3] == 'S' {
                ans += 1;
            }

            // Right + Up
            if y <= map[x].len() - 4
                && x >= 3
                && map[x - 1][y + 1] == 'M'
                && map[x - 2][y + 2] == 'A'
                && map[x - 3][y + 3] == 'S' {
                ans += 1;
            }

            // Left + Down
            if y >= 3
                && x <= map[x].len() - 4
                && map[x + 1][y - 1] == 'M'
                && map[x + 2][y - 2] == 'A'
                && map[x + 3][y - 3] == 'S' {
                ans += 1;
            }

            // Left + Up
            if y >= 3
                && x >= 3
                && map[x - 1][y - 1] == 'M'
                && map[x - 2][y - 2] == 'A'
                && map[x - 3][y - 3] == 'S' {
                ans += 1;
            }

        }
    }

    ans
}

fn part_b(input: String) -> usize {
    let map = input.lines().map(|line| {
        line.chars().collect::<Vec<char>>()
    }).collect::<Vec<Vec<char>>>();

    let mut ans = 0;
    for x in 0..map.len() {
        for y in 0..map[0].len() {
            if map[x][y] != 'A' { continue; }
            if x < 1 || y < 1 || x > map[x].len() - 2 || y > map.len() - 2 { continue; }

            // M.S
            // .A.
            // M.S
            if map[x - 1][y - 1] == 'M' && map[x + 1][y + 1] == 'S'
                && map[x + 1][y - 1] == 'M' && map[x - 1][y + 1] == 'S' {
                ans += 1;
            }

            // S.S
            // .A.
            // M.M
            if map[x - 1][y - 1] == 'S' && map[x + 1][y + 1] == 'M'
                && map[x + 1][y - 1] == 'S' && map[x - 1][y + 1] == 'M' {
                ans += 1;
            }

            // S.M
            // .A.
            // S.M
            if map[x - 1][y - 1] == 'S' && map[x + 1][y + 1] == 'M'
                && map[x + 1][y - 1] == 'M' && map[x - 1][y + 1] == 'S' {
                ans += 1;
            }

            // M.S
            // .A.
            // M.S
            if map[x - 1][y - 1] == 'M' && map[x + 1][y + 1] == 'S'
                && map[x + 1][y - 1] == 'S' && map[x - 1][y + 1] == 'M' {
                ans += 1;
            }
        }
    }
    ans
}

#[cfg(test)]
mod day04_tests {
    use std::fs;

    use crate::y2024::day04::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(18, part_a(fs::read_to_string("input/2024/day04/test.txt").unwrap()));
        assert_eq!(9, part_b(fs::read_to_string("input/2024/day04/test.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(2532, part_a(fs::read_to_string("input/2024/day04/input.txt").unwrap()));
        assert_eq!(1941, part_b(fs::read_to_string("input/2024/day04/input.txt").unwrap()));
    }
}
