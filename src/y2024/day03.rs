use std::fs;

pub(crate) fn day03() {
    println!("{}", part_a(fs::read_to_string("input/2024/day03/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2024/day03/input.txt").unwrap()));
}

fn part_a(_: String) -> usize {
    0
}

fn part_b(_: String) -> usize {
    0
}

#[cfg(test)]
mod day03_tests {
    use std::fs;

    use crate::y2024::day03::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(0, part_a(fs::read_to_string("input/2024/day03/test.txt").unwrap()));
        assert_eq!(0, part_b(fs::read_to_string("input/2024/day03/test.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(0, part_a(fs::read_to_string("input/2024/day03/input.txt").unwrap()));
        assert_eq!(0, part_b(fs::read_to_string("input/2024/day03/input.txt").unwrap()));
    }
}
