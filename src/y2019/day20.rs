use std::collections::HashMap;
use std::fs;

pub(crate) fn day20() {
    let input = fs::read_to_string("input/2019/day20/input.txt").unwrap();
    let mut raw_map = vec!();
    for line in input.split("\n") {
        let row: Vec<char> = line.chars().collect();
        raw_map.push(row);
    }

    // Read the map and extract the portals and the start/finish points
    let (start, finish, portals) = find_portals(raw_map.clone());

    part_a(&mut raw_map, start, &finish, &portals);

    let max_x = raw_map[2].len() - 1;
    let max_y = raw_map.len() - 4;
    part_b(&mut raw_map, start, &finish, &portals, max_x, max_y);
}

fn part_a(raw_map: &mut Vec<Vec<char>>, start: (usize, usize), finish: &(usize, usize), portals: &HashMap<(usize, usize), (usize, usize)>) {
    // Run slightly modified DFS w/ distances from start for each point
    let mut dist: HashMap<(usize, usize), usize> = HashMap::new();
    let mut to_visit: Vec<((usize, usize), usize)> = vec!();
    to_visit.push((start, 0));
    while !to_visit.is_empty() {
        let ((x, y), d) = to_visit.pop().unwrap();
        dist.insert((x, y), d);

        if x > 0 && raw_map[y][x - 1] == '.' && *dist.entry((x - 1, y)).or_insert(usize::MAX) > (d + 1) {
            visit_with_portals(x - 1, y, d + 1, &mut to_visit, &portals);
        }
        if x < raw_map[y].len() - 1 && raw_map[y][x + 1] == '.' && *dist.entry((x + 1, y)).or_insert(usize::MAX) > (d + 1) {
            visit_with_portals(x + 1, y, d + 1, &mut to_visit, &portals);
        }
        if y > 0 && raw_map[y - 1][x] == '.' && *dist.entry((x, y - 1)).or_insert(usize::MAX) > (d + 1) {
            visit_with_portals(x, y - 1, d + 1, &mut to_visit, &portals);
        }
        if y < (raw_map.len() - 1) && raw_map[y + 1][x] == '.' && *dist.entry((x, y + 1)).or_insert(usize::MAX) > (d + 1) {
            visit_with_portals(x, y + 1, d + 1, &mut to_visit, &portals);
        }
    }
    println!("It takes {} steps to get from AA to ZZ", dist[&finish]);
}

fn part_b(raw_map: &mut Vec<Vec<char>>, start: (usize, usize), finish: &(usize, usize),
          portals: &HashMap<(usize, usize), (usize, usize)>,
          max_x: usize, max_y: usize) {
    let mut dist: HashMap<(usize, usize, usize), usize> = HashMap::new();
    let mut to_visit: Vec<((usize, usize, usize), usize)> = vec!();
    to_visit.push(((start.0, start.1, 0), 0));
    while !to_visit.is_empty() {
        let ((x, y, dim), d) = to_visit.pop().unwrap();
        dist.insert((x, y, dim), d);

        if x > 0 && raw_map[y][x - 1] == '.' {
            visit_with_portals_multidim(x - 1, y, dim, d + 1, &mut to_visit, &mut dist, &portals, max_x, max_y);
        }
        if x < raw_map[y].len() - 1 && raw_map[y][x + 1] == '.' {
            visit_with_portals_multidim(x + 1, y, dim, d + 1, &mut to_visit, &mut dist, &portals, max_x, max_y);
        }
        if y > 0 && raw_map[y - 1][x] == '.' {
            visit_with_portals_multidim(x, y - 1, dim, d + 1, &mut to_visit, &mut dist, &portals, max_x, max_y);
        }
        if y < (raw_map.len() - 1) && raw_map[y + 1][x] == '.' {
            visit_with_portals_multidim(x, y + 1, dim, d + 1, &mut to_visit, &mut dist, &portals, max_x, max_y);
        }
    }


    if dist.contains_key(&(finish.0, finish.1, 0)) {
        println!("It takes {} steps to get from AA to ZZ with recurstion", dist[&(finish.0, finish.1, 0)]);
    } else {
        println!("Cannot get to the ZZ with recursion");
    }
}

