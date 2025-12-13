use std::collections::{HashMap, HashSet};
use std::fs;

pub(crate) fn day12() {
    let input = fs::read_to_string("input/2018/day12/input.txt").unwrap();
    let (initial, raw_rules) = input.trim().split_once("\n\n").unwrap();

    let mut rules: HashMap<String, char> = HashMap::new();
    raw_rules
        .trim()
        .split("\n")
        .into_iter()
        .map(|c| c.split_once(" => ").unwrap())
        .for_each(|(pattern, result)| {
            rules.insert(pattern.to_string(), result.chars().nth(0).unwrap());
        });

    let mut population: HashMap<i32, char> = HashMap::new();
    for i in 15..initial.len() {
        population.insert(i as i32 - 15, initial.chars().nth(i).unwrap());
    }

    let mut generation = 0;
    for _ in 0..20 {
        population = calc_new_pop(&population, &rules);
        generation += 1;
    }

    println!(
        "After {} generations, the indexes of the pots with flower sum up to {}",
        generation,
        sum_pop(&population)
    );

    let mut visited: HashSet<String> = HashSet::new();
    let start;
    loop {
        population = calc_new_pop(&population, &rules);
        generation += 1;
        cut_off(&mut population);
        if !visited.insert(pop_to_str(&population)) {
            println!(
                "The cycle starts to repeat after {} generations",
                generation
            );
            start = sum_pop(&population);
            break;
        }
    }
    let end = sum_pop(&calc_new_pop(&population, &rules));
    generation += 1;

    let inc = (end - start) as i64;
    println!(
        "Afterwards, each new generation adds {} to the pot flower sum",
        inc
    );

    let res = end as i64 + (50000000000 - generation) * inc;
    println!(
        "After 50000000000 generations, the indexes of the pots with flower sum up to {}",
        res
    );
}

fn calc_new_pop(
    population: &HashMap<i32, char>,
    rules: &HashMap<String, char>,
) -> HashMap<i32, char> {
    let mut new_pop = HashMap::new();
    let from = *population.keys().min().unwrap();
    let to = *population.keys().max().unwrap();

    for i in from - 3..=to + 3 {
        let mut pattern = String::new();
        for j in i - 2..=i + 2 {
            match population.get(&j) {
                Some(v) => pattern.push(*v),
                None => pattern.push('.'),
            };
        }
        let res = *rules.get(&pattern).unwrap();
        new_pop.insert(i, res);
    }

    return new_pop;
}

fn cut_off(population: &mut HashMap<i32, char>) {
    let from = *population.keys().min().unwrap();
    let to = *population.keys().max().unwrap();
    let mut i = from;
    let mut c = *population.get(&i).unwrap();
    while c == '.' {
        population.remove(&i);
        i += 1;
        c = *population.get(&i).unwrap();
    }
    i = to;
    c = *population.get(&i).unwrap();
    while c == '.' {
        population.remove(&i);
        i -= 1;
        c = *population.get(&i).unwrap();
    }
}

fn sum_pop(population: &HashMap<i32, char>) -> i32 {
    population
        .iter()
        .map(|(i, c)| if *c == '#' { *i } else { 0 })
        .sum::<i32>()
}

fn pop_to_str(population: &HashMap<i32, char>) -> String {
    let from = *population.keys().min().unwrap();
    let to = *population.keys().max().unwrap();
    let mut s = String::new();
    for i in from..=to {
        s.push(*population.get(&i).unwrap());
    }
    return s;
}
