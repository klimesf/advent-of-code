use std::collections::{HashSet};
use std::{fs, mem};

pub(crate) fn day23() {
    let input = fs::read_to_string("input/2022/day23/input.txt").unwrap();
    let mut map = HashSet::new();
    let mut new_map = HashSet::new();
    map.reserve(input.len());
    new_map.reserve(input.len());
    input.lines().enumerate().for_each(|(x, line)| line.chars().enumerate()
        .for_each(|(y, c)| {
            match c {
                '#' => { map.insert((x as i32, y as i32)); },
                _ => { }
            };
        }));

    let all_dirs = [(1, 0), (-1, 0), (0, 1), (0, -1), (1, -1), (1, 1), (-1, 1), (-1, -1)];
    let north_dirs = ([(-1, 0), (-1, -1), (-1, 1)], (-1, 0));
    let south_dirs = ([(1, 0), (1, -1), (1, 1)], (1, 0));
    let west_dirs = ([(0, -1), (1, -1), (-1, -1)], (0, -1));
    let east_dirs = ([(0, 1), (1, 1), (-1, 1)], (0, 1));
    let ordnung = [north_dirs, south_dirs, west_dirs, east_dirs];

    let mut i = 0;
    loop {
        let mut move_ctr = 0;
        new_map.clear();

        'outer: for elf in &map {
            if all_dirs.iter().any(|(dx, dy)| map.contains(&(elf.0 + dx, elf.1 + dy))) {
                for j in i..i + 4 {
                    let (dirs, mv) = ordnung[j % 4];
                    if dirs.iter().all(|(dx, dy)| !map.contains(&(elf.0 + dx, elf.1 + dy))) {
                        let new_pos = (elf.0 + mv.0, elf.1 + mv.1);
                        if new_map.contains(&(new_pos)) {
                            // There is another elf in the position, so we cannot move there
                            // He must have come from the opposite direction, so push him back
                            // Note: we can disregard the proposed directions, because only collision can happen
                            //       from the opposite direction by single elf
                            move_ctr -= 1;
                            new_map.remove(&new_pos);
                            new_map.insert((new_pos.0 + mv.0, new_pos.1 + mv.1));
                            new_map.insert(*elf);
                        } else {
                            move_ctr += 1;
                            new_map.insert(new_pos);
                        }
                        continue 'outer;
                    }
                }
            }
            new_map.insert(*elf);
        }

        mem::swap(&mut map, &mut new_map);

        i += 1;
        if i == 10 { // Part 1
            get_empty_space(&map);
        }
        if move_ctr == 0 { // Part 2
            println!("{}", i);
            break;
        }
    }
}

fn get_empty_space(map: &HashSet<(i32, i32)>) {
    let max_x = map.iter().map(|(x, _)| *x).max().unwrap();
    let min_x = map.iter().map(|(x, _)| *x).min().unwrap();
    let max_y = map.iter().map(|(_, y)| *y).max().unwrap();
    let min_y = map.iter().map(|(_, y)| *y).min().unwrap();
    let mut ans = 0;
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if !map.contains(&(x, y)) { ans += 1 };
        }
    }
    println!("{}", ans);
}

#[allow(dead_code)]
fn print_map(map: &HashSet<(i32, i32)>) {
    for x in -2..=9 {
        for y in -3..=10 {
            if !map.contains(&(x, y)) { print!(".") } else { print!("#") };
        }
        println!()
    }
    println!();
}

