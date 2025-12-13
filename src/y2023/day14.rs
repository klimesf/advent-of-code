use std::fs;

pub(crate) fn day14() {
    println!(
        "{}",
        part_a(fs::read_to_string("input/2023/day14/input.txt").unwrap())
    );
    println!(
        "{}",
        part_b(fs::read_to_string("input/2023/day14/input.txt").unwrap())
    );
}

fn part_a(input: String) -> usize {
    let mut dish: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    for r in 0..dish.len() {
        for c in 0..dish[r].len() {
            if dish[r][c] != 'O' {
                continue;
            }

            for i in 1..=r {
                if dish[r - i][c] == '.' {
                    dish[r - i][c] = 'O';
                    dish[r - i + 1][c] = '.';
                } else {
                    break;
                }
            }
        }
    }

    let mut ans = 0;
    for r in 0..dish.len() {
        'uh: for c in 0..dish[r].len() {
            if dish[r][c] != 'O' {
                continue 'uh;
            }
            ans += dish.len() - r;
        }
    }

    ans
}

fn part_b(input: String) -> usize {
    let mut dish: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut prev_dishes: Vec<Vec<Vec<char>>> = vec![];
    let mut repeat = (0, 0);
    'outer: for cycle in 0..1000000000 {
        'next: for i in 0..prev_dishes.len() {
            let prev_dish = &prev_dishes[i];
            for r in 0..dish.len() {
                for c in 0..dish[r].len() {
                    if dish[r][c] != prev_dish[r][c] {
                        continue 'next;
                    }
                }
            }

            repeat = (i, cycle);
            prev_dishes.push(dish.clone());
            break 'outer;
        }

        prev_dishes.push(dish.clone());

        // North
        for r in 0..dish.len() {
            for c in 0..dish[r].len() {
                if dish[r][c] != 'O' {
                    continue;
                }

                for i in 1..=r {
                    if dish[r - i][c] == '.' {
                        dish[r - i][c] = 'O';
                        dish[r - i + 1][c] = '.';
                    } else {
                        break;
                    }
                }
            }
        }

        // West
        for r in 0..dish.len() {
            for c in 0..dish[r].len() {
                if dish[r][c] != 'O' {
                    continue;
                }

                for i in 1..=c {
                    if dish[r][c - i] == '.' {
                        dish[r][c - i] = 'O';
                        dish[r][c - i + 1] = '.';
                    } else {
                        break;
                    }
                }
            }
        }

        // South
        for r in (0..dish.len()).rev() {
            for c in 0..dish[r].len() {
                if dish[r][c] != 'O' {
                    continue;
                }

                for i in r..(dish.len() - 1) {
                    if dish[i + 1][c] == '.' {
                        dish[i + 1][c] = 'O';
                        dish[i][c] = '.';
                    } else {
                        break;
                    }
                }
            }
        }

        // East
        for r in 0..dish.len() {
            for c in (0..dish[r].len()).rev() {
                if dish[r][c] != 'O' {
                    continue;
                }

                for i in c..(dish[r].len() - 1) {
                    if dish[r][i + 1] == '.' {
                        dish[r][i + 1] = 'O';
                        dish[r][i] = '.';
                    } else {
                        break;
                    }
                }
            }
        }
    }

    let period = repeat.1 - repeat.0;
    let remaining = (1000000000 - repeat.0) % period;
    dish = prev_dishes[repeat.0 + remaining].clone();

    let mut ans = 0;
    for r in 0..dish.len() {
        for c in 0..dish[r].len() {
            if dish[r][c] != 'O' {
                continue;
            }
            ans += dish.len() - r;
        }
    }

    ans
}

#[cfg(test)]
mod day14_tests {
    use std::fs;

    use crate::y2023::day14::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(
            136,
            part_a(fs::read_to_string("input/2023/day14/test.txt").unwrap())
        );
        assert_eq!(
            64,
            part_b(fs::read_to_string("input/2023/day14/test.txt").unwrap())
        );
    }

    #[test]
    fn input_works() {
        assert_eq!(
            109345,
            part_a(fs::read_to_string("input/2023/day14/input.txt").unwrap())
        );
        assert_eq!(
            112452,
            part_b(fs::read_to_string("input/2023/day14/input.txt").unwrap())
        );
    }
}
