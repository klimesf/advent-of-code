use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

pub(crate) fn day06() {
    println!("{}", part_a(fs::read_to_string("input/2016/day06/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2016/day06/input.txt").unwrap()));
}

fn part_a(input: String) -> String {
    let (frequency, max_pos) = get_frequencies(input);

    let mut ans = String::new();
    for i in 0..=max_pos {
        let most = frequency
            .get(&i)
            .unwrap()
            .iter()
            .sorted_by(|(_, count1), (_, count2)| count2.cmp(count1))
            .map(|(c, _)| *c)
            .collect_vec();
        ans.push(most[0])
    }
    ans
}

fn part_b(input: String) -> String {
    let (frequency, max_pos) = get_frequencies(input);

    let mut ans = String::new();
    for i in 0..=max_pos {
        let most = frequency
            .get(&i)
            .unwrap()
            .iter()
            .sorted_by(|(_, count1), (_, count2)| count1.cmp(count2))
            .map(|(c, _)| *c)
            .collect_vec();
        ans.push(most[0])
    }
    ans
}

fn get_frequencies(input: String) -> (HashMap<usize, HashMap<char, i32>>, usize) {
    let mut frequency = HashMap::new();
    let mut max_pos = 0;
    input.lines().for_each(|line| {
        line.chars().enumerate().for_each(|(pos, c)| {
            if pos > max_pos {
                max_pos = pos
            }
            let pos_map = frequency.entry(pos).or_insert(HashMap::new());
            *pos_map.entry(c).or_insert(0) += 1;
        });
    });
    (frequency, max_pos)
}

#[cfg(test)]
mod day06_tests {
    use std::fs;

    use crate::y2016::day06::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!("easter", part_a(fs::read_to_string("input/2016/day06/test.txt").unwrap()));
        assert_eq!("advent", part_b(fs::read_to_string("input/2016/day06/test.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!("gebzfnbt", part_a(fs::read_to_string("input/2016/day06/input.txt").unwrap()));
        assert_eq!("fykjtwyn", part_b(fs::read_to_string("input/2016/day06/input.txt").unwrap()));
    }
}
