use std::collections::HashSet;
use std::fs;

pub(crate) fn day07() {
    println!("{}", part_a(fs::read_to_string("input/2016/day07/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2016/day07/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    input.lines().filter(|addr| supports_tls(addr)).count()
}

fn part_b(input: String) -> usize {
    input.lines().filter(|addr| supports_ssl(addr)).count()
}

fn supports_tls(addr: &str) -> bool {
    let (out_bracket_parts, in_bracket_parts) = split_brackets(addr);

    out_bracket_parts.iter().any(|addr| contains_abba(addr)) && in_bracket_parts.iter().all(|addr| !contains_abba(addr))
}

fn supports_ssl(addr: &str) -> bool {
    let (out_bracket_parts, in_bracket_parts) = split_brackets(addr);

    let mut abas = HashSet::new();
    let mut babs = HashSet::new();
    for obp in out_bracket_parts {
        for aba in get_abas(obp) {
            abas.insert(aba);
        }
    }
    for ibp in in_bracket_parts {
        for aba in get_abas(ibp) {
            babs.insert(aba);
        }
    }

    for aba in &abas {
        for bab in &babs {
            let aba_chars: Vec<char> = aba.chars().collect();
            let bab_chars: Vec<char> = bab.chars().collect();

            if aba_chars[0] == bab_chars[1] && aba_chars[1] == bab_chars[0] {
                return true;
            }
        }
    }

    false
}

fn split_brackets(addr: &str) -> (Vec<String>, Vec<String>) {
    let mut out_bracket_parts = vec![];
    let mut in_bracket_parts = vec![];
    let mut curr = String::new();
    let mut in_bracket = false;
    let chars: Vec<char> = addr.chars().collect();
    for i in 0..addr.len() {
        let c = chars[i];
        if c == '[' {
            in_bracket = true;
            out_bracket_parts.push(curr.to_string());
            curr.clear();
        } else if c == ']' {
            in_bracket = false;
            in_bracket_parts.push(curr.to_string());
            curr.clear();
        } else {
            curr.push(c);
        }
    }
    if in_bracket {
        in_bracket_parts.push(curr.to_string());
    } else {
        out_bracket_parts.push(curr.to_string());
    }
    (out_bracket_parts, in_bracket_parts)
}

fn contains_abba(addr: &str) -> bool {
    let chars: Vec<char> = addr.chars().collect();
    for i in 0..(addr.len() - 3) {
        let i1 = chars[i];
        let i2 = chars[i + 1];
        let i3 = chars[i + 2];
        let i4 = chars[i + 3];

        if i1 != i2 && i1 == i4 && i2 == i3 {
            return true;
        }
    }
    false
}

fn get_abas(addr: String) -> Vec<String> {
    let mut abas = vec![];
    let chars: Vec<char> = addr.chars().collect();
    for i in 0..(addr.len() - 2) {
        let i1 = chars[i];
        let i2 = chars[i + 1];
        let i3 = chars[i + 2];

        if i1 != i2 && i1 == i3 {
            abas.push(format!("{}{}{}", i1, i2, i3))
        }
    }
    abas
}

#[cfg(test)]
mod day07_tests {
    use std::fs;

    use crate::y2016::day07::{part_a, part_b, supports_ssl, supports_tls};

    #[test]
    fn supports_tls_works() {
        assert_eq!(true, supports_tls("abba[mnop]qrst"));
        assert_eq!(false, supports_tls("abcd[bddb]xyyx"));
        assert_eq!(false, supports_tls("aaaa[qwer]tyui"));
        assert_eq!(true, supports_tls("ioxxoj[asdfgh]zxcvbn"));
    }

    #[test]
    fn supports_ssl_works() {
        assert_eq!(true, supports_ssl("aba[bab]xyz"));
        assert_eq!(false, supports_ssl("xyx[xyx]xyx"));
        assert_eq!(true, supports_ssl("aaa[kek]eke"));
        assert_eq!(true, supports_ssl("zazbz[bzb]cdb"));
    }

    #[test]
    fn test_works() {
        assert_eq!(2, part_a(fs::read_to_string("input/2016/day07/test.txt").unwrap()));
        assert_eq!(0, part_b(fs::read_to_string("input/2016/day07/test.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(115, part_a(fs::read_to_string("input/2016/day07/input.txt").unwrap()));
        assert_eq!(231, part_b(fs::read_to_string("input/2016/day07/input.txt").unwrap()));
    }
}
