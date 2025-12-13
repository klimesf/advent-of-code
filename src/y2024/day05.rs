use std::collections::{HashMap, HashSet};
use std::fs;

pub fn day05(print: fn(usize)) {
    let (part_a, part_b) = solve(fs::read_to_string("input/2024/day05/input.txt").unwrap());
    print(part_a);
    print(part_b);
}

fn solve(input: String) -> (usize, usize) {
    let (rules_input, order_input) = input.split_once("\n\n").unwrap();

    let mut rules: HashMap<usize, Vec<usize>> = HashMap::new();
    rules_input
        .lines()
        .map(|line| line.split_once("|").unwrap())
        .map(|(rule, output)| {
            (
                rule.parse::<usize>().unwrap(),
                output.parse::<usize>().unwrap(),
            )
        })
        .for_each(|(before, after)| {
            rules.entry(before).or_insert_with(Vec::new).push(after);
        });

    let mut to_reorder = vec![];
    let part_a = order_input
        .lines()
        .map(|line| {
            let pages = line
                .split(",")
                .map(|page| page.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            for i in 0..pages.len() {
                let page = pages[i];
                if !rules.contains_key(&page) {
                    continue;
                }
                let rule = rules.get(&page).unwrap();

                for j in 0..i {
                    let after = pages[j];
                    if rule.contains(&after) {
                        to_reorder.push(pages);
                        return 0;
                    }
                }
            }

            pages[pages.len() / 2]
        })
        .sum();

    let part_b = to_reorder
        .iter()
        .map(|line| {
            let mut visited = HashSet::new();
            let mut sort = vec![];
            for i in 0..line.len() {
                let page = line[i];
                // The sort is the other way around here,
                // but we are interested only in the middle element
                topological_sort(&page, &rules, &mut visited, &mut sort, &line);
            }

            sort[sort.len() / 2]
        })
        .sum();

    (part_a, part_b)
}

fn topological_sort(
    name: &usize,
    adjacency_list: &HashMap<usize, Vec<usize>>,
    visited: &mut HashSet<usize>,
    sort: &mut Vec<usize>,
    line: &Vec<usize>,
) {
    if !line.contains(&name) {
        // Little edit from the standard topological sort to not include all vertices in the ruleset
        return;
    }
    if !visited.insert(*name) {
        return;
    }
    if !adjacency_list.contains_key(name) {
        sort.push(*name);
        return;
    }

    let neighbors = adjacency_list.get(name).unwrap();
    if neighbors.is_empty() {
        sort.push(*name);
        return;
    }

    for neighbor in neighbors {
        topological_sort(neighbor, adjacency_list, visited, sort, line);
    }
    sort.push(*name);
}

#[cfg(test)]
mod day05_tests {
    use std::fs;

    use crate::y2024::day05::solve;

    #[test]
    fn test_works() {
        assert_eq!(
            (143, 123),
            solve(fs::read_to_string("input/2024/day05/test.txt").unwrap())
        );
    }

    #[test]
    fn input_works() {
        assert_eq!(
            (6242, 5169),
            solve(fs::read_to_string("input/2024/day05/input.txt").unwrap())
        );
    }
}
