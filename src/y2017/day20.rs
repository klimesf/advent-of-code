use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs;

use regex::Regex;

use crate::utils::toolbox::parse_i32;

pub(crate) fn day20() {
    let input = fs::read_to_string("input/2017/day20/input.txt").unwrap();
    let particles = parse(input.as_str());
    println!("{}", find_closest(&particles));
    println!("{}", resolve_collisions(&particles));
}

type Particle = ((i32, i32, i32), (i32, i32, i32), (i32, i32, i32));

fn find_closest(particles: &Vec<Particle>) -> usize {
    particles.iter().enumerate()
        .map(|(i, (_, _, a))| (i, a.0.abs() + a.1.abs() + a.2.abs()))
        .min_by(|a, b| a.1.cmp(&b.1))
        .unwrap().0
}

fn resolve_collisions(particles: &Vec<Particle>) -> usize {
    let mut surviving_particles: HashMap<usize, Particle> = particles.iter().enumerate()
        .map(|(i, p)| (i, *p)).collect();

    for _ in 0..50 {
        let mut next_generation = HashMap::new();
        let mut collision_map: HashMap<(i32, i32, i32), Vec<usize>> = HashMap::new();

        for (id, particle) in &surviving_particles {
            let (mut p, mut v, a) = particle;
            v = (v.0 + a.0, v.1 + a.1, v.2 + a.2);
            p = (p.0 + v.0, p.1 + v.1, p.2 + v.2);
            next_generation.insert(*id, (p, v, *a));
            collision_map.entry(p).or_insert(vec!()).push(*id);
        }

        for (_, ids) in &collision_map {
            if ids.len() < 2 { continue; }
            for id in ids {
                match next_generation.entry(*id) {
                    Entry::Occupied(o) => { o.remove_entry(); }
                    _ => { panic!("Removing already removed entry {}", id) }
                }
            }
        }
        surviving_particles = next_generation;
    }

    surviving_particles.len()
}

fn parse(input: &str) -> Vec<Particle> {
    let re = Regex::new(r"^p=<([\-0-9]+),([\-0-9]+),([\-0-9]+)>, v=<([\-0-9]+),([\-0-9]+),([\-0-9]+)>, a=<([\-0-9]+),([\-0-9]+),([\-0-9]+)>$").unwrap();
    input.lines().map(|line| {
        let g = re.captures(line).unwrap();
        let px = parse_i32(g.get(1));
        let py = parse_i32(g.get(2));
        let pz = parse_i32(g.get(3));
        let vx = parse_i32(g.get(4));
        let vy = parse_i32(g.get(5));
        let vz = parse_i32(g.get(6));
        let ax = parse_i32(g.get(7));
        let ay = parse_i32(g.get(8));
        let az = parse_i32(g.get(9));
        ((px, py, pz), (vx, vy, vz), (ax, ay, az))
    }).collect()
}

#[cfg(test)]
mod day20_tests {
    use std::fs;

    use crate::y2017::day20::{parse, find_closest, resolve_collisions};

    #[test]
    fn test_works() {
        let input = fs::read_to_string("input/2017/day20/test.txt").unwrap();
        let particles = parse(input.as_str());
        assert_eq!(0, find_closest(&particles));
    }

    #[test]
    fn test_2_works() {
        let input = fs::read_to_string("input/2017/day20/test_2.txt").unwrap();
        let particles = parse(input.as_str());
        assert_eq!(1, resolve_collisions(&particles));
    }

    #[test]
    fn input_works() {
        let input = fs::read_to_string("input/2017/day20/input.txt").unwrap();
        let particles = parse(input.as_str());
        assert_eq!(144, find_closest(&particles));
        assert_eq!(477, resolve_collisions(&particles));
    }
}
