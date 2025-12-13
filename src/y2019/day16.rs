use itertools::Itertools;
use std::fs;

pub(crate) fn day16() {
    let input = fs::read_to_string("input/2019/day16/input.txt").unwrap();
    let digits: Vec<i32> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();

    part_a(digits.clone());
    part_b(digits.clone());
}

fn part_a(mut digits: Vec<i32>) {
    let sequence = vec![0, 1, 0, -1];
    for _ in 0..100 {
        let mut new_digits = vec![];

        for i in 1..=digits.len() {
            let mut sum = 0;
            for j in 1..=digits.len() {
                let multiply = sequence[(j / i) % 4];
                sum += digits[j - 1] * multiply;
            }
            new_digits.push(sum.abs() % 10);
        }

        digits = new_digits;
    }

    for i in 0..8 {
        print!("{}", digits[i])
    }
    println!();
}

fn part_b(digits: Vec<i32>) {
    // Start = the offset given by the first 7 digits of the input
    let start = (digits[0] * 1000000
        + digits[1] * 100000
        + digits[2] * 10000
        + digits[3] * 1000
        + digits[4] * 100
        + digits[5] * 10
        + digits[6]) as usize;
    let finish = digits.len() * 10000;

    // Only work with the numbers from the offset to the end.. no need to worry about the others
    let mut numbers = vec![];
    for i in start..finish {
        numbers.push(digits[i % digits.len()]);
    }
    numbers.reverse();

    // After the first half, the multiplier will always be 1
    // We can construct the next number from the back (where there is only one 1 - in the last position)
    // => and for each next number, take the previous result and add one number on i - 1
    for _ in 0..100 {
        let mut new_numbers = vec![];
        for i in 0..numbers.len() {
            let number = if i == 0 {
                numbers[i]
            } else {
                numbers[i] + new_numbers[i - 1]
            };
            new_numbers.push(number % 10);
        }
        numbers = new_numbers;
    }

    numbers.reverse();
    println!("{:?}", numbers[0..8].iter().join(""));
}
