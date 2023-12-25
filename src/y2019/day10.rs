use std::collections::{HashMap, HashSet};
use std::f32::consts::PI;
use std::fs;
use crate::utils::toolbox::gcd;

type Coord = (i32, i32);

pub(crate) fn day10() {
    let input = fs::read_to_string("input/2019/day10/input.txt").unwrap();

    // Load asteroids as locations, a.k.a. graph vertices
    let mut asteroids: HashSet<Coord> = HashSet::new();
    let dim = (input.trim().len() as f64).sqrt() as i32;
    for (y, row) in input.trim().split('\n').enumerate() {
        for (x, val) in row.chars().enumerate() {
            match val {
                '#' => { asteroids.insert((x as i32, y as i32)); }
                _ => {}
            }
        }
    }

    // --- Part A

    // Build lines of sight between asteroids, a.k.a. graph edges
    let mut los: HashMap<Coord, Vec<Coord>> = HashMap::new();
    for asteroid in &asteroids {
        for other in &asteroids {
            if asteroid == other { continue; }

            // Check if we have a line of sight, if yes, add it to the map:
            // First, find the direction vector between the asteroids.
            // Then add the vector to the asteroid incrementally, check if there is another asteroid in the way,
            // and if not, add the edge.
            let dir = direction(other.0 - asteroid.0, other.1 - asteroid.1);
            let mut visible = false;
            let mut x = asteroid.0;
            let mut y = asteroid.1;
            while x != other.0 || y != other.1 {
                x = x + dir.0;
                y = y + dir.1;
                if asteroids.contains(&(x, y)) {
                    visible = x == other.0 && y == other.1;
                    break;
                }
            }

            if visible {
                let edges = los.entry(*asteroid).or_insert(vec!());
                edges.push(*other);
            }
        }
    }

    let (station, station_count) = &los.iter()
        .max_by(|(_, e1), (_, e2)| e1.len().cmp(&e2.len()))
        .unwrap();
    println!("Station at {:?} sees {} asteroids", station, station_count.len());
    // --- Part B

    // Calculate angles between starting positions and the vector from station to each asteroid
    let mut targets = asteroids.clone();
    targets.remove(station);
    let mut angles: Vec<(Coord, f32)> = vec!();
    for target in &targets {
        let dir = direction(target.0 - station.0, target.1 - station.1);
        let angle = clockwise_angle((0, -dim), dir);
        if angles.contains(&(dir, angle)) {
            continue;
        }
        angles.push((dir, angle));
    }
    angles.sort_by(|t1, t2| return t1.1.partial_cmp(&t2.1).unwrap());

    let mut hits: Vec<Coord> = vec!();
    let mut i = 1;
    while !targets.is_empty() {
        for (dir, _) in &angles {
            let mut x = station.0;
            let mut y = station.1;
            while x >= 0 && x <= 2 * dim && y >= 0 && y <= 2 * dim {
                x = x + dir.0;
                y = y + dir.1;
                if targets.contains(&(x, y)) {
                    targets.remove(&(x, y));
                    hits.push((x, y));
                    break;
                }
            }
        }
        println!("After rotation {}, {} targets remain", i, targets.len());
        i += 1;
    }

    let the_200th_hit = if hits.len() >= 200 { hits[199] } else { (0, 0) };
    println!("200th hit: {}", the_200th_hit.0 * 100 + the_200th_hit.1)
}

fn clockwise_angle(b: Coord, a: Coord) -> f32 {
    let angle = ((a.1 * b.0 - a.0 * b.1) as f32).atan2((a.0 * b.0 + a.1 * b.1) as f32);
    // It gives only degrees up to PI, both positive and negative.
    // But we want it to calculate the whole angle clockwise
    // So we need to transform to the whole circle by adding 2*PI in case it's negative..
    return if angle < (0 as f32) { angle + PI + PI } else { angle };
}

fn direction(mut x: i32, mut y: i32) -> (i32, i32) {
    // Shrink a vector to the smallest possible length while keeping it's direction within 2D array
    loop {
        let gcd = if x > y { gcd(x, y) } else { gcd(y, x) };
        if gcd == 1 {
            return (x, y);
        }
        x = x / gcd;
        y = y / gcd;
    }
}
