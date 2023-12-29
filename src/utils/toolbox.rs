use core::mem;
use regex::Match;

#[allow(dead_code)]
pub(crate) fn gcd(mut a: i32, mut b: i32) -> i32 {
    if b > a {
        mem::swap(&mut a, &mut b);
    }
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    return a.max(-a);
}

#[allow(dead_code)]
pub(crate) fn lcm(a: i32, b: i32) -> i32 {
    if a > b {
        (a * b) / gcd(a, b)
    } else {
        (a * b) / gcd(b, a)
    }
}

#[allow(dead_code)]
pub(crate) fn gcd_64(mut a: i64, mut b: i64) -> i64 {
    if b > a {
        mem::swap(&mut a, &mut b);
    }
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    return a.max(-a);
}

#[allow(dead_code)]
pub(crate) fn lcm_64(a: i64, b: i64) -> i64 {
    if a > b {
        (a * b) / gcd_64(a, b)
    } else {
        (a * b) / gcd_64(b, a)
    }
}

#[allow(dead_code)]
pub(crate) fn parse_usize(g: Option<Match>) -> usize {
    g.map_or(0, |m| m.as_str().parse().unwrap())
}

#[allow(dead_code)]
pub(crate) fn parse_i32(g: Option<Match>) -> i32 {
    return g.map_or(0, |m| m.as_str().parse().unwrap());
}

#[allow(dead_code)]
pub(crate) fn parse_i64(g: Option<Match>) -> i64 {
    return g.map_or(0, |m| m.as_str().parse().unwrap());
}

#[allow(dead_code)]
pub(crate) fn parse_char(g: Option<Match>) -> char {
    return g.map_or('_', |m| m.as_str().chars().nth(0).unwrap());
}

#[allow(dead_code)]
pub(crate) fn prime_factors(mut n: u32) -> Vec<u32> {
    let mut ans = vec!();
    while n % 2 == 0 {
        ans.push(2);
        n /= 2;
    }

    for i in (3..=(n as f64).sqrt() as u32).step_by(2) {
        while n % i == 0
        {
            ans.push(i);
            n /= i;
        }
    }

    if n > 2 { ans.push(n) }

    ans
}

#[macro_export]
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

#[macro_export]
macro_rules! measure {
    ($s:stmt) => {
        let timer = std::time::Instant::now();
        $s
        println!("{}", format!("Elapsed: {:?}", timer.elapsed()).italic().dimmed());
    };
}

#[cfg(test)]
mod toolbox_tests {
    use crate::utils::toolbox::prime_factors;

    #[test]
    fn prime_factors_works() {
        assert_eq!(vec! {2}, prime_factors(2));
        assert_eq!(vec! {3}, prime_factors(3));
        assert_eq!(vec! {2, 2}, prime_factors(4));
        assert_eq!(vec! {5}, prime_factors(5));
        assert_eq!(vec! {2, 3}, prime_factors(6));
        assert_eq!(vec! {7}, prime_factors(7));
        assert_eq!(vec! {2, 2, 2}, prime_factors(8));
        assert_eq!(vec! {3, 3}, prime_factors(9));
        assert_eq!(vec! {2, 2, 7}, prime_factors(28));
    }
}
