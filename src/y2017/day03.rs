use std::collections::HashMap;
use std::fs;

pub(crate) fn day03() {
    let input = fs::read_to_string("input/2017/day03/input.txt")
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();
    println!("{}", part_a(input));
    println!("{}", part_b(input));
}

fn part_a(num: i32) -> i32 {
    // RULLDDRRRUUULLLLDDDDRRRRRUUUUULLLLLL...
    let mut pos: (i32, i32) = (0, 0);
    let mut curr = 1;
    let mut ctr = [1, 1, 2, 2];
    let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut dir = 0;
    while curr < num {
        for _ in 0..ctr[dir] {
            pos = (pos.0 + dirs[dir].0, pos.1 + dirs[dir].1);
            curr += 1;
            if curr == num {
                break;
            }
        }
        ctr[dir] += 2;
        dir = (dir + 1) % 4;
    }
    return pos.0.abs() + pos.1.abs();
}

fn part_b(num: i32) -> i32 {
    let mut pos = (0, 0);
    let mut curr = 1;
    let mut ctr = [1, 1, 2, 2];
    let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let neighbors = [(0, 1), (0, -1), (1, 0), (-1, 0), (1, 1), (1, -1), (-1, 1), (-1, -1)];
    let mut dir = 0;
    let mut memory: HashMap<(i32, i32), i32> = HashMap::new();
    memory.insert(pos, curr);

    'outer: loop {
        for _ in 0..ctr[dir] {
            pos = (pos.0 + dirs[dir].0, pos.1 + dirs[dir].1);
            curr = neighbors
                .iter()
                .map(|(dx, dy)| memory.get(&(pos.0 + dx, pos.1 + dy)).unwrap_or(&0))
                .sum();
            memory.insert(pos, curr);
            if curr > num {
                break 'outer;
            }
        }
        ctr[dir] += 2;
        dir = (dir + 1) % 4;
    }
    return curr;
}

#[cfg(test)]
mod day03_tests {
    use crate::y2017::day03::{part_a, part_b};
    use std::fs;

    #[test]
    fn part_a_works() {
        assert_eq!(0, part_a(1));
        assert_eq!(3, part_a(12));
        assert_eq!(2, part_a(23));
        assert_eq!(31, part_a(1024));
    }

    #[test]
    fn part_b_works() {
        assert_eq!(2, part_b(1));
        assert_eq!(4, part_b(2));
        assert_eq!(26, part_b(25));
        assert_eq!(54, part_b(26));
    }

    #[test]
    fn input_works() {
        let input = fs::read_to_string("input/2017/day03/input.txt")
            .unwrap()
            .trim()
            .parse::<i32>()
            .unwrap();
        assert_eq!(326, part_a(input));
        assert_eq!(363010, part_b(input));
    }
}
