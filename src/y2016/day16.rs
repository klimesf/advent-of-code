use itertools::Itertools;

pub(crate) fn day16() {
    println!("{}", solve(272, "10010000000110000"));
    println!("{}", solve(35651584, "10010000000110000"));
}

fn solve(disk_len: usize, initial: &str) -> String {
    let mut data: Vec<char> = initial.chars().collect();
    while data.len() < disk_len {
        data = dragon_curve(data);
    }

    let cs = checksum(data[0..disk_len].iter().map(|c| *c).collect());
    let mut ans = String::new();
    for i in 0..cs.len() {
        ans.push(cs[i]);
    }
    ans
}

fn dragon_curve(a: Vec<char>) -> Vec<char> {
    let mut b = a.clone();
    b.reverse();
    b = b
        .iter()
        .map(|c| if *c == '1' { '0' } else { '1' })
        .collect();

    let mut ans = vec![];
    for i in 0..a.len() {
        ans.push(a[i]);
    }
    ans.push('0');
    for i in 0..b.len() {
        ans.push(b[i]);
    }
    ans
}

fn checksum(mut data: Vec<char>) -> Vec<char> {
    data = data
        .into_iter()
        .tuples()
        .map(|(a, b)| if a == b { '1' } else { '0' })
        .collect();

    while data.len() % 2 == 0 {
        data = data
            .into_iter()
            .tuples()
            .map(|(a, b)| if a == b { '1' } else { '0' })
            .collect();
    }

    data
}

#[cfg(test)]
mod day16_tests {
    use crate::y2016::day16::{checksum, dragon_curve, solve};

    fn dragon_curve_s(a: &str) -> String {
        let res = dragon_curve(a.chars().collect());
        let mut ans = String::new();
        res.iter().for_each(|c| ans.push(*c));
        ans
    }

    fn checksum_s(data: &str) -> String {
        let res = checksum(data.chars().collect());
        let mut ans = String::new();
        res.iter().for_each(|c| ans.push(*c));
        ans
    }

    #[test]
    fn dragon_curve_works() {
        assert_eq!("100", dragon_curve_s("1"));
        assert_eq!("001", dragon_curve_s("0"));
        assert_eq!("11111000000", dragon_curve_s("11111"));
        assert_eq!("1111000010100101011110000", dragon_curve_s("111100001010"));
    }

    #[test]
    fn checksum_works() {
        assert_eq!("100", checksum_s("110010110100"));
    }

    #[test]
    fn test_works() {
        assert_eq!("01100", solve(20, "10000"));
    }

    #[test]
    fn input_works() {
        assert_eq!("10010110010011110", solve(272, "10010000000110000"));
        assert_eq!("01101011101100011", solve(35651584, "10010000000110000"));
    }
}
