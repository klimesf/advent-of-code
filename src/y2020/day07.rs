use std::collections::{HashMap, HashSet};
use std::fs;

pub(crate) fn day07() {
    println!("{}", part_a(fs::read_to_string("input/2020/day07/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2020/day07/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    let mut reverse_map = HashMap::new();
    input.lines().for_each(|line| {
        let (from, r) = line.split_once(" bags contain ").unwrap();
        r[..r.len() - 1]
            .split(", ")
            .filter(|ah| *ah != "no other bags")
            .map(|h| {
                let x = h.split_whitespace().collect::<Vec<&str>>();
                x[1..x.len() - 1].join(" ")
            })
            .for_each(|to| {
                reverse_map.entry(to).or_insert_with(Vec::new).push(from);
            });
    });

    let mut visited = HashSet::new();
    let mut stack = vec![];
    stack.push("shiny gold");

    while let Some(vertex) = stack.pop() {
        if !visited.insert(vertex) {
            continue;
        }
        match reverse_map.get(&vertex.to_string()) {
            Some(tos) => {
                tos.iter().for_each(|to| stack.push(to));
            }
            None => {}
        }
    }

    visited.len() - 1 // Do not count shiny gold itself
}

fn part_b(input: String) -> usize {
    let map: HashMap<String, Vec<(usize, String)>> = input
        .lines()
        .map(|line| {
            let (from, r) = line.split_once(" bags contain ").unwrap();
            let to = r[..r.len() - 1]
                .split(", ")
                .filter(|ah| *ah != "no other bags")
                .map(|h| {
                    let x = h.split_whitespace().collect::<Vec<&str>>();
                    (x[0].parse().unwrap(), x[1..x.len() - 1].join(" "))
                })
                .collect::<Vec<(usize, String)>>();
            (from.to_string(), to)
        })
        .collect();

    recursive_count("shiny gold".to_string(), &map) - 1 // Do not count shiny gold itself
}

fn recursive_count(pos: String, map: &HashMap<String, Vec<(usize, String)>>) -> usize {
    let tos = map.get(&pos).unwrap();
    if tos.is_empty() {
        return 1;
    }

    // Count in the actual bag itself, plus every one it contains
    tos.iter()
        .map(|(count, to)| count * recursive_count(to.clone(), map))
        .sum::<usize>()
        + 1
}

#[cfg(test)]
mod day07_tests {
    use std::fs;

    use crate::y2020::day07::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(4, part_a(fs::read_to_string("input/2020/day07/test.txt").unwrap()));
        assert_eq!(32, part_b(fs::read_to_string("input/2020/day07/test.txt").unwrap()));
        assert_eq!(126, part_b(fs::read_to_string("input/2020/day07/test_2.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(179, part_a(fs::read_to_string("input/2020/day07/input.txt").unwrap()));
        assert_eq!(18925, part_b(fs::read_to_string("input/2020/day07/input.txt").unwrap()));
    }
}
