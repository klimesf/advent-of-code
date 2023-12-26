pub(crate) fn day10() {
    println!("{}", part_a("1113122113".to_string()));
    println!("{}", part_b("1113122113".to_string()));
}

fn part_a(input: String) -> usize {
    let mut ans = input.clone();
    for _ in 0..40 {
        ans = look_and_say(ans)
    }
    ans.len()
}

fn part_b(input: String) -> usize {
    let mut ans = input.clone();
    for _ in 0..50 {
        ans = look_and_say(ans)
    }
    ans.len()
}

fn look_and_say(s: String) -> String {
    let mut ans = String::new();
    let mut last_c = 'x';
    let mut last_c_ctr = 0;

    for c in s.chars() {
        if c == last_c {
            last_c_ctr += 1;
        } else {
            if last_c.is_ascii_digit() {
                ans.push_str(format!("{}{}", last_c_ctr, last_c).as_str())
            }
            last_c = c;
            last_c_ctr = 1;
        }
    }
    ans.push_str(format!("{}{}", last_c_ctr, last_c).as_str());

    ans
}

#[cfg(test)]
mod day10_tests {
    use crate::y2015::day10::{look_and_say, part_a, part_b};

    #[test]
    fn look_and_say_works() {
        assert_eq!("11", look_and_say("1".to_string()));
        assert_eq!("21", look_and_say("11".to_string()));
        assert_eq!("1211", look_and_say("21".to_string()));
        assert_eq!("111221", look_and_say("1211".to_string()));
        assert_eq!("312211", look_and_say("111221".to_string()));
    }

    #[test]
    fn input_works() {
        assert_eq!(360154, part_a("1113122113".to_string()));
        assert_eq!(2080688, part_b("1113122113".to_string()));
    }
}
