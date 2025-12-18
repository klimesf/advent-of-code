use std::fs;

pub(crate) fn day01() {
    println!("{}", part_a(fs::read_to_string("input/2015/day01/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2015/day01/input.txt").unwrap()));
}

fn part_a(input: String) -> i32 {
    let mut floor = 0;
    input.chars().for_each(|c| match c {
        '(' => floor += 1,
        ')' => floor -= 1,
        _ => {
            panic!()
        }
    });
    floor
}

fn part_b(input: String) -> i32 {
    let mut floor = 0;
    let mut instr = 1;
    for c in input.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {
                panic!()
            }
        }
        if floor == -1 {
            return instr;
        }
        instr += 1;
    }
    panic!()
}

#[cfg(test)]
mod day01_tests {
    use std::fs;

    use crate::y2015::day01::{part_a, part_b};

    #[test]
    fn input_works() {
        assert_eq!(138, part_a(fs::read_to_string("input/2015/day01/input.txt").unwrap()));
        assert_eq!(1771, part_b(fs::read_to_string("input/2015/day01/input.txt").unwrap()));
    }
}
