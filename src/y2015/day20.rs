use std::collections::HashMap;

pub(crate) fn day20() {
    println!("{}", part_a(34000000));
    println!("{}", part_b(34000000));
}

fn part_a(target: usize) -> usize {
    let mut houses = vec!{ 0; target / 10 };

    for i in 1..(target / 10) {
        for j in (i..(target / 10)).step_by(i) {
            houses[j] += i * 10;
        }
    }

    for i in 0..houses.len() {
        if houses[i] >= target { return i }
    }
    panic!();
}

fn part_b(target: usize) -> usize {
    let mut houses = vec!{ 0; target / 10 };

    for i in 1..(target / 10) {
        for j in (i..(target / 10)).step_by(i) {
            if j / i > 50 { break }
            houses[j] += i * 11;
        }
    }

    for i in 0..houses.len() {
        if houses[i] >= target { return i }
    }
    panic!();
}

#[allow(unused)]
fn prime_factors(mut n: usize) -> Vec<usize> {
    let mut ans = vec!();
    while n % 2 == 0 {
        ans.push(2);
        n = n / 2;
    }

    let mut i = 3;
    loop {
        if i * i > n { break; }
        while n % i == 0 {
            ans.push(i);
            n = n / i;
        }
        i += 2;
    }

    if n > 2 { ans.push(n); }
    ans
}

#[allow(unused)]
fn house(n: usize) -> usize {
    // house (4) = 4 * 10 + 2 * 10 + 10 = 70
    // house (8) = 8 * 10 + 4 * 10 + 2 * 10 + 10 = 150
    // house (9) = 9 * 10 + 3 * 10 + 10 = 130
    let factors = prime_factors(n);
    let mut factors_ctr = HashMap::new();
    for factor in factors {
        *factors_ctr.entry(factor).or_insert(0) += 1;
    }
    factors_ctr.insert(1, 1);
    factors_ctr.insert(n, 1);
    let mut ans = 0;

    for (factor, cnt) in factors_ctr {
        for i in 1..=cnt {
            if factor.pow(i) == n && factor != n { continue; }
            ans += 10 * factor.pow(i);
        }
    }
    ans
}

#[cfg(test)]
mod day20_tests {
    use crate::y2015::day20::{house, part_a, part_b, prime_factors};

    #[test]
    fn prime_factors_works() {
        assert_eq!(vec! {2}, prime_factors(2));
        assert_eq!(vec! {3}, prime_factors(3));
        assert_eq!(vec! {2, 2}, prime_factors(4));
        assert_eq!(vec! {5}, prime_factors(5));
        assert_eq!(vec! {2, 3}, prime_factors(6));
        assert_eq!(vec! {7}, prime_factors(7));
        assert_eq!(vec! {2, 2, 2}, prime_factors(8));
    }

    #[test]
    fn house_works() {
        assert_eq!(10, house(1));
        assert_eq!(30, house(2));
        assert_eq!(40, house(3));
        assert_eq!(70, house(4));
        assert_eq!(60, house(5));
        assert_eq!(120, house(6));
        assert_eq!(80, house(7));
        assert_eq!(150, house(8));
        assert_eq!(130, house(9));
    }

    #[test]
    fn test_works() {
        assert_eq!(6, part_a(120));
        assert_eq!(8, part_a(150));
    }

    #[test]
    fn input_works() {
        assert_eq!(786240, part_a(34000000));
        assert_eq!(831600, part_b(34000000));
    }
}
