pub(crate) fn day05() {
    println!("{}", part_a("uqwqemis"));
    println!("{}", part_b("uqwqemis"));
}

fn part_a(door_id: &str) -> String {
    let mut password = String::new();
    let mut i = 0;
    while password.len() < 8 {
        let s = format!("{}{}", door_id, i);
        let hash = format!("{:x}", md5::compute(s));
        if hash.starts_with("00000") {
            password.push(hash.chars().nth(5).unwrap());
        }
        i += 1;
    }
    password
}

fn part_b(door_id: &str) -> String {
    let mut password = vec![' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '];
    let mut i = 0;
    while password.iter().filter(|c| **c == ' ').count() > 0 {
        let s = format!("{}{}", door_id, i);
        let hash = format!("{:x}", md5::compute(s));
        if hash.starts_with("00000") {
            let pos = hash.chars().nth(5).unwrap() as usize - '0' as usize;
            if pos < 8 && password[pos] == ' ' {
                let char = hash.chars().nth(6).unwrap();
                password[pos] = char;
            }
        }
        i += 1;
    }

    format!(
        "{}{}{}{}{}{}{}{}",
        password[0],
        password[1],
        password[2],
        password[3],
        password[4],
        password[5],
        password[6],
        password[7]
    )
}

#[cfg(test)]
mod day05_tests {
    use crate::y2016::day05::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!("18f47a30", part_a("abc"));
        assert_eq!("05ace8e3", part_b("abc"));
    }

    #[test]
    fn input_works() {
        assert_eq!("1a3099aa", part_a("uqwqemis"));
        assert_eq!("694190cd", part_b("uqwqemis"));
    }
}
