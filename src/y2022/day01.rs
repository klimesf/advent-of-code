use std::collections::BinaryHeap;
use crate::utils::io::read_lines;

pub(crate) fn day01() {
    let mut elves = BinaryHeap::new();
    let mut current_elf: i32 = 0;

    if let Ok(lines) = read_lines("input/2022/day01/input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                if ip.len() == 0 {
                    elves.push(current_elf);
                    current_elf = 0;
                } else {
                    current_elf += ip.parse::<i32>().unwrap();
                }
            }
        }
        elves.push(current_elf)
    }

    part_a(&elves);
    part_b(&mut elves);
}

fn part_a(elves: &BinaryHeap<i32>) {
    println!("{}", elves.peek().unwrap())
}

fn part_b(elves: &mut BinaryHeap<i32>) {
    let max1 = elves.pop().unwrap();
    let max2 = elves.pop().unwrap();
    let max3 = elves.pop().unwrap();

    println!("{}", max1 + max2 + max3)
}
