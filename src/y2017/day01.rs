use std::fs;

pub(crate) fn day01() {
    let input = fs::read_to_string("input/2017/day01/input.txt").unwrap();
    println!("{}", captcha_a(input.trim().chars().collect()));
    println!("{}", captcha_b(input.trim().chars().collect()));
}

fn captcha_a(number: Vec<char>) -> u32 {
    (0..number.len())
        .map(|i|
            if number[i] == number[(i + 1) % number.len()] {
                number[i].to_digit(10).unwrap()
            } else { 0 }
        )
        .sum()
}

fn captcha_b(number: Vec<char>) -> u32 {
    let half_len = number.len() / 2;
    (0..number.len())
        .map(|i|
            if number[i] == number[(i + half_len) % number.len()] {
                number[i].to_digit(10).unwrap()
            } else { 0 }
        )
        .sum()
}

#[cfg(test)]
mod day01_tests {
    use std::fs;
    use crate::y2017::day01::{captcha_a, captcha_b};

    #[test]
    fn captcha_a_works() {
        assert_eq!(3, captcha_a("1122".chars().collect()));
        assert_eq!(4, captcha_a("1111".chars().collect()));
        assert_eq!(0, captcha_a("1234".chars().collect()));
        assert_eq!(9, captcha_a("91212129".chars().collect()));
    }

    #[test]
    fn captcha_b_works() {
        assert_eq!(6, captcha_b("1212".chars().collect()));
        assert_eq!(0, captcha_b("1122".chars().collect()));
        assert_eq!(4, captcha_b("123425".chars().collect()));
        assert_eq!(12, captcha_b("123123".chars().collect()));
        assert_eq!(4, captcha_b("12131415".chars().collect()));
    }

    #[test]
    fn input_works() {
        let input = fs::read_to_string("input/2017/day01/input.txt").unwrap();
        assert_eq!(1216, captcha_a(input.trim().chars().collect()));
        assert_eq!(1072, captcha_b(input.trim().chars().collect()));
    }
}
