use std::collections::HashMap;
use std::fs;

pub(crate) fn day12() {
    println!("{}", part_a(fs::read_to_string("input/2023/day12/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2023/day12/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    input
        .lines()
        .map(|line| {
            let (springs, r) = line.split_once(" ").unwrap();
            let groups: Vec<usize> = r.split(",").map(|n| n.parse::<usize>().unwrap()).collect();
            solve(springs, &groups)
        })
        .sum()
}

fn part_b(input: String) -> usize {
    input
        .lines()
        .map(|line| {
            let (springs, r) = line.split_once(" ").unwrap();
            let groups: Vec<usize> = r.split(",").map(|n| n.parse::<usize>().unwrap()).collect();
            let groups_5: Vec<usize> = groups.iter().cycle().take(groups.len() * 5).map(|n| *n).collect();
            let springs_5 = format!("{}?{}?{}?{}?{}", springs, springs, springs, springs, springs);
            solve(springs_5.as_str(), &groups_5)
        })
        .sum()
}

fn solve(springs: &str, groups: &Vec<usize>) -> usize {
    let mut state_space = HashMap::new();
    state_space.insert((0, 0), 1);
    for si in 0..springs.len() {
        let c = springs.chars().nth(si).unwrap();
        let mut new_state_space = HashMap::new();

        for ((gi, g_size), cardinality) in state_space {
            if (c == '.' || c == '?') && g_size > 0 && g_size == groups[gi] {
                *new_state_space.entry((gi + 1, 0)).or_insert(0) += cardinality;
            } else if (c == '.' || c == '?') && g_size == 0 {
                *new_state_space.entry((gi, g_size)).or_insert(0) += cardinality;
            }
            if (c == '#' || c == '?') && gi < groups.len() && g_size < groups[gi] {
                *new_state_space.entry((gi, g_size + 1)).or_insert(0) += cardinality;
            }
        }

        state_space = new_state_space;
    }

    state_space
        .iter()
        .filter(|((gi, g_size), _)| {
            (*g_size == 0 && *gi == groups.len()) || (*gi == groups.len() - 1 && *g_size == groups[*gi])
        })
        .map(|(_, cardinality)| *cardinality)
        .sum()
}

#[cfg(test)]
mod day12_tests {
    use std::fs;

    use crate::y2023::day12::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(21, part_a(fs::read_to_string("input/2023/day12/test.txt").unwrap()));
        assert_eq!(525152, part_b(fs::read_to_string("input/2023/day12/test.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(7716, part_a(fs::read_to_string("input/2023/day12/input.txt").unwrap()));
        assert_eq!(18716325559999, part_b(fs::read_to_string("input/2023/day12/input.txt").unwrap()));
    }
}
