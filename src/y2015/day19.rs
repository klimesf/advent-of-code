use std::collections::{HashMap, HashSet};
use std::fs;

pub(crate) fn day19() {
    println!(
        "{}",
        part_a(fs::read_to_string("input/2015/day19/input.txt").unwrap())
    );
    println!(
        "{}",
        part_b(fs::read_to_string("input/2015/day19/input.txt").unwrap())
    );
}

fn part_a(input: String) -> usize {
    let (repls, mut mols) = input.split_once("\n\n").unwrap();
    mols = mols.trim();
    let mut replacements = HashMap::new();
    repls.lines().for_each(|line| {
        let (from, to) = line.split_once(" => ").unwrap();
        replacements.entry(from).or_insert(vec![]).push(to);
    });

    let mut ans = HashSet::new();

    let mut buf = String::new();
    let mut ptr = 0;
    let molecules: Vec<char> = mols.chars().collect();
    while ptr < molecules.len() {
        buf.push(molecules[ptr]);

        for (key, vals) in &replacements {
            if buf.ends_with(key) {
                for val in vals {
                    ans.insert(format!(
                        "{}{}{}",
                        mols[0..(ptr + 1 - key.len())].to_string(),
                        val,
                        mols[ptr + 1..].to_string()
                    ));
                }
                buf.clear();
                break;
            }
        }
        ptr += 1;
    }

    ans.len()
}

fn part_b(input: String) -> usize {
    let (repls, mut orig) = input.split_once("\n\n").unwrap();
    orig = orig.trim();
    let mut replacements = HashMap::new();
    repls.lines().for_each(|line| {
        let (from, to) = line.split_once(" => ").unwrap();
        replacements.entry(to).or_insert(vec![]).push(from);
    });

    let mut min = usize::MAX;

    let mut stack = vec![];
    stack.push((0, orig.to_string()));

    while let Some((steps, mols)) = stack.pop() {
        if mols.as_str() == "e" {
            min = steps.min(min);
            return min;
        }

        let mut buf = String::new();
        let mut ptr = 0;
        let molecules: Vec<char> = mols.chars().collect();
        while ptr < molecules.len() {
            buf.push(molecules[ptr]);

            for (key, vals) in &replacements {
                if buf.ends_with(key) {
                    for val in vals {
                        let next = format!(
                            "{}{}{}",
                            mols[0..(ptr + 1 - key.len())].to_string(),
                            val,
                            mols[ptr + 1..].to_string()
                        );
                        stack.push((steps + 1, next));
                    }
                }
            }
            ptr += 1;
        }
    }

    min
}

#[cfg(test)]
mod day19_tests {
    use std::fs;

    use crate::y2015::day19::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(
            4,
            part_a(fs::read_to_string("input/2015/day19/test.txt").unwrap())
        );
        assert_eq!(
            7,
            part_a(fs::read_to_string("input/2015/day19/test_b.txt").unwrap())
        );
        assert_eq!(
            3,
            part_b(fs::read_to_string("input/2015/day19/test.txt").unwrap())
        );
        assert_eq!(
            6,
            part_b(fs::read_to_string("input/2015/day19/test_b.txt").unwrap())
        );
    }

    #[test]
    fn input_works() {
        assert_eq!(
            518,
            part_a(fs::read_to_string("input/2015/day19/input.txt").unwrap())
        );
        assert_eq!(
            200,
            part_b(fs::read_to_string("input/2015/day19/input.txt").unwrap())
        ); // TODO: dijkstra
    }
}
