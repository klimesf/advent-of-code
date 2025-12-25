use itertools::Itertools;
use std::fs;

pub(crate) fn day23() {
    println!("{}", part_a(fs::read_to_string("input/2020/day23/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2020/day23/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    let cups = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>();

    let len = cups.len();
    let mut current = cups[0];
    let mut links = vec![0; len + 1];

    cups.iter().circular_tuple_windows().for_each(|(c1, c2)| {
        links[*c1] = *c2;
    });

    for _ in 0..100 {
        let r1 = links[current];
        let r2 = links[r1];
        let r3 = links[r2];

        links[current] = links[r3];

        let rs = [r1, r2, r3];
        let mut label = current;
        loop {
            label = label - 1;
            if label == 0 {
                label = len;
            }
            if !rs.contains(&label) {
                break;
            }
        }

        let swap = links[label];
        links[label] = r1;
        links[r3] = swap;

        current = links[current];
    }

    let mut s = String::new();
    let mut b = links[1];
    while b != 1 {
        s.push_str(b.to_string().as_str());
        b = links[b];
    }
    s.parse::<usize>().unwrap()
}

fn part_b(input: String) -> usize {
    let mut cups = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>();
    cups.reserve(1_000_000 - cups.len());
    for i in cups.len() + 1..=1_000_000 {
        cups.push(i);
    }
    let len = cups.len();
    let mut current = cups[0];
    let mut links = vec![0; len + 1];

    cups.iter().circular_tuple_windows().for_each(|(c1, c2)| {
        links[*c1] = *c2;
    });

    for _ in 0..10_000_000 {
        let r1 = links[current];
        let r2 = links[r1];
        let r3 = links[r2];

        links[current] = links[r3];

        let rs = [r1, r2, r3];
        let mut label = current;
        loop {
            label = label - 1;
            if label == 0 {
                label = len;
            }
            if !rs.contains(&label) {
                break;
            }
        }

        let swap = links[label];
        links[label] = r1;
        links[r3] = swap;

        current = links[current];
    }

    links[1] * links[links[1]]
}

#[cfg(test)]
mod day23_tests {
    use std::fs;

    use crate::y2020::day23::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(67384529, part_a(fs::read_to_string("input/2020/day23/test.txt").unwrap()));
        assert_eq!(149245887792, part_b(fs::read_to_string("input/2020/day23/test.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(58427369, part_a(fs::read_to_string("input/2020/day23/input.txt").unwrap()));
        assert_eq!(111057672960, part_b(fs::read_to_string("input/2020/day23/input.txt").unwrap()));
    }
}
