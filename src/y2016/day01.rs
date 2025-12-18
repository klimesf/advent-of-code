use std::collections::HashSet;
use std::fs;

pub(crate) fn day01() {
    println!("{}", part_a(fs::read_to_string("input/2016/day01/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2016/day01/input.txt").unwrap()));
}

fn part_a(input: String) -> i32 {
    let mut pos = (0, 0);
    let mut dir = 0;
    input.split(", ").for_each(|instruction| {
        let (rot, ns) = instruction.split_at(1);
        let mv = ns.parse::<i32>().unwrap();

        match rot {
            "L" => dir = (dir + 270) % 360,
            "R" => dir = (dir + 90) % 360,
            _ => panic!("Unknown rotation {}", rot),
        }

        match dir {
            0 => pos = (pos.0, pos.1 + mv),
            90 => pos = (pos.0 + mv, pos.1),
            180 => pos = (pos.0, pos.1 - mv),
            270 => pos = (pos.0 - mv, pos.1),
            _ => panic!("Unknown direction {}", dir),
        }
    });

    pos.0.abs() + pos.1.abs()
}

fn part_b(input: String) -> i32 {
    let mut pos: (i32, i32) = (0, 0);
    let mut dir = 0;
    let mut visited = HashSet::new();
    visited.insert(pos);
    for instruction in input.split(", ") {
        let (rot, ns) = instruction.split_at(1);
        let mv = ns.parse::<i32>().unwrap();

        match rot {
            "L" => dir = (dir + 270) % 360,
            "R" => dir = (dir + 90) % 360,
            _ => panic!("Unknown rotation {}", rot),
        }

        for _ in 0..mv {
            match dir {
                0 => pos = (pos.0, pos.1 + 1),
                90 => pos = (pos.0 + 1, pos.1),
                180 => pos = (pos.0, pos.1 - 1),
                270 => pos = (pos.0 - 1, pos.1),
                _ => panic!("Unknown direction {}", dir),
            }

            if !visited.insert(pos) {
                return pos.0.abs() + pos.1.abs();
            }
        }
    }
    panic!("No location visited twice")
}

#[cfg(test)]
mod day01_tests {
    use std::fs;

    use crate::y2016::day01::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(5, part_a("R2, L3".to_string()));
        assert_eq!(2, part_a("R2, R2, R2".to_string()));
        assert_eq!(12, part_a("R5, L5, R5, R3".to_string()));
        assert_eq!(4, part_b("R8, R4, R4, R8".to_string()));
    }

    #[test]
    fn input_works() {
        assert_eq!(291, part_a(fs::read_to_string("input/2016/day01/input.txt").unwrap()));
        assert_eq!(159, part_b(fs::read_to_string("input/2016/day01/input.txt").unwrap()));
    }
}
