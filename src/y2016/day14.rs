use std::collections::HashSet;
use std::fs;

pub(crate) fn day14() {
    // gen_hashes("input/2016/day14/test.txt", "abc", 0);
    // gen_hashes("input/2016/day14/input.txt", "ngcjuoqr", 0);
    // gen_hashes("input/2016/day14/test_b.txt", "abc", 2016);
    // gen_hashes("input/2016/day14/input_b.txt", "ngcjuoqr", 2016);
    // println!("{}", solve(fs::read_to_string("input/2016/day14/test.txt").unwrap()));
    // println!("{}", solve(fs::read_to_string("input/2016/day14/test_b.txt").unwrap()));
    println!("{}", solve(fs::read_to_string("input/2016/day14/input.txt").unwrap()));
    println!("{}", solve(fs::read_to_string("input/2016/day14/input_b.txt").unwrap()));
}

#[allow(unused)]
fn gen_hashes(path: &str, salt: &str, rep: usize) {
    let mut data = String::new();
    for i in 0..30000 {
        let mut hash = format!("{:x}", md5::compute(format!("{}{}", salt, i)));
        for i in 0..rep {
            hash = format!("{:x}", md5::compute(hash));
        }
        data.push_str(format!("{}\n", hash).as_str());
    }
    fs::write(path, data).expect("Cannot write to file");
}

fn solve(input: String) -> usize {
    let hashes: Vec<&str> = input.lines().collect();
    let mut i = 0;
    let mut key_ctr = 0;

    loop {
        let hash = hashes[i];

        match find_triplet(hash.to_string()) {
            Some(triplet) => {
                for j in i + 1..=i + 1000 {
                    if find_quintet(hashes[j].to_string()).contains(&triplet) {
                        key_ctr += 1;
                        if key_ctr == 64 {
                            return i;
                        }
                        break;
                    }
                }
            }
            None => {}
        }

        i += 1;
    }
}

fn find_triplet(hash: String) -> Option<char> {
    let chars: Vec<char> = hash.chars().collect();
    for i in 0..chars.len() - 2 {
        let a = chars[i];
        let b = chars[i + 1];
        let c = chars[i + 2];

        if a == b && b == c {
            return Some(a);
        }
    }
    None
}

fn find_quintet(hash: String) -> HashSet<char> {
    let chars: Vec<char> = hash.chars().collect();
    let mut quintets = HashSet::new();
    for i in 0..chars.len() - 4 {
        let a = chars[i];
        let b = chars[i + 1];
        let c = chars[i + 2];
        let d = chars[i + 3];
        let e = chars[i + 4];

        if a == b && b == c && c == d && d == e {
            quintets.insert(a);
        }
    }
    quintets
}

#[cfg(test)]
mod day14_tests {
    use std::fs;
    use crate::y2016::day14::{find_triplet, solve};

    #[test]
    fn find_triplet_works() {
        assert_eq!(Some('8'), find_triplet("0034e0923cc38887a57bd7b1d4f953df".to_string()));
        assert_eq!(None, find_triplet("65b7d651740a924bef52e4eefa46fe76".to_string()));
    }

    #[test]
    fn test_works() {
        assert_eq!(22728, solve(fs::read_to_string("input/2016/day14/test.txt").unwrap()));
        assert_eq!(22551, solve(fs::read_to_string("input/2016/day14/test_b.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(18626, solve(fs::read_to_string("input/2016/day14/input.txt").unwrap()));
        assert_eq!(20092, solve(fs::read_to_string("input/2016/day14/input_b.txt").unwrap()));
    }
}
