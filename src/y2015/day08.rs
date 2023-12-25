use std::fs;

pub(crate) fn day08() {
    println!("{}", part_a(fs::read_to_string("input/2015/day08/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2015/day08/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    input.lines().map(|line| { line.len() - in_memory_len(line) }).sum()
}

fn part_b(input: String) -> usize {
    input.lines().map(|line| { escaped_len(line) - line.len() }).sum()
}

fn in_memory_len(s: &str) -> usize {
    // This could be solved much easier, but let's make a FSM for fun
    let mut ans = 0;
    let mut state = State::Start;
    for c in s.chars() {
        state = state.next(c, &mut ans);
    }
    ans
}

fn escaped_len(s: &str) -> usize {
    let mut ans = 0;
    for c in s.chars() {
       match c {
           '"' => { ans += 2 }
           '\\' => { ans += 2 }
           _ => { ans += 1 }
       }
    }
    ans + 2 // +2 escaping quotes
}

#[derive(Eq, PartialEq)]
enum State {
    Start,
    Reading,
    Escaping,
    EscapingHex,
    EscapingHex2,
    Terminal,
}

impl State {
    fn next(&self, c: char, ctr: &mut usize) -> State {
        match self {
            State::Start => {
                if c == '"' { State::Reading }
                else { panic!("String doesn't start with escape quotes") }
            }
            State::Reading => {
                if c == '\\' { State::Escaping }
                else if c == '"' { State::Terminal }
                else {
                    *ctr += 1;
                    State::Reading
                }
            }
            State::Escaping => {
                *ctr += 1;
                if c == 'x' { State::EscapingHex }
                else { State::Reading }
            }
            State::EscapingHex => {
                State::EscapingHex2
            }
            State::EscapingHex2 => {
                State::Reading
            }
            State::Terminal => {
                panic!("String doesn't end with escape quotes")
            }
        }
    }
}

#[cfg(test)]
mod day08_tests {
    use std::fs;

    use crate::y2015::day08::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(12, part_a(fs::read_to_string("input/2015/day08/test.txt").unwrap()));
        assert_eq!(19, part_b(fs::read_to_string("input/2015/day08/test.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(1333, part_a(fs::read_to_string("input/2015/day08/input.txt").unwrap()));
        assert_eq!(2046, part_b(fs::read_to_string("input/2015/day08/input.txt").unwrap()));
    }
}
