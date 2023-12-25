use std::fs::File;
use std::io::{BufRead, BufReader};

pub(crate) fn day01() {
    let file = File::open("input/2019/day01/input.txt");
    let reader = BufReader::new(file.unwrap());

    let mut modules: Vec<u32> = vec!();
    for line in reader.lines() {
        modules.push(line.unwrap().parse().unwrap());
    }

    let part_a: u32 = modules.iter()
        .map(|m| (m / 3) - 2)
        .sum();
    println!("{}", part_a);

    let part_b: u32 = modules.iter()
        .map(|m| calculate_fuel(*m))
        .sum();
    println!("{}", part_b);
}

fn calculate_fuel(mut module: u32) -> u32 {
    let mut total = 0;
    while module > 6 {
        let fuel = (module / 3) - 2;
        total += fuel;
        module = fuel;
    }
    return total;
}
