use std::collections::{HashMap, HashSet};
use std::fs;

pub(crate) fn day07() {
    let input = fs::read_to_string("input/2017/day07/input.txt").unwrap();
    let (adjacency_list, weights) = parse(input.as_str());
    let sort = part_a(&adjacency_list);
    println!("{}", sort.last().unwrap());
    println!("{}", part_b(sort, &adjacency_list, &weights));
}

fn parse<'a>(input: &'a str) -> (HashMap<&'a str, Vec<&'a str>>, HashMap<&'a str, i32>) {
    let mut adjacency_list: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut weights: HashMap<&str, i32> = HashMap::new();
    input.lines().for_each(|line| {
        let (name_str, children_str) = if line.contains(" -> ") {
            line.split_once(" -> ").unwrap()
        } else {
            (line, "")
        };
        let (name, weight_str) = name_str.split_once(" (").unwrap();
        adjacency_list.insert(
            name,
            if children_str.len() > 0 {
                children_str.split(", ").collect()
            } else {
                vec![]
            },
        );
        let weight = weight_str[0..weight_str.len() - 1].parse().unwrap();
        weights.insert(&name, weight);
    });
    (adjacency_list, weights)
}

fn part_a<'a>(adjacency_list: &HashMap<&'a str, Vec<&'a str>>) -> Vec<&'a str> {
    let mut visited = HashSet::new();
    let mut sort = vec![];
    for name in adjacency_list.keys() {
        topological_sort_rec(*name, adjacency_list, &mut visited, &mut sort);
    }
    sort
}

fn topological_sort_rec<'a>(
    name: &'a str,
    adjacency_list: &HashMap<&'a str, Vec<&'a str>>,
    visited: &mut HashSet<&'a str>,
    sort: &mut Vec<&'a str>,
) {
    if !visited.insert(name) {
        return;
    }

    let neighbors = adjacency_list.get(name).unwrap();
    if neighbors.is_empty() {
        sort.push(name);
        return;
    }

    for neighbor in neighbors {
        topological_sort_rec(neighbor, adjacency_list, visited, sort);
    }
    sort.push(name);
}

fn part_b(
    sort: Vec<&str>,
    adjacency_list: &HashMap<&str, Vec<&str>>,
    weights: &HashMap<&str, i32>,
) -> i32 {
    let mut balance: HashMap<&str, i32> = HashMap::new();
    for name in sort {
        // If we go in the topological sort order, we are guaranteed not to get unwrap on None in the balance map
        let mut weight_sum = 0;
        let mut weight_map = HashMap::new();
        adjacency_list
            .get(name)
            .unwrap()
            .iter()
            .for_each(|neighbor| {
                let w = *balance.get(neighbor).unwrap();
                weight_map.entry(w).or_insert(vec![]).push(*neighbor);
                weight_sum += w;
            });

        if weight_map.len() > 1 {
            let (unbalanced_w_sum, unbalanced_n) = weight_map
                .iter()
                .filter(|(_, c)| c.len() == 1)
                .last()
                .unwrap();
            let balanced_w_sum = weight_map
                .iter()
                .filter(|(_, c)| c.len() > 1)
                .map(|(w, _)| w)
                .last()
                .unwrap();
            let unbalanced_w = weights.get(unbalanced_n.last().unwrap()).unwrap();
            return unbalanced_w - (unbalanced_w_sum - balanced_w_sum);
        }

        let weight = weights.get(name).unwrap();
        balance.insert(name, weight + weight_sum);
    }
    panic!("No unbalanced discs")
}

#[cfg(test)]
mod day07_tests {
    use crate::y2017::day07::{parse, part_a, part_b};
    use std::fs;

    #[test]
    fn test_works() {
        let input = fs::read_to_string("input/2017/day07/test.txt").unwrap();
        let (adjacency_list, weights) = parse(input.as_str());
        let topological = part_a(&adjacency_list);
        assert_eq!("tknk", *topological.last().unwrap());
        assert_eq!(60, part_b(topological, &adjacency_list, &weights));
    }

    #[test]
    fn input_works() {
        let input = fs::read_to_string("input/2017/day07/input.txt").unwrap();
        let (adjacency_list, weights) = parse(input.as_str());
        let topological = part_a(&adjacency_list);
        assert_eq!("vmpywg", *topological.last().unwrap());
        assert_eq!(1674, part_b(topological, &adjacency_list, &weights));
    }
}
