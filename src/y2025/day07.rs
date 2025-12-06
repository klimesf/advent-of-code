use std::collections::{HashMap, HashSet};
use std::fs;

pub(crate) fn day07(print: fn(usize)) {
    print(part_a(fs::read_to_string("input/2025/day07/input.txt").unwrap()));
    print(part_b(fs::read_to_string("input/2025/day07/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    let map: Vec<Vec<char>> = input.lines().map(|line| {
        line.chars().collect()
    }).collect();
    
    let mut start = (0, 0);
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 'S' {
                start = (i, j);
                break;
            }
        }
    }
    
    let mut stack = vec!();
    stack.push(start);
    
    let mut ans = HashSet::new();
    while let Some(pos) = stack.pop() {
        let (i, j) = pos;
        if ans.contains(&(i, j)) { continue; }
        if i >= map.len() || j >= map[0].len() { continue; }

        if map[i][j] == '^' {
            stack.push((i, j + 1));
            stack.push((i, j - 1));
            ans.insert((i, j));
        } else {
            stack.push((i + 1, j));
        }
    }

    ans.iter().len()
}

fn part_b(input: String) -> usize {
    let map: Vec<Vec<char>> = input.lines().map(|line| {
        line.chars().collect()
    }).collect();

    let mut start = (0, 0);
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 'S' {
                start = (i, j);
                break;
            }
        }
    }

    let mut cache = HashMap::new();
    cached_rec(start.0, start.1, &map, &mut cache)
}

fn cached_rec(i: usize, j: usize, map: &Vec<Vec<char>>, cache: &mut HashMap<(usize, usize), usize>) -> usize {
    if i >= map.len() && j < map[0].len() { return 1; }
    if j >= map[0].len() { return 0; }
    if cache.contains_key(&(i, j)) {
        return *cache.get(&(i, j)).unwrap();
    }

    if map[i][j] == '^' {
        let ans = cached_rec(i, j + 1, map, cache) + cached_rec(i, j - 1, map, cache);
        cache.insert((i, j), ans);
        ans
    } else {
        let ans = cached_rec(i + 1, j, map, cache);
        cache.insert((i, j), ans);
        ans
    }
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
