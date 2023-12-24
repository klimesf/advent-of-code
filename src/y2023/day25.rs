use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fs;
use itertools::Itertools;

pub(crate) fn day25() {
    println!("{}", part_a(fs::read_to_string("input/2023/day25/input.txt").unwrap()));
}

fn part_a(input: String) -> i32 {
    let mut edges = HashSet::new();
    let mut nodes = HashSet::new();
    let mut graph = HashMap::new();

    input.lines().for_each(|line| {
        let (from, to_list) = line.split_once(": ").unwrap();
        to_list.split_whitespace().for_each(|to| {
            edges.insert((from, to));
            nodes.insert(from);
            nodes.insert(to);
            graph.entry(from).or_insert(HashSet::new()).insert(to);
            graph.entry(to).or_insert(HashSet::new()).insert(from);
        });
    });

    // Run BFS from each node
    let mut edge_ctr = HashMap::new();
    for node in &nodes {
        let mut stack = VecDeque::new();
        let mut visited = HashSet::new();
        stack.push_back((*node, ("start", *node)));

        while let Some((pos, (from, to))) = stack.pop_front() {
            if !visited.insert(pos) { continue }
            *edge_ctr.entry((from, to)).or_insert(0) += 1;
            *edge_ctr.entry((to, from)).or_insert(0) += 1;

            for to in graph.get(pos).unwrap() {
                stack.push_back((to, (pos, to)));
            }
        }
    }

    let candidates = edge_ctr.iter()
        .filter(|((from, to), _)| from != &"start" && to != &"start")
        .sorted_by(|(_, ctr_a), (_, ctr_b)| ctr_b.cmp(ctr_a))
        .collect_vec();

    let mut heap = BinaryHeap::new();
    for i in 0..10 {
        for j in 0..10 { // candidates.len() {
            if i == j { continue }
            for k in 0..10 {
                if j == k { continue }
                let (_, ic) = candidates[i];
                let (_, jc) = candidates[j];
                let (_, kc) = candidates[k];
                heap.push((ic  + jc  + kc, i, j, k))
            }
        }
    }

    for (_, i, j, k) in heap {
        let mut new_graph = graph.clone();
        for x in [i, j, k] {
            let ((from, to), _) = candidates[x];
            new_graph.get_mut(from).unwrap().remove(to);
            new_graph.get_mut(to).unwrap().remove(from);
        }

        let mut component = HashMap::new();
        let mut i = 0;
        for node in &nodes {
            let mut stack = vec!();
            stack.push(node);
            while let Some(pos) = stack.pop() {
                if component.contains_key(pos) { continue }
                component.insert(pos, i);
                for to in new_graph.get(pos).unwrap() {
                    stack.push(to);
                }
            }
            i += 1;
        }

        let mut component_size = HashMap::new();
        for (_, cmp) in &component {
            *component_size.entry(*cmp).or_insert(0) += 1;
        }

        if component_size.len() == 2 {
            return component_size.values().product()
        }
    }

    panic!()
}

#[cfg(test)]
mod day25_tests {
    use std::fs;

    use crate::y2023::day25::{part_a};

    #[test]
    fn test_works() {
        assert_eq!(54, part_a(fs::read_to_string("input/2023/day25/test.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(527790, part_a(fs::read_to_string("input/2023/day25/input.txt").unwrap()));
    }
}
