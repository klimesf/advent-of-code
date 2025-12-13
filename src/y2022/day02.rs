use std::fs;

pub(crate) fn day02() {
    let input = fs::read_to_string("input/2022/day02/input.txt").unwrap();
    part_a(input.clone());
    part_b(input);
}

fn part_a(input: String) {
    let mut total = 0;
    for round in input.lines() {
        let parts: Vec<&str> = round.split(' ').collect();
        let opponent = map_shape(parts[0]);
        let you = map_shape(parts[1]);

        let points_for_round = match opponent {
            "ROCK" => match you {
                "ROCK" => 3,
                "PAPER" => 6,
                "SCISSORS" => 0,
                _ => panic!("Unknown shape {}", you),
            },
            "PAPER" => match you {
                "ROCK" => 0,
                "PAPER" => 3,
                "SCISSORS" => 6,
                _ => panic!("Unknown shape {}", you),
            },
            "SCISSORS" => match you {
                "ROCK" => 6,
                "PAPER" => 0,
                "SCISSORS" => 3,
                _ => panic!("Unknown shape {}", you),
            },
            _ => panic!("Unknown opponent shape {}", opponent),
        };

        total += points_for_round + points_for_shape(you);
    }
    println!("{}", total)
}

fn part_b(input: String) {
    let mut total = 0;
    for round in input.lines() {
        let parts: Vec<&str> = round.split(' ').collect();
        let opponent = map_shape(parts[0]);
        let outcome = parts[1];

        let points_for_round = match outcome {
            "X" => 0,
            "Y" => 3,
            "Z" => 6,
            _ => panic!("Unknown outcome {}", outcome),
        };

        let you = match opponent {
            "ROCK" => match points_for_round {
                0 => "SCISSORS",
                3 => "ROCK",
                6 => "PAPER",
                _ => panic!("Unknown outcome {}", points_for_round),
            },
            "PAPER" => match points_for_round {
                0 => "ROCK",
                3 => "PAPER",
                6 => "SCISSORS",
                _ => panic!("Unknown outcome {}", points_for_round),
            },
            "SCISSORS" => match points_for_round {
                0 => "PAPER",
                3 => "SCISSORS",
                6 => "ROCK",
                _ => panic!("Unknown outcome {}", points_for_round),
            },
            _ => panic!("Unknown shape {}", opponent),
        };

        // println!("{} vs {} = {} + {}", opponent, your_shape, points_for_round, points_for_shape(your_shape));
        total += points_for_round + points_for_shape(you);
    }
    println!("{}", total)
}

fn map_shape(shape: &str) -> &str {
    return match shape {
        "A" | "X" => "ROCK",
        "B" | "Y" => "PAPER",
        "C" | "Z" => "SCISSORS",
        _ => panic!("Unknown shape {}", shape),
    };
}

fn points_for_shape(shape: &str) -> i32 {
    return match shape {
        "ROCK" => 1,
        "PAPER" => 2,
        "SCISSORS" => 3,
        _ => panic!("Unknown shape {}", shape),
    };
}

#[cfg(test)]
mod day02_tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
