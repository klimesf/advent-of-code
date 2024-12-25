use std::collections::HashMap;
use std::fs;
use itertools::Itertools;

pub fn day23(print_usize: fn(usize), print_string: fn(String)) {
    print_usize(part_a(fs::read_to_string("input/2024/day23/input.txt").unwrap()));
    print_string(part_b(fs::read_to_string("input/2024/day23/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    let mut adjacency_list: HashMap<&str, Vec<&str>> = HashMap::new();
    input.lines().for_each(|line| {
        let (l, r) = line.split_once('-').unwrap();
        adjacency_list.entry(l).or_insert(vec!()).push(r);
        adjacency_list.entry(r).or_insert(vec!()).push(l);
    });

    let keys = adjacency_list.keys().cloned().collect::<Vec<&str>>();

    keys.iter()
        .filter(|k| adjacency_list[*k].len() >= 2)
        .filter(|k| k.starts_with("t"))
        .map(|k| {
            adjacency_list[*k].iter().combinations(2)
                .filter(|comb|
                    adjacency_list[*k].contains(comb[0])
                        && adjacency_list[*k].contains(comb[1])
                        && adjacency_list[comb[0]].contains(k)
                        && adjacency_list[comb[0]].contains(comb[1])
                        && adjacency_list[comb[1]].contains(k)
                        && adjacency_list[comb[1]].contains(comb[0])
                )
                .map(|v| {
                    let mut comb = v.clone();
                    comb.push(k);
                    comb.sort_unstable();
                    comb
                })
                .collect::<Vec<Vec<&&str>>>()
        })
        .flatten()
        .unique()
        .count()
}

fn part_b(input: String) -> String {
    let mut adjacency_list: HashMap<&str, Vec<&str>> = HashMap::new();
    input.lines().for_each(|line| {
        let (l, r) = line.split_once('-').unwrap();
        adjacency_list.entry(l).or_insert(vec!()).push(r);
        adjacency_list.entry(r).or_insert(vec!()).push(l);
    });
    let keys = adjacency_list.keys().cloned().sorted().collect::<Vec<&str>>();

    let mut max = usize::MIN;
    let mut max_clique = vec!();

    for i in 0..keys.len() {
        let from = keys[i];
        let mut clique = vec![from];
        for j in i..keys.len() {
            let to = keys[j];
            if clique.iter().all(|c| adjacency_list[to].contains(c)) {
                clique.push(to);
            }
        }
        if clique.len() > max {
            max = clique.len();
            max_clique = clique;
        }
    }

    max_clique.iter().join(",")
}

#[cfg(test)]
mod day23_tests {
    use std::fs;

    use crate::y2024::day23::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(7, part_a(fs::read_to_string("input/2024/day23/test.txt").unwrap()));
        assert_eq!("co,de,ka,ta", part_b(fs::read_to_string("input/2024/day23/test.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(1304, part_a(fs::read_to_string("input/2024/day23/input.txt").unwrap()));
        assert_eq!("ao,es,fe,if,in,io,ky,qq,rd,rn,rv,vc,vl", part_b(fs::read_to_string("input/2024/day23/input.txt").unwrap()));
    }
}
