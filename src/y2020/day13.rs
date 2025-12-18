use std::fs;

pub(crate) fn day13() {
    println!("{}", part_a(fs::read_to_string("input/2020/day13/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2020/day13/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    let (f, s) = input.split_once("\n").unwrap();
    let earliest = f.parse::<usize>().unwrap();
    let buses = s
        .split(",")
        .filter(|x| x != &"x")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut time = earliest;
    loop {
        for bus in &buses {
            if time % bus == 0 {
                return bus * (time - earliest);
            }
        }
        time += 1;
    }
}

fn part_b(input: String) -> i64 {
    let (_, s) = input.split_once("\n").unwrap();
    let buses = s
        .split(",")
        .map(|x| if x == "x" { "0" } else { x })
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    // t = 7 * a
    // t + 1 = 13 * b
    // t + 4 = 59 * c
    // t + 6 = 31 * d
    // t + 7 = 19 * e

    // t mod 7 = 0
    // t mod 13 = -1 = 12
    // t mod 31 = -6 = 25
    // t mod 19 = -7 = 12
    // t mod 59 = -4 = 55

    let mut moduli: Vec<i64> = vec![];
    let mut remainders: Vec<i64> = vec![];

    for t in 0..buses.len() {
        if buses[t] == 0 {
            continue;
        }

        let modulo = buses[t] as i64;
        moduli.push(modulo);
        let remainder = (-1 * t as i64).rem_euclid(modulo);
        remainders.push(remainder);
    }

    chinese_remainder_theorem(remainders.as_slice(), moduli.as_slice()).unwrap()
}

fn update_step(a: &mut i64, old_a: &mut i64, quotient: i64) {
    let temp = *a;
    *a = *old_a - quotient * temp;
    *old_a = temp;
}

pub fn extended_euclidean_algorithm(a: i64, b: i64) -> (i64, i64, i64) {
    let (mut old_r, mut rem) = (a, b);
    let (mut old_s, mut coeff_s) = (1, 0);
    let (mut old_t, mut coeff_t) = (0, 1);

    while rem != 0 {
        let quotient = old_r / rem;

        update_step(&mut rem, &mut old_r, quotient);
        update_step(&mut coeff_s, &mut old_s, quotient);
        update_step(&mut coeff_t, &mut old_t, quotient);
    }

    (old_r, old_s, old_t)
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = extended_euclidean_algorithm(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

pub fn chinese_remainder_theorem(residues: &[i64], modulli: &[i64]) -> Option<i64> {
    let prod = modulli.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulli) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }
    Some(sum % prod)
}

#[cfg(test)]
mod day13_tests {
    use std::fs;

    use crate::y2020::day13::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(295, part_a(fs::read_to_string("input/2020/day13/test.txt").unwrap()));
        assert_eq!(1068781, part_b(fs::read_to_string("input/2020/day13/test.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(203, part_a(fs::read_to_string("input/2020/day13/input.txt").unwrap()));
        assert_eq!(905694340256752, part_b(fs::read_to_string("input/2020/day13/input.txt").unwrap()));
    }
}
