use std::fs;

pub(crate) fn day17() {
    let input = fs::read_to_string("input/2017/day17/input.txt")
        .unwrap()
        .trim()
        .parse::<usize>()
        .unwrap();
    println!("{}", spinlock(input));
    println!("{}", spinlock_more(input));
}

fn spinlock(moves: usize) -> usize {
    let mut buffer = vec![0];
    let mut pos = 0;
    for i in 1..=2017 {
        pos = (pos + moves) % i;
        buffer.insert(pos + 1, i);
        pos += 1;
    }
    buffer[(pos + 1) % buffer.len()]
}

fn spinlock_more(moves: usize) -> usize {
    let mut ans = 0;
    let mut pos = 0;
    for i in 1..=50000000 {
        pos = (pos + moves) % i;
        if pos == 0 {
            ans = i
        }
        pos += 1;
    }
    ans
}

#[cfg(test)]
mod day17_tests {
    use crate::y2017::day17::{spinlock, spinlock_more};
    use std::fs;

    #[test]
    fn test_works() {
        assert_eq!(638, spinlock(3));
    }

    #[test]
    fn input_works() {
        let input = fs::read_to_string("input/2017/day17/input.txt")
            .unwrap()
            .trim()
            .parse::<usize>()
            .unwrap();
        assert_eq!(204, spinlock(input));
        assert_eq!(28954211, spinlock_more(input));
    }
}
