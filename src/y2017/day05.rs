use std::fs;

pub(crate) fn day05() {
    let input = fs::read_to_string("input/2017/day05/input.txt").unwrap();
    let instructions: Vec<i32> = input.lines().map(|line| line.parse::<i32>().unwrap()).collect();
    println!("{}", part_a(instructions.clone()));
    println!("{}", part_b(instructions))
}

fn part_a(mut instructions: Vec<i32>) -> i32 {
    let mut i = 0;
    let mut cursor = 0;
    loop {
        let next_cursor = cursor + instructions[cursor as usize];
        i += 1;
        if next_cursor < 0 || next_cursor as usize >= instructions.len() {
            return i;
        }
        instructions[cursor as usize] += 1;
        cursor = next_cursor;
    }
}

fn part_b(mut instructions: Vec<i32>) -> i32 {
    let mut i = 0;
    let mut cursor = 0;
    loop {
        let next_cursor = cursor + instructions[cursor as usize];
        i += 1;
        if next_cursor < 0 || next_cursor as usize >= instructions.len() {
            return i;
        }
        if instructions[cursor as usize] >= 3 {
            instructions[cursor as usize] -= 1;
        } else {
            instructions[cursor as usize] += 1;
        }
        cursor = next_cursor;
    }
}

#[cfg(test)]
mod day05_tests {
    use crate::y2017::day05::{part_a, part_b};
    use std::fs;

    #[test]
    fn test_works() {
        let input = fs::read_to_string("input/2017/day05/test.txt").unwrap();
        let instructions: Vec<i32> = input.lines().map(|line| line.parse::<i32>().unwrap()).collect();
        assert_eq!(5, part_a(instructions.clone()));
        assert_eq!(10, part_b(instructions));
    }

    #[test]
    fn input_works() {
        let input = fs::read_to_string("input/2017/day05/input.txt").unwrap();
        let instructions: Vec<i32> = input.lines().map(|line| line.parse::<i32>().unwrap()).collect();
        assert_eq!(356945, part_a(instructions.clone()));
        assert_eq!(28372145, part_b(instructions));
    }
}
