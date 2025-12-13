use std::fs;

pub(crate) fn day02() {
    println!(
        "{}",
        part_a(fs::read_to_string("input/2016/day02/input.txt").unwrap())
    );
    println!(
        "{}",
        part_b(fs::read_to_string("input/2016/day02/input.txt").unwrap())
    );
}

fn part_a(input: String) -> i32 {
    let keypad = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

    let mut r = 1;
    let mut c = 1;
    input
        .lines()
        .map(|instructions| {
            for i in instructions.chars() {
                match i {
                    'U' => {
                        if r > 0 {
                            r -= 1
                        }
                    }
                    'D' => {
                        if r < 2 {
                            r += 1
                        }
                    }
                    'L' => {
                        if c > 0 {
                            c -= 1
                        }
                    }
                    'R' => {
                        if c < 2 {
                            c += 1
                        }
                    }
                    _ => panic!("Unknown instruction {}", i),
                }
            }

            keypad[r][c]
        })
        .reduce(|a, b| a * 10 + b)
        .unwrap()
}

fn part_b(input: String) -> String {
    let keypad = vec![
        vec![' ', ' ', '1', ' ', ' '],
        vec![' ', '2', '3', '4', ' '],
        vec!['5', '6', '7', '8', '9'],
        vec![' ', 'A', 'B', 'C', ' '],
        vec![' ', ' ', 'D', ' ', ' '],
    ];

    let mut ans = String::new();
    let mut r = 2;
    let mut c = 0;

    for instructions in input.lines() {
        for i in instructions.chars() {
            match i {
                'U' => {
                    if r > 0 && keypad[r - 1][c] != ' ' {
                        r -= 1
                    }
                }
                'D' => {
                    if r < 4 && keypad[r + 1][c] != ' ' {
                        r += 1
                    }
                }
                'L' => {
                    if c > 0 && keypad[r][c - 1] != ' ' {
                        c -= 1
                    }
                }
                'R' => {
                    if c < 4 && keypad[r][c + 1] != ' ' {
                        c += 1
                    }
                }
                _ => panic!("Unknown instruction {}", i),
            }
        }

        ans.push(keypad[r][c])
    }

    ans
}

#[cfg(test)]
mod day02_tests {
    use std::fs;

    use crate::y2016::day02::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(
            1985,
            part_a(fs::read_to_string("input/2016/day02/test.txt").unwrap())
        );
        assert_eq!(
            "5DB3",
            part_b(fs::read_to_string("input/2016/day02/test.txt").unwrap())
        );
    }

    #[test]
    fn input_works() {
        assert_eq!(
            92435,
            part_a(fs::read_to_string("input/2016/day02/input.txt").unwrap())
        );
        assert_eq!(
            "C1A88",
            part_b(fs::read_to_string("input/2016/day02/input.txt").unwrap())
        );
    }
}
