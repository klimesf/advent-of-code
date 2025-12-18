use std::fs;

pub(crate) fn day05() {
    println!("{}", part_a(fs::read_to_string("input/2020/day05/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2020/day05/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    input
        .lines()
        .map(|line| {
            let chars = line.chars().collect::<Vec<char>>();
            let mut row = (0, 127);
            chars[0..chars.len() - 3].iter().for_each(|c| match c {
                'F' => row = (row.0, row.1 - ((row.1 - row.0 + 1) / 2)),
                'B' => row = (row.0 + ((row.1 - row.0 + 1) / 2), row.1),
                _ => panic!("Unknown char {}", c),
            });

            let mut col = (0, 7);
            chars[chars.len() - 3..chars.len()].iter().for_each(|c| match c {
                'L' => col = (col.0, col.1 - ((col.1 - col.0 + 1) / 2)),
                'R' => col = (col.0 + ((col.1 - col.0 + 1) / 2), col.1),
                _ => panic!("Unknown char {}", c),
            });
            8 * row.0 + col.0
        })
        .max()
        .unwrap()
}

fn part_b(input: String) -> usize {
    let ids = input
        .lines()
        .map(|line| {
            let chars = line.chars().collect::<Vec<char>>();
            let mut row = (0, 127);
            chars[0..chars.len() - 3].iter().for_each(|c| match c {
                'F' => row = (row.0, row.1 - ((row.1 - row.0 + 1) / 2)),
                'B' => row = (row.0 + ((row.1 - row.0 + 1) / 2), row.1),
                _ => panic!("Unknown char {}", c),
            });

            let mut col = (0, 7);
            chars[chars.len() - 3..chars.len()].iter().for_each(|c| match c {
                'L' => col = (col.0, col.1 - ((col.1 - col.0 + 1) / 2)),
                'R' => col = (col.0 + ((col.1 - col.0 + 1) / 2), col.1),
                _ => panic!("Unknown char {}", c),
            });
            8 * row.0 + col.0
        })
        .collect::<Vec<usize>>();
    let min = *ids.iter().min().unwrap();
    let max = *ids.iter().max().unwrap();

    for i in min..=max {
        if !ids.contains(&i) {
            return i;
        }
    }
    panic!("No solution found");
}

#[cfg(test)]
mod day05_tests {
    use std::fs;

    use crate::y2020::day05::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(820, part_a(fs::read_to_string("input/2020/day05/test.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(938, part_a(fs::read_to_string("input/2020/day05/input.txt").unwrap()));
        assert_eq!(696, part_b(fs::read_to_string("input/2020/day05/input.txt").unwrap()));
    }
}
