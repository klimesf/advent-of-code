use std::fs;

pub(crate) fn day04(print: fn(usize)) {
    print(part_a(fs::read_to_string("input/2025/day04/input.txt").unwrap()));
    print(part_b(fs::read_to_string("input/2025/day04/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    count_removable(&map)
}

fn part_b(input: String) -> usize {
    let mut map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut new_map = map.clone();
    let mut ans = 0;
    while count_removable(&map) > 0 {
        for i in 0..map.len() {
            for j in 0..map[i].len() {
                if is_removable(&map, i, j) {
                    new_map[i][j] = '.';
                    ans += 1;
                }
            }
        }
        map = new_map.clone();
    }
    ans
}

fn count_removable(map: &Vec<Vec<char>>) -> usize {
    let mut ans = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if is_removable(&map, i, j) {
                ans += 1;
            }
        }
    }
    ans
}

fn is_removable(map: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    if map[i][j] == '.' { return false; }
    let mut adj = 0;
    if i > 0 && map[i - 1][j] == '@' { adj += 1; }
    if j > 0 && map[i][j - 1] == '@' { adj += 1; }
    if i < map.len() - 1 && map[i + 1][j] == '@' { adj += 1; }
    if j < map[0].len() - 1 && map[i][j + 1] == '@' { adj += 1; }
    if i > 0 && j > 0 && map[i - 1][j - 1] == '@' { adj += 1; }
    if i > 0 && j < map.len() - 1 && map[i - 1][j + 1] == '@' { adj += 1; }
    if i < map.len() - 1 && j > 0 && map[i + 1][j - 1] == '@' { adj += 1; }
    if i < map.len() - 1 && j < map[0].len() - 1 && map[i + 1][j + 1] == '@' { adj += 1; }
    adj < 4
}

#[cfg(test)]
mod day04_tests {
    use std::fs;

    use crate::y2025::day04::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(13, part_a(fs::read_to_string("input/2025/day04/test.txt").unwrap()));
        assert_eq!(43, part_b(fs::read_to_string("input/2025/day04/test.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(1435, part_a(fs::read_to_string("input/2025/day04/input.txt").unwrap()));
        assert_eq!(8623, part_b(fs::read_to_string("input/2025/day04/input.txt").unwrap()));
    }
}
