use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs;

pub(crate) fn day25() {
    println!(
        "{}",
        part_a(fs::read_to_string("input/2018/day25/input.txt").unwrap())
    );
}

fn part_a(input: String) -> usize {
    let stars: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.splitn(4, ",")
                .map(|n| n.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();

    for i in 0..stars.len() {
        for j in 0..stars.len() {
            if i == j {
                continue;
            }
            if manhattan_dist(&stars[i], &stars[j]) <= 3 {
                graph.entry(i).or_insert(vec![]).push(j);
            }
        }
    }

    let mut constellation = vec![0; stars.len()];
    for i in 0..stars.len() {
        constellation[i] = i;
    }

    let mut visited = HashSet::new();
    let mut stack = vec![];
    for i in 0..stars.len() {
        stack.push((i, i));
    }

    while let Some((star, component)) = stack.pop() {
        if !visited.insert(star) {
            continue;
        }
        constellation[star] = component;

        for target in graph.get(&star).unwrap_or(&vec![]) {
            stack.push((*target, component));
        }
    }

    constellation.iter().unique().count()
}

fn manhattan_dist(a: &Vec<i32>, b: &Vec<i32>) -> i32 {
    let mut ans = 0;
    for i in 0..a.len() {
        ans += a[i].max(b[i]) - a[i].min(b[i]);
    }
    ans
}

#[cfg(test)]
mod day25_tests {
    use std::fs;

    use crate::y2018::day25::part_a;

    #[test]
    fn test_works() {
        assert_eq!(
            2,
            part_a(fs::read_to_string("input/2018/day25/test_2.txt").unwrap())
        );
        assert_eq!(
            3,
            part_a(fs::read_to_string("input/2018/day25/test_3.txt").unwrap())
        );
        assert_eq!(
            4,
            part_a(fs::read_to_string("input/2018/day25/test_4.txt").unwrap())
        );
        assert_eq!(
            8,
            part_a(fs::read_to_string("input/2018/day25/test_8.txt").unwrap())
        );
    }

    #[test]
    fn input_works() {
        assert_eq!(
            394,
            part_a(fs::read_to_string("input/2018/day25/input.txt").unwrap())
        );
    }
}
