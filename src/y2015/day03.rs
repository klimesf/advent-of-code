use std::collections::HashSet;
use std::fs;

pub(crate) fn day03() {
    println!("{}", part_a(fs::read_to_string("input/2015/day03/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2015/day03/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    let mut houses = HashSet::new();
    let mut pos = (0, 0);
    houses.insert(pos);
    input.chars().for_each(|c| {
        match c {
            '^' => pos = (pos.0 - 1, pos.1),
            'v' => pos = (pos.0 + 1, pos.1),
            '>' => pos = (pos.0, pos.1 - 1),
            '<' => pos = (pos.0, pos.1 + 1),
            _ => {
                panic!()
            }
        }
        houses.insert(pos);
    });
    houses.len()
}

fn part_b(input: String) -> usize {
    let mut houses = HashSet::new();
    let mut santa_pos = (0, 0);
    let mut robo_santa_pos = (0, 0);
    houses.insert(santa_pos);
    let mut turn = 0;
    input.chars().for_each(|c| {
        if turn % 2 == 0 {
            match c {
                '^' => santa_pos = (santa_pos.0 - 1, santa_pos.1),
                'v' => santa_pos = (santa_pos.0 + 1, santa_pos.1),
                '>' => santa_pos = (santa_pos.0, santa_pos.1 - 1),
                '<' => santa_pos = (santa_pos.0, santa_pos.1 + 1),
                _ => {
                    panic!()
                }
            }
            houses.insert(santa_pos);
        } else {
            match c {
                '^' => robo_santa_pos = (robo_santa_pos.0 - 1, robo_santa_pos.1),
                'v' => robo_santa_pos = (robo_santa_pos.0 + 1, robo_santa_pos.1),
                '>' => robo_santa_pos = (robo_santa_pos.0, robo_santa_pos.1 - 1),
                '<' => robo_santa_pos = (robo_santa_pos.0, robo_santa_pos.1 + 1),
                _ => {
                    panic!()
                }
            }
            houses.insert(robo_santa_pos);
        }
        turn += 1;
    });
    houses.len()
}

#[cfg(test)]
mod day03_tests {
    use std::fs;

    use crate::y2015::day03::{part_a, part_b};

    #[test]
    fn input_works() {
        assert_eq!(2081, part_a(fs::read_to_string("input/2015/day03/input.txt").unwrap()));
        assert_eq!(2341, part_b(fs::read_to_string("input/2015/day03/input.txt").unwrap()));
    }
}
