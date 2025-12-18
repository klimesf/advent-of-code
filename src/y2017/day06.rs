use std::collections::HashMap;
use std::fs;

pub(crate) fn day06() {
    let input = fs::read_to_string("input/2017/day06/input.txt").unwrap();
    let banks = input.split_whitespace().map(|line| line.parse().unwrap()).collect();
    let (part_a, part_b) = redistribute(banks);
    println!("{}", part_a);
    println!("{}", part_b);
}

fn redistribute(mut banks: Vec<usize>) -> (usize, usize) {
    let banks_len = banks.len();
    let mut ctr = 0;
    let mut seen_states = HashMap::new();
    seen_states.insert(banks.clone(), ctr);
    loop {
        let mut pos = 0;
        let mut max = usize::MIN;
        for i in 0..banks_len {
            if banks[i] > max {
                pos = i;
                max = banks[i];
            }
        }
        banks[pos] = 0;
        for i in 1..=max {
            banks[(pos + i as usize) % banks_len] += 1;
        }
        ctr += 1;

        if let Some(prev_ctr) = seen_states.insert(banks.clone(), ctr) {
            return (ctr, ctr - prev_ctr);
        }
    }
}

#[cfg(test)]
mod day06_tests {
    use std::fs;

    use crate::y2017::day06::redistribute;

    #[test]
    fn redistribute_works() {
        let (part_a, part_b) = redistribute(vec![0, 2, 7, 0]);
        assert_eq!(5, part_a);
        assert_eq!(4, part_b);
    }

    #[test]
    fn input_works() {
        let input = fs::read_to_string("input/2017/day06/input.txt").unwrap();
        let banks = input.split_whitespace().map(|line| line.parse().unwrap()).collect();
        let (part_a, part_b) = redistribute(banks);
        assert_eq!(6681, part_a);
        assert_eq!(2392, part_b);
    }
}
