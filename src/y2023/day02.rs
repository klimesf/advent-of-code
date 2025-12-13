use std::{cmp, fs};

pub(crate) fn day02() {
    println!(
        "{}",
        part_a(fs::read_to_string("input/2023/day02/input.txt").unwrap())
    );
    println!(
        "{}",
        part_b(fs::read_to_string("input/2023/day02/input.txt").unwrap())
    );
}

fn part_a(games: String) -> i32 {
    games
        .lines()
        .map(|line| {
            let (id_str, cubes_str) = line.split_once(": ").unwrap();
            let id = &id_str[5..];

            for set_str in cubes_str.split("; ") {
                let mut red_total = 12;
                let mut green_total = 13;
                let mut blue_total = 14;

                for cube in set_str.split(", ") {
                    let (num_str, color) = cube.split_once(" ").unwrap();
                    let num = num_str.parse::<i32>().unwrap();

                    match color {
                        "red" => red_total -= num,
                        "green" => green_total -= num,
                        "blue" => blue_total -= num,
                        _ => panic!("unknown color {}", color),
                    }

                    if red_total < 0 || green_total < 0 || blue_total < 0 {
                        return 0;
                    }
                }
            }

            id.parse().unwrap()
        })
        .sum()
}

fn part_b(games: String) -> i32 {
    games
        .lines()
        .map(|line| {
            let (_, cubes_str) = line.split_once(": ").unwrap();

            let mut red_max = 0;
            let mut green_max = 0;
            let mut blue_max = 0;

            for set_str in cubes_str.split("; ") {
                for cube in set_str.split(", ") {
                    let (num_str, color) = cube.split_once(" ").unwrap();
                    let num = num_str.parse::<i32>().unwrap();

                    match color {
                        "red" => red_max = cmp::max(red_max, num),
                        "green" => green_max = cmp::max(green_max, num),
                        "blue" => blue_max = cmp::max(blue_max, num),
                        _ => panic!("unknown color {}", color),
                    }
                }
            }

            red_max * green_max * blue_max
        })
        .sum()
}

#[cfg(test)]
mod day02_tests {
    use std::fs;

    use crate::y2023::day02::{part_a, part_b};

    #[test]
    fn test_works() {
        let input_a = fs::read_to_string("input/2023/day02/test.txt").unwrap();
        assert_eq!(8, part_a(input_a));
        let input_b = fs::read_to_string("input/2023/day02/test.txt").unwrap();
        assert_eq!(2286, part_b(input_b));
    }

    #[test]
    fn input_works() {
        assert_eq!(
            2085,
            part_a(fs::read_to_string("input/2023/day02/input.txt").unwrap())
        );
        assert_eq!(
            79315,
            part_b(fs::read_to_string("input/2023/day02/input.txt").unwrap())
        );
    }
}
