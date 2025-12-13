use std::fs;

pub(crate) fn day04() {
    let mut total_a = 0;
    let mut total_b = 0;

    let input = fs::read_to_string("input/2022/day04/input.txt").unwrap();
    for line in input.lines() {
        let elves = line.split_once(',').unwrap();
        let elf1 = elves.0.split_once('-').unwrap();
        let elf2 = elves.1.split_once('-').unwrap();
        let elf1start = elf1.0.parse::<i32>().unwrap();
        let elf1end = elf1.1.parse::<i32>().unwrap();
        let elf2start = elf2.0.parse::<i32>().unwrap();
        let elf2end = elf2.1.parse::<i32>().unwrap();

        if fully_contains(elf1start, elf1end, elf2start, elf2end) {
            total_a += 1;
        }
        if overlaps(elf1start, elf1end, elf2start, elf2end) {
            total_b += 1;
        }
    }

    println!("{}", total_a);
    println!("{}", total_b);
}

fn fully_contains(start1: i32, end1: i32, start2: i32, end2: i32) -> bool {
    if start1 <= start2 && end1 >= end2 {
        return true;
    } else if start1 >= start2 && end1 <= end2 {
        return true;
    } else {
        return false;
    }
}

fn overlaps(start1: i32, end1: i32, start2: i32, end2: i32) -> bool {
    if start1 <= start2 && end1 >= start2 {
        return true;
    } else if start2 <= start1 && end2 >= start1 {
        return true;
    } else {
        return false;
    }
}
