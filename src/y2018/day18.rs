use std::collections::HashMap;
use std::fs;

pub(crate) fn day18() {
    println!(
        "{}",
        part_a(fs::read_to_string("input/2018/day18/input.txt").unwrap())
    );
    println!(
        "{}",
        part_b(fs::read_to_string("input/2018/day18/input.txt").unwrap())
    );
}

fn part_a(input: String) -> usize {
    let mut state: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    for _ in 0..10 {
        state = get_new_state(&state);
    }

    let wooded: usize = state
        .iter()
        .map(|row| row.iter().filter(|c| **c == '|').count())
        .sum();
    let lumbers: usize = state
        .iter()
        .map(|row| row.iter().filter(|c| **c == '#').count())
        .sum();
    wooded * lumbers
}

fn part_b(input: String) -> usize {
    let mut state: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut seen = HashMap::new();
    let mut i = 0;
    let period_len;
    loop {
        if seen.contains_key(&state) {
            period_len = i - seen.get(&state).unwrap();
            break;
        } else {
            seen.insert(state.clone(), i);
        }

        state = get_new_state(&state);
        i += 1;
    }

    for _ in 0..(1000000000 - i) % period_len {
        state = get_new_state(&state);
    }

    let wooded: usize = state
        .iter()
        .map(|row| row.iter().filter(|c| **c == '|').count())
        .sum();
    let lumbers: usize = state
        .iter()
        .map(|row| row.iter().filter(|c| **c == '#').count())
        .sum();
    wooded * lumbers
}

fn get_new_state(state: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_state = vec![vec! {'.'; state[0].len()}; state.len()];
    for r in 0..state.len() {
        for c in 0..state.len() {
            let neighbors = count_neighbors(r, c, &state);
            match state[r][c] {
                '.' => {
                    if neighbors.2 >= 3 {
                        new_state[r][c] = '|'
                    } else {
                        new_state[r][c] = '.'
                    }
                }
                '|' => {
                    if neighbors.1 >= 3 {
                        new_state[r][c] = '#'
                    } else {
                        new_state[r][c] = '|'
                    }
                }
                '#' => {
                    if neighbors.1 >= 1 && neighbors.2 >= 1 {
                        new_state[r][c] = '#'
                    } else {
                        new_state[r][c] = '.'
                    }
                }
                v => {
                    panic!("Unknown state {}", v)
                }
            }
        }
    }
    new_state
}

fn count_neighbors(r: usize, c: usize, state: &Vec<Vec<char>>) -> (usize, usize, usize) {
    let mut ans = (0, 0, 0);
    let ri = r as i32;
    let ci = c as i32;

    for dr in [-1, 0, 1] {
        for dc in [-1, 0, 1] {
            if ri + dr < 0 || ri + dr >= state.len() as i32 {
                continue;
            }
            if ci + dc < 0 || ci + dc >= state[0].len() as i32 {
                continue;
            }
            if dr == 0 && dc == 0 {
                continue;
            }

            match state[(ri + dr) as usize][(ci + dc) as usize] {
                '.' => ans = (ans.0 + 1, ans.1, ans.2),
                '#' => ans = (ans.0, ans.1 + 1, ans.2),
                '|' => ans = (ans.0, ans.1, ans.2 + 1),
                _ => {
                    panic!()
                }
            }
        }
    }
    ans
}

#[cfg(test)]
mod day18_tests {
    use std::fs;

    use crate::y2018::day18::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(
            1147,
            part_a(fs::read_to_string("input/2018/day18/test.txt").unwrap())
        );
    }

    #[test]
    fn input_works() {
        assert_eq!(
            638400,
            part_a(fs::read_to_string("input/2018/day18/input.txt").unwrap())
        );
        assert_eq!(
            195952,
            part_b(fs::read_to_string("input/2018/day18/input.txt").unwrap())
        );
    }
}
