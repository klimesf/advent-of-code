use std::collections::HashMap;
use std::fs;

pub(crate) fn day06() {
    let input = fs::read_to_string("input/2018/day06/input.txt").unwrap();
    let coords: Vec<(usize, usize)> = input.trim().split("\n")
        .map(|coord| coord.split_once(", ").unwrap())
        .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
        .collect();

    part_a(&coords);
    part_b(&coords);
}

fn part_a(coords: &Vec<(usize, usize)>) {
    let min_x = *coords.iter().map(|(x, _)| x).min().unwrap();
    let max_x = *coords.iter().map(|(x, _)| x).max().unwrap();
    let min_y = *coords.iter().map(|(_, y)| y).min().unwrap();
    let max_y = *coords.iter().map(|(_, y)| y).max().unwrap();

    let mut areas = HashMap::new();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let mut min = i32::MAX;
            let mut min_cnt = 0;
            let mut min_area_name = usize::MAX;

            coords.iter().enumerate()
                .map(|(c, (cx, cy))| (c, (*cx as i32 - x as i32).abs() + (*cy as i32 - y as i32).abs()))
                .for_each(|(c, dist)|
                    if dist < min {
                        min = dist;
                        min_cnt = 1;
                        min_area_name = c;
                    } else if dist == min {
                        min_cnt += 1;
                    }
                );

            if min_cnt == 1 {
                *areas.entry(min_area_name).or_insert(0) += 1;
            }
        }
    }

    // Remove the areas that are on infinite
    coords.iter().enumerate()
        .for_each(|(c, (cx, cy))|
            if *cx == min_x || *cx == max_x || *cy == min_y || *cy == max_x { areas.remove(&c); });

    let largest_area = areas.values().max().unwrap();
    println!("The largest area's size is {}", largest_area);
}

fn part_b(coords: &Vec<(usize, usize)>) {
    let min_x = *coords.iter().map(|(x, _)| x).min().unwrap() as i32;
    let max_x = *coords.iter().map(|(x, _)| x).max().unwrap() as i32;
    let min_y = *coords.iter().map(|(_, y)| y).min().unwrap() as i32;
    let max_y = *coords.iter().map(|(_, y)| y).max().unwrap() as i32;

    // Total distance from all points should be 10.000
    // Max distance from one point should be 10.000 / num of points
    let inc = 10000 / coords.len() as i32;

    let mut ctr = 0;
    for x in (min_x - inc)..=(max_x + inc) {
        for y in (min_y - inc)..=(max_y + inc) {
            let sum: i32 = coords.iter().map(|(cx, cy)| (*cx as i32 - x).abs() + (*cy as i32 - y).abs()).sum();
            if sum < 10000 {
                ctr += 1;
            }
        }
    }

    println!("There are {} points with distance from all locations < 10000", ctr);
}
