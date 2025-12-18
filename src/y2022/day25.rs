use std::fs;

pub(crate) fn day25() {
    let input = fs::read_to_string("input/2022/day25/input.txt").unwrap();
    let numbers: Vec<&str> = input.lines().collect();
    let ans = numbers.iter().map(|number| snafu_to_decimal(number)).sum::<i64>();
    println!("{}", ans);
    println!("{}", decimal_to_snafu(ans));
}

fn snafu_to_decimal(snafu: &str) -> i64 {
    snafu
        .chars()
        .enumerate()
        .map(|(i, c)| {
            let base = 5_i64.pow((snafu.len() - i - 1) as u32);
            match c {
                '2' => base * 2,
                '1' => base,
                '0' => 0,
                '-' => base * -1,
                '=' => base * -2,
                _ => panic!(),
            }
        })
        .sum()
}

fn decimal_to_snafu(mut number: i64) -> String {
    let mut res = vec![];
    while number > 0 {
        let remainder = number % 5;
        number = number / 5; // Dividing the number by 5 effectively makes it 1 digit shorter in base 5
        match remainder {
            0 => {
                res.push('0');
            }
            1 => {
                res.push('1');
            }
            2 => {
                res.push('2');
            }
            // If the remainder is larger than 2, we need to carry over +1 to the next place and subtract from it
            3 => {
                number += 1;
                res.push('=');
            }
            4 => {
                number += 1;
                res.push('-');
            }
            _ => panic!("reminder {} after moduling by 5", remainder),
        }
    }
    res.iter().rev().collect() // The vector is in reversed order since we go from the back
}

#[cfg(test)]
mod day25_tests {
    use crate::y2022::day25::{decimal_to_snafu, snafu_to_decimal};

    #[test]
    fn snafu_to_decimal_works() {
        assert_eq!(1, snafu_to_decimal("1"));
        assert_eq!(1747, snafu_to_decimal("1=-0-2"));
        assert_eq!(906, snafu_to_decimal("12111"));
    }

    #[test]
    fn decimal_to_snafu_works() {
        assert_eq!("1", decimal_to_snafu(1));
        assert_eq!("12111", decimal_to_snafu(906));
        assert_eq!("1=-0-2", decimal_to_snafu(1747));
        assert_eq!("2=-1=0", decimal_to_snafu(4890));
    }
}
