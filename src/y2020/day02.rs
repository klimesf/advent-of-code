use std::fs;

pub(crate) fn day02() {
    println!("{}", part_a(fs::read_to_string("input/2020/day02/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2020/day02/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    input
        .lines()
        .filter(|line| {
            let (rule, password) = line.split_once(": ").unwrap();

            let (times, s_char) = rule.split_once(" ").unwrap();
            let (s_min, s_max) = times.split_once("-").unwrap();

            let min = s_min.parse::<usize>().unwrap();
            let max = s_max.parse::<usize>().unwrap();
            let char = s_char.chars().nth(0).unwrap();

            let char_count = password.chars().filter(|c| *c == char).count();

            char_count >= min && char_count <= max
        })
        .count()
}

fn part_b(input: String) -> usize {
    input
        .lines()
        .filter(|line| {
            let (rule, password) = line.split_once(": ").unwrap();

            let (times, s_char) = rule.split_once(" ").unwrap();
            let (s_pos_1, s_pos_2) = times.split_once("-").unwrap();

            let pos_1 = s_pos_1.parse::<usize>().unwrap();
            let pos_2 = s_pos_2.parse::<usize>().unwrap();
            let char = s_char.chars().nth(0).unwrap();

            let pos_1_has_char = password.chars().nth(pos_1 - 1) == Some(char);
            let pos_2_has_char = password.chars().nth(pos_2 - 1) == Some(char);

            pos_1_has_char != pos_2_has_char
        })
        .count()
}

#[cfg(test)]
mod day02_tests {
    use std::fs;

    use crate::y2020::day02::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(2, part_a(fs::read_to_string("input/2020/day02/test.txt").unwrap()));
        assert_eq!(1, part_b(fs::read_to_string("input/2020/day02/test.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(622, part_a(fs::read_to_string("input/2020/day02/input.txt").unwrap()));
        assert_eq!(263, part_b(fs::read_to_string("input/2020/day02/input.txt").unwrap()));
    }
}
