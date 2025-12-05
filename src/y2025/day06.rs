use std::fs;

pub(crate) fn day06(print: fn(usize)) {
    print(part_a(fs::read_to_string("input/2025/day06/input.txt").unwrap()));
    print(part_b(fs::read_to_string("input/2025/day06/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    let mut rows: Vec<Vec<usize>> = vec!();
    let mut ans = 0;
    input.lines().for_each(|line| {
        if line.contains("*") {
            line.split_whitespace().enumerate().for_each(|(col, c)| {
                match c {
                    "*" => {
                        let mut tot = 1;
                        for i in 0..rows.len() {
                            tot *= rows[i][col];
                        }
                        ans += tot;
                    },
                    "+" => {
                        let mut tot = 0;
                        for i in 0..rows.len() {
                            tot += rows[i][col];
                        }
                        ans += tot;
                    }
                    _ => panic!("invalid char {}", c)
                }
            })
        } else {
            let row = line
                .split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            rows.push(row);
        }
    });
    ans
}

fn part_b(input: String) -> usize {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let max_col = map.iter().map(|row| row.len()).max().unwrap();


    let mut op = '.';
    let mut ans = 0;
    let mut nums: Vec<usize> = vec!();
    for col in 0..max_col {
        let mut whitespace_ct = 0;
        let mut num = String::new();
        for row in 0..map.len() {
            if col >= map[row].len() {
                continue;
            }
            if map[row][col].is_whitespace() {
                whitespace_ct += 1;
            } else if map[row][col].is_alphanumeric() {
                num.push(map[row][col]);
            } else {
                if op != '.' {
                    panic!("two operators!");
                } else {
                    op = map[row][col];
                }
            }
        }

        if whitespace_ct == map.len() {
            match op {
                '*' => {
                    let mut tot = 1;
                    for i in 0..nums.len() {
                        tot *= nums[i];
                    }
                    ans += tot;
                },
                '+' => {
                    let mut tot = 0;
                    for i in 0..nums.len() {
                        tot += nums[i];
                    }
                    ans += tot;
                }
                _ => panic!("invalid op {}", op)
            }
            op = '.';
            nums.clear();
        } else {
            nums.push(num.parse::<usize>().unwrap());
            num.clear();
        }
    }

    // Also one last line, last column is not all whitespace
    match op {
        '*' => {
            let mut tot = 1;
            for i in 0..nums.len() {
                tot *= nums[i];
            }
            ans += tot;
        },
        '+' => {
            let mut tot = 0;
            for i in 0..nums.len() {
                tot += nums[i];
            }
            ans += tot;
        }
        _ => panic!("invalid op {}", op)
    }
    nums.clear();

    ans
}

#[cfg(test)]
mod day06_tests {
    use std::fs;

    use crate::y2025::day06::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(4277556, part_a(fs::read_to_string("input/2025/day06/test.txt").unwrap()));
        assert_eq!(3263827, part_b(fs::read_to_string("input/2025/day06/test.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(3785892992137, part_a(fs::read_to_string("input/2025/day06/input.txt").unwrap()));
        assert_eq!(7669802156452, part_b(fs::read_to_string("input/2025/day06/input.txt").unwrap()));
    }
}
