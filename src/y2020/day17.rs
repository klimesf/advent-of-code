use std::collections::{HashMap, HashSet};
use std::fs;

pub(crate) fn day17() {
    println!("{}", part_a(fs::read_to_string("input/2020/day17/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2020/day17/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    let mut state = HashSet::new();

    input.lines().enumerate().for_each(|(x, line)| {
        line.chars().enumerate().filter(|(_, c)| *c == '#').for_each(|(y, _)| {
            state.insert((x as isize, y as isize, 1));
        })
    });

    for _ in 0..6 {
        let mut neighbor_count: HashMap<(isize, isize, isize), usize> = HashMap::new();
        let mut new_state = HashSet::new();

        for (x, y, z) in state.clone() {
            for dx in [-1, 0, 1] {
                for dy in [-1, 0, 1] {
                    for dz in [-1, 0, 1] {
                        if dx == 0 && dy == 0 && dz == 0 {
                            continue;
                        }
                        *neighbor_count.entry((x + dx, y + dy, z + dz)).or_insert(0) += 1;
                    }
                }
            }
        }

        for (pos, count) in neighbor_count.iter() {
            if state.contains(pos) {
                if *count == 2 || *count == 3 {
                    new_state.insert(pos.clone());
                }
            } else if *count == 3 {
                new_state.insert(pos.clone());
            }
        }
        state = new_state.clone();
    }

    state.len()
}

fn part_b(input: String) -> usize {
    let mut state = HashSet::new();

    input.lines().enumerate().for_each(|(x, line)| {
        line.chars().enumerate().filter(|(_, c)| *c == '#').for_each(|(y, _)| {
            state.insert((x as isize, y as isize, 1, 1));
        })
    });

    for _ in 0..6 {
        let mut neighbor_count: HashMap<(isize, isize, isize, isize), usize> = HashMap::new();
        let mut new_state = HashSet::new();

        for (x, y, z, w) in state.clone() {
            for dx in [-1, 0, 1] {
                for dy in [-1, 0, 1] {
                    for dz in [-1, 0, 1] {
                        for dw in [-1, 0, 1] {
                            if dx == 0 && dy == 0 && dz == 0 && dw == 0 {
                                continue;
                            }
                            *neighbor_count.entry((x + dx, y + dy, z + dz, w + dw)).or_insert(0) += 1;
                        }
                    }
                }
            }
        }

        for (pos, count) in neighbor_count.iter() {
            if state.contains(pos) {
                if *count == 2 || *count == 3 {
                    new_state.insert(pos.clone());
                }
            } else if *count == 3 {
                new_state.insert(pos.clone());
            }
        }
        state = new_state.clone();
    }

    state.len()
}

#[cfg(test)]
mod day17_tests {
    use std::fs;

    use crate::y2020::day17::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(112, part_a(fs::read_to_string("input/2020/day17/test.txt").unwrap()));
        assert_eq!(848, part_b(fs::read_to_string("input/2020/day17/test.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(232, part_a(fs::read_to_string("input/2020/day17/input.txt").unwrap()));
        assert_eq!(1620, part_b(fs::read_to_string("input/2020/day17/input.txt").unwrap()));
    }
}
