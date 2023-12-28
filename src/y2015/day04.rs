pub(crate) fn day04() {
    println!("{}", part_a("yzbqklnj"));
    println!("{}", part_b("yzbqklnj"));
}

fn part_a(secret: &str) -> usize {
    let mut i = 1;
    loop {
        let s = format!("{}{}", secret, i);
        let hash = format!("{:x}", md5::compute(s));
        if hash.starts_with("00000") {
            return i;
        }
        i += 1;
    }
}

fn part_b(secret: &str) -> usize {
    let mut i = 1;
    loop {
        let s = format!("{}{}", secret, i);
        let hash = format!("{:x}", md5::compute(s));
        if hash.starts_with("000000") {
            return i;
        }
        i += 1;
    }
}

#[cfg(test)]
mod day04_tests {
    use crate::y2015::day04::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(609043, part_a("abcdef"));
        assert_eq!(1048970, part_a("pqrstuv"));
    }

    #[test]
    fn input_works() {
        assert_eq!(282749, part_a("yzbqklnj"));
        assert_eq!(9962624, part_b("yzbqklnj"));
    }
}
