use std::fs;

pub(crate) fn day09() {
    let input = fs::read_to_string("input/2017/day09/input.txt").unwrap();
    let (part_a, part_b) = score(input.trim());
    println!("{}", part_a);
    println!("{}", part_b);
}

fn score(stream: &str) -> (i32, i32) {
    let mut part_a = 0;
    let mut part_b = 0;
    let mut depth = 0;
    let mut garbage_open = false;
    let mut negate_next = false;
    for c in stream.chars() {
        if negate_next {
            negate_next = false;
            continue;
        }
        if c == '!' {
            negate_next = true;
            continue;
        }
        if garbage_open {
            if c == '>' { garbage_open = false; }
            else { part_b += 1; }
            continue;
        }
        match c {
            '{' => { depth += 1 }
            '}' => {
                part_a += depth;
                depth -= 1;
            }
            '<' => { garbage_open = true; }
            ',' => { }
            _ => panic!("Unknown character {}", c)
        }
    }
    (part_a, part_b)
}

#[cfg(test)]
mod day09_tests {
    use std::fs;
    use crate::y2017::day09::score;

    #[test]
    fn part_a_works() {
        assert_eq!(1, score("{}").0);
        assert_eq!(6, score("{{{}}}").0);
        assert_eq!(5, score("{{},{}}").0);
        assert_eq!(16, score("{{{},{},{{}}}}").0);
        assert_eq!(1, score("{<a>,<a>,<a>,<a>}").0);
        assert_eq!(9, score("{{<ab>},{<ab>},{<ab>},{<ab>}}").0);
        assert_eq!(9, score("{{<!!>},{<!!>},{<!!>},{<!!>}}").0);
        assert_eq!(3, score("{{<a!>},{<a!>},{<a!>},{<ab>}}").0);
    }

    #[test]
    fn part_b_works() {
        assert_eq!(0, score("<>").1);
        assert_eq!(17, score("<random characters>").1);
        assert_eq!(3, score("<<<<>").1);
        assert_eq!(2, score("<{!>}>").1);
        assert_eq!(0, score("<!!>").1);
        assert_eq!(0, score("<!!!>>").1);
        assert_eq!(10, score("<{o\"i!a,<{i<a>").1);
    }

    #[test]
    fn input_works() {
        let input = fs::read_to_string("input/2017/day09/input.txt").unwrap();
        assert_eq!((11089, 5288), score(input.trim()));
    }
}
