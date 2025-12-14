use std::collections::HashMap;
use std::fs;

pub(crate) fn day10() {
    println!(
        "{}",
        part_a(fs::read_to_string("input/2020/day10/input.txt").unwrap())
    );
    println!(
        "{}",
        part_b(fs::read_to_string("input/2020/day10/input.txt").unwrap())
    );
}

fn part_a(input: String) -> usize {
    let mut adapters = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<usize>>();

    adapters.push(0); // Start output
    adapters.sort();

    let mut diff_1 = 0;
    let mut diff_3 = 0;

    adapters.windows(2).for_each(|window| {
        let diff = window[1] - window[0];
        match diff {
            1 => diff_1 += 1,
            3 => diff_3 += 1,
            _ => panic!("diff is {}", diff),
        }
    });

    diff_1 * (diff_3 + 1) // Plus your adapter, always diff 3
}

fn part_b(input: String) -> usize {
    let mut adapters = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<usize>>();

    adapters.push(0); // Start output
    adapters.sort();
    adapters.push(adapters[adapters.len() - 1] + 3);
    let mut cache = HashMap::new();

    recursive_arrange(0, &adapters, &mut cache)
}

fn recursive_arrange(
    pos: usize,
    adapters: &Vec<usize>,
    cache: &mut HashMap<usize, usize>,
) -> usize {
    if pos == adapters.len() - 1 {
        return 1;
    }
    if cache.contains_key(&pos) {
        return cache[&pos];
    }

    let mut combinations = 0;
    combinations += recursive_arrange(pos + 1, adapters, cache);

    if pos < adapters.len() - 2 && adapters[pos + 2] - adapters[pos] <= 3 {
        combinations += recursive_arrange(pos + 2, adapters, cache);
    }

    if pos < adapters.len() - 3 && adapters[pos + 3] - adapters[pos] <= 3 {
        combinations += recursive_arrange(pos + 3, adapters, cache);
    }

    cache.insert(pos, combinations);
    combinations
}

#[cfg(test)]
mod day10_tests {
    use std::fs;

    use crate::y2020::day10::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(
            7 * 5,
            part_a(fs::read_to_string("input/2020/day10/test.txt").unwrap())
        );
        assert_eq!(
            22 * 10,
            part_a(fs::read_to_string("input/2020/day10/test_2.txt").unwrap())
        );
        assert_eq!(
            8,
            part_b(fs::read_to_string("input/2020/day10/test.txt").unwrap())
        );
        assert_eq!(
            19208,
            part_b(fs::read_to_string("input/2020/day10/test_2.txt").unwrap())
        );
    }

    #[test]
    fn input_works() {
        assert_eq!(
            2046,
            part_a(fs::read_to_string("input/2020/day10/input.txt").unwrap())
        );
        assert_eq!(
            1157018619904,
            part_b(fs::read_to_string("input/2020/day10/input.txt").unwrap())
        );
    }
}
