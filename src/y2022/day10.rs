use std::collections::HashSet;
use std::fs;

pub(crate) fn day10() {
    let input = fs::read_to_string("input/2022/day10/input.txt").unwrap();

    let mut sum = 0;
    let mut cycle = 0;
    let mut reg = 1;
    let mut pixels = [['.'; 40]; 6];

    for line in input.lines() {
        if line.starts_with("addx") {
            let split = line.split_once(' ').unwrap();
            let val: i32 = split.1.parse().unwrap();

            cycle += 1;
            sum = add_to_sum(sum, cycle, reg);
            pixels = add_to_pixels(pixels, cycle, reg);

            cycle += 1;
            sum = add_to_sum(sum, cycle, reg);
            pixels = add_to_pixels(pixels, cycle, reg);
            reg += val;
        } else {
            cycle += 1;
            sum = add_to_sum(sum, cycle, reg);
            pixels = add_to_pixels(pixels, cycle, reg);
        }
    }

    println!("{}", sum);

    for i in 0..6 {
        for j in 0..40 {
            print!("{}", pixels[i][j])
        }
        println!();
    }
}

fn add_to_sum(sum: i32, cycle: i32, reg: i32) -> i32 {
    let mut interested: HashSet<i32> = HashSet::new();
    interested.insert(20);
    interested.insert(60); // <-- ???
    interested.insert(100);
    interested.insert(140);
    interested.insert(180);
    interested.insert(220);

    if interested.contains(&cycle) {
        return sum + (cycle * reg)
    } else {
        return sum
    }
}

fn add_to_pixels(pixels: [[char; 40]; 6], cycle: i32, reg: i32) -> [[char; 40]; 6] {
    let mut res = pixels.clone();
    let pixel = (cycle - 1) % 40;

    if reg >= pixel - 1 && reg <= pixel + 1 {
        let x = ((cycle - 1) / 40) as usize;
        let y = pixel as usize;
        res[x][y] = '#';
    }

    return res;
}
