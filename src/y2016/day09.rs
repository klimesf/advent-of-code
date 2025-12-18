use std::fs;

pub(crate) fn day09() {
    println!("{}", part_a(fs::read_to_string("input/2016/day09/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2016/day09/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    decompressed_size(input.as_str())
}

fn part_b(input: String) -> usize {
    decompressed_size_v2(input.as_str())
}

fn decompressed_size(s: &str) -> usize {
    let chars: Vec<char> = s.chars().collect();
    let mut size = 0;
    let mut in_bracket = false;
    let mut bracket_start = 0;
    let mut i: usize = 0;
    while i < s.len() {
        if chars[i] == '(' {
            in_bracket = true;
            i += 1;
            bracket_start = i;
        } else if chars[i] == ')' {
            in_bracket = false;
            let (l, r) = s[bracket_start..i].split_once("x").unwrap();
            i += l.parse::<usize>().unwrap() + 1;
            size += l.parse::<usize>().unwrap() * r.parse::<usize>().unwrap();
        } else {
            if !in_bracket {
                size += 1
            }
            i += 1;
        }
    }
    size
}

fn decompressed_size_v2(s: &str) -> usize {
    let chars: Vec<char> = s.chars().collect();
    let mut size = 0;
    let mut in_bracket = false;
    let mut bracket_start = 0;
    let mut i: usize = 0;
    while i < s.len() {
        if chars[i] == '(' {
            in_bracket = true;
            i += 1;
            bracket_start = i;
        } else if chars[i] == ')' {
            in_bracket = false;
            let (l, r) = s[bracket_start..i].split_once("x").unwrap();
            size += decompressed_size_v2(s[(i + 1)..(i + 1 + l.parse::<usize>().unwrap())].to_string().as_str())
                * r.parse::<usize>().unwrap();
            i += l.parse::<usize>().unwrap() + 1;
        } else {
            if !in_bracket {
                size += 1
            }
            i += 1;
        }
    }
    size
}

#[cfg(test)]
mod day09_tests {
    use std::fs;

    use crate::y2016::day09::{decompressed_size, decompressed_size_v2, part_a, part_b};

    #[test]
    fn decompressed_size_works() {
        assert_eq!(6, decompressed_size("ADVENT"));
        assert_eq!(7, decompressed_size("A(1x5)BC"));
        assert_eq!(9, decompressed_size("(3x3)XYZ"));
        assert_eq!(11, decompressed_size("A(2x2)BCD(2x2)EFG"));
        assert_eq!(6, decompressed_size("(6x1)(1x3)A"));
        assert_eq!(18, decompressed_size("X(8x2)(3x3)ABCY")); // X(3x3)ABC(3x3)ABCY
    }

    #[test]
    fn decompressed_size_v2_works() {
        assert_eq!(3, decompressed_size_v2("XYZ"));
        assert_eq!(9, decompressed_size_v2("(3x3)XYZ"));
        assert_eq!(20, decompressed_size_v2("X(8x2)(3x3)ABCY"));
        assert_eq!(241920, decompressed_size_v2("(27x12)(20x12)(13x14)(7x10)(1x12)A"));
        assert_eq!(445, decompressed_size_v2("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"));
    }

    #[test]
    fn input_works() {
        assert_eq!(74532, part_a(fs::read_to_string("input/2016/day09/input.txt").unwrap()));
        assert_eq!(11558231665, part_b(fs::read_to_string("input/2016/day09/input.txt").unwrap()));
    }
}
