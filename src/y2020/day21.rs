use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs;

pub(crate) fn day21() {
    let (part_a, part_b) = solve(fs::read_to_string("input/2020/day21/input.txt").unwrap());
    println!("{}", part_a);
    println!("{}", part_b);
}

fn solve(input: String) -> (usize, String) {
    let foods = input
        .lines()
        .map(|line| {
            let (l, r) = line.split_once(" (contains ").unwrap();
            let ingredients = l.split_whitespace().collect::<Vec<_>>();
            let allergens = r[..r.len() - 1].split(", ").collect::<Vec<_>>();
            (ingredients, allergens)
        })
        .collect::<Vec<(Vec<&str>, Vec<&str>)>>();

    let mut allergen2ingredients: HashMap<&str, Vec<HashSet<&str>>> = HashMap::new();
    for (ingredients, allergens) in &foods {
        for allergen in allergens {
            allergen2ingredients
                .entry(allergen)
                .or_insert(Vec::new())
                .push(ingredients.iter().map(|s| *s).collect());
        }
    }

    let mut allergen_assignment: HashMap<&str, &str> = HashMap::new();
    while allergen_assignment.len() < allergen2ingredients.len() {
        allergen2ingredients.keys().for_each(|allergen| {
            let intersection = allergen2ingredients[allergen]
                .iter()
                .map(|ingredients| {
                    ingredients
                        .iter()
                        .cloned()
                        .filter(|ingredient| !allergen_assignment.values().contains(ingredient))
                        .collect::<HashSet<_>>()
                })
                .reduce(|a, b| a.intersection(&b).cloned().collect())
                .unwrap();

            if intersection.len() == 1 {
                allergen_assignment.insert(allergen, intersection.iter().next().unwrap());
            }
        });
    }

    let part_a = foods
        .iter()
        .map(|(ingredients, _)| {
            ingredients
                .iter()
                .filter(|ingredient| !allergen_assignment.values().contains(ingredient))
                .count()
        })
        .sum::<usize>();

    let part_b = allergen_assignment
        .iter()
        .sorted_by(|a, b| a.0.cmp(&b.0))
        .map(|(_, ingredient)| ingredient)
        .join(",");

    (part_a, part_b)
}

#[cfg(test)]
mod day21_tests {
    use std::fs;

    use crate::y2020::day21::solve;

    #[test]
    fn test_works() {
        assert_eq!(
            (5, "mxmxvkd,sqjhc,fvjkl".to_string()),
            solve(fs::read_to_string("input/2020/day21/test.txt").unwrap())
        );
    }

    #[test]
    fn input_works() {
        assert_eq!(
            (2734, "kbmlt,mrccxm,lpzgzmk,ppj,stj,jvgnc,gxnr,plrlg".to_string()),
            solve(fs::read_to_string("input/2020/day21/input.txt").unwrap())
        );
    }
}
