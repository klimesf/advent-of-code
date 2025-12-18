use std::collections::HashMap;
use std::{cmp, fs};

pub(crate) fn day03() {
    let (a, b) = solve(fs::read_to_string("input/2023/day03/input.txt").unwrap());
    println!("{}", a);
    println!("{}", b);
}

fn solve(input: String) -> (u32, u32) {
    let engine: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut digits: Vec<(usize, usize, usize, u32)> = vec![];
    let x_max = engine.len();
    let y_max = engine[0].len();

    engine.clone().into_iter().enumerate().for_each(|(x, row)| {
        let mut num = 0;
        let mut start_y = y_max + 1;
        row.into_iter().enumerate().for_each(|(y, c)| {
            if c.is_ascii_digit() {
                num = num * 10 + c.to_digit(10).unwrap();
                start_y = cmp::min(y, start_y);
                if y == y_max - 1 {
                    digits.push((x, start_y, y, num));
                }
            } else {
                if num > 0 {
                    digits.push((x, start_y, y - 1, num));
                }
                num = 0;
                start_y = y_max + 1;
            }
        });
    });

    let mut sum_a = 0;
    let mut gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    digits.clone().iter().for_each(|(x, y1, y2, num)| {
        for ix in -1..=1 {
            let dx: usize = cmp::min(x_max - 1, cmp::max(0, *x as i32 + ix) as usize);
            for iy in -1..=1 {
                for jy in *y1..=*y2 {
                    let dy: usize = cmp::min(y_max - 1, cmp::max(0, jy as i32 + iy) as usize);
                    if engine[dx][dy] != '.' && !engine[dx][dy].is_ascii_digit() {
                        sum_a += num;
                        if engine[dx][dy] == '*' {
                            gears.entry((dx, dy)).or_insert(vec![]).push(*num);
                        }
                        return;
                    }
                }
            }
        }
    });

    (
        sum_a,
        gears
            .values()
            .filter(|nums| nums.len() > 1)
            .map(|nums| nums.iter().map(|n| *n).product::<u32>())
            .sum(),
    )
}

#[cfg(test)]
mod day03_tests {
    use std::fs;

    use crate::y2023::day03::solve;

    #[test]
    fn test_works() {
        let input_a = fs::read_to_string("input/2023/day03/test.txt").unwrap();
        assert_eq!((4361, 467835), solve(input_a));
    }

    #[test]
    fn input_works() {
        assert_eq!((539713, 84159075), solve(fs::read_to_string("input/2023/day03/input.txt").unwrap()));
    }
}
