pub(crate) fn day11() {
    println!("{}", next_pass("hxbxwxba".to_string()));
    println!("{}", next_pass("hxbxxyzz".to_string()));
}

fn next_pass(input: String) -> String {
    let mut new_pass = inc(input);
    while !is_valid(new_pass.as_str()) {
        // println!("{:?}", new_pass);
        new_pass = inc(new_pass);
    }
    new_pass
}

fn inc(pass: String) -> String {
    let mut new_pass: Vec<char> = pass.chars().collect();

    for i in (0..new_pass.len()).rev() {
        let mut new_c = new_pass[i] as u8 + 1;
        if new_c > b'z' {
            new_c = b'a'
        }
        new_pass[i] = new_c as char;
        if new_c != b'a' {
            break;
        }
    }

    let mut ans = String::new();
    for i in 0..new_pass.len() {
        ans.push(new_pass[i]);
    }
    ans
}

fn is_valid(pass: &str) -> bool {
    if !has_increasing(pass) {
        return false;
    }
    if pass.contains("i") || pass.contains("o") || pass.contains("l") {
        return false;
    }
    if !has_two_pairs(pass) {
        return false;
    }
    true
}

fn has_increasing(pass: &str) -> bool {
    let cs: Vec<u8> = pass.chars().map(|c| c as u8).collect();
    for i in 0..cs.len() - 2 {
        let c1 = cs[i];
        let c2 = cs[i + 1];
        let c3 = cs[i + 2];
        if c1 + 1 == c2 && c2 + 1 == c3 {
            return true;
        }
    }
    false
}

fn has_two_pairs(pass: &str) -> bool {
    let cs: Vec<char> = pass.chars().collect();
    let mut fp = '_';
    for i in 0..cs.len() - 1 {
        let c1 = cs[i];
        let c2 = cs[i + 1];

        if c1 == c2 {
            if fp == c1 || fp == '_' {
                fp = c1
            } else {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod day11_tests {
    use crate::y2015::day11::{inc, is_valid, next_pass};

    #[test]
    fn inc_works() {
        assert_eq!("ab".to_string(), inc("aa".to_string()));
        assert_eq!("ba".to_string(), inc("az".to_string()));
        assert_eq!("xz".to_string(), inc("xy".to_string()));
        assert_eq!("ya".to_string(), inc("xz".to_string()));
        assert_eq!("yb".to_string(), inc("ya".to_string()));
    }

    #[test]
    fn is_valid_works() {
        assert_eq!(false, is_valid("hijklmmn"));
        assert_eq!(false, is_valid("abbceffg"));
        assert_eq!(false, is_valid("abbcegjk"));
        assert_eq!(true, is_valid("abcdffaa"));
        assert_eq!(false, is_valid("hxbxwxyy"));
    }

    #[test]
    fn test_works() {
        assert_eq!("abcdffaa", next_pass("abcdefgh".to_string()));
        assert_eq!("ghjaabcc", next_pass("ghijklmn".to_string()));
    }

    #[test]
    fn input_works() {
        assert_eq!("hxbxxyzz", next_pass("hxbxwxba".to_string()));
        assert_eq!("hxcaabcc", next_pass("hxbxxyzz".to_string()));
    }
}