fn visit_with_portals(x: usize, y: usize, d: usize,
                      to_visit: &mut Vec<((usize, usize), usize)>,
                      portals: &HashMap<(usize, usize), (usize, usize)>) {
    if portals.contains_key(&(x, y)) {
        let (new_x, new_y) = portals.get(&(x, y)).unwrap();
        to_visit.push(((*new_x, *new_y), d + 1));
    } else {
        to_visit.push(((x, y), d));
    }
}

fn visit_with_portals_multidim(x: usize, y: usize, dim: usize, d: usize,
                               to_visit: &mut Vec<((usize, usize, usize), usize)>,
                               dist: &mut HashMap<(usize, usize, usize), usize>,
                               portals: &HashMap<(usize, usize), (usize, usize)>,
                               max_x: usize, max_y: usize
) {
    let outer = x == 2 || x == max_x || y == 2 || y == max_y;

    if portals.contains_key(&(x, y)) {
        if dim == 0 && outer {
            // Outer portals don't work on dim 0, unless they are start or finish
            return;
        }

        let (new_x, new_y) = portals.get(&(x, y)).unwrap();
        let new_dim = if outer { dim - 1 } else { dim + 1 };

        if new_dim < portals.len() / 2 && *dist.entry((*new_x, *new_y, new_dim)).or_insert(usize::MAX) > d {
            to_visit.push(((*new_x, *new_y, new_dim), d + 1));
        }
    } else if *dist.entry((x, y, dim)).or_insert(usize::MAX) > d {
        to_visit.push(((x, y, dim), d));
    }
}

fn find_portals(raw_map: Vec<Vec<char>>) -> ((usize, usize), (usize, usize), HashMap<(usize, usize), (usize, usize)>) {
    // Search horizontally
    let mut portal_candidates = HashMap::new();
    let mut y = 0;
    for row in &raw_map {
        let mut x = 0;
        let mut prev_prev_c = ' ';
        let mut prev_c = ' ';
        for c in row {
            if *c != ' ' && *c != '.' && *c != '#' {
                if prev_c != ' ' && prev_c != '.' && prev_c != '#' {
                    // if prev prev c is an open passage, the next c won't be one.. so we found the spot for the portal
                    let pos = if prev_prev_c == '.' { (x - 2, y) } else { (x + 1, y) };

                    // We found pair of letters
                    portal_candidates.entry((prev_c, *c)).or_insert(vec!()).push(pos);
                }
            }
            prev_prev_c = prev_c;
            prev_c = *c;
            x += 1;
        }
        y += 1;
    }

    // Search vertically
    for x in 0..raw_map[3].len() {
        let mut prev_prev_c = ' ';
        let mut prev_c = ' ';
        for y in 0..raw_map.len() {
            if x >= raw_map[y].len() { continue; }

            let c = raw_map[y][x];
            if c != ' ' && c != '.' && c != '#' {
                if prev_c != ' ' && prev_c != '.' && prev_c != '#' {
                    // if prev prev c is an open passage, the next c won't be one.. so we found the spot for the portal
                    let pos = if prev_prev_c == '.' { (x, y - 2) } else { (x, y + 1) };

                    // We found pair of letters
                    portal_candidates.entry((prev_c, c)).or_insert(vec!()).push(pos);
                }
            }
            prev_prev_c = prev_c;
            prev_c = c;
        }
    }

    let mut portals = HashMap::new();
    portal_candidates.values()
        .filter(|p| p.len() == 2)
        .for_each(|p| {
            portals.insert(p[0], p[1]);
            portals.insert(p[1], p[0]);
        });

    let start = *portal_candidates.get(&('A', 'A')).unwrap().first().unwrap();
    let finish = *portal_candidates.get(&('Z', 'Z')).unwrap().first().unwrap();

    return (start, finish, portals);
}
