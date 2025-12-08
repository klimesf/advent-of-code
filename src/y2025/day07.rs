use std::fs;

pub(crate) fn day07(print: fn(usize)) {
    print(part_a(fs::read_to_string("input/2025/day07/input.txt").unwrap()));
    print(part_b(fs::read_to_string("input/2025/day07/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    let map: Vec<Vec<char>> = input.lines().map(|line| {
        line.chars().collect()
    }).collect();
    
    let mut collector = vec![0; map.len()];
    let mut ans = 0;
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            match map[row][col] {
                'S' => collector[col] += 1,
                '^' => {
                    if collector[col] == 0 { continue; }
                    collector[col] = 0;
                    collector[col - 1] = 1;
                    collector[col + 1] = 1;
                    ans += 1;
                }
                _ => { }
            }
        }
    }
    ans
}

fn part_b(input: String) -> usize {
    let map: Vec<Vec<char>> = input.lines().map(|line| {
        line.chars().collect()
    }).collect();

    let mut collector = vec![0; map.len()];
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            match map[row][col] {
                'S' => collector[col] += 1,
                '^' => {
                    if collector[col] == 0 { continue; }
                    collector[col - 1] += collector[col];
                    collector[col + 1] += collector[col];
                    collector[col] = 0;
                }
                _ => { }
            }
        }
    }
    collector.iter().sum()
}

#[cfg(test)]
mod day07_tests {
    use std::fs;

    use crate::y2025::day07::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(21, part_a(fs::read_to_string("input/2025/day07/test.txt").unwrap()));
        assert_eq!(40, part_b(fs::read_to_string("input/2025/day07/test.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(1605, part_a(fs::read_to_string("input/2025/day07/input.txt").unwrap()));
        assert_eq!(29893386035180, part_b(fs::read_to_string("input/2025/day07/input.txt").unwrap()));
    }
}
