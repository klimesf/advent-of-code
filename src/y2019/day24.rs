use std::collections::{HashMap, HashSet};
use std::fs;

type Grid = Vec<Vec<char>>;

pub(crate) fn day24() {
    let input = fs::read_to_string("input/2019/day24/input.txt").unwrap();
    let map: Grid = input.trim().split("\n").map(|r| r.chars().collect()).collect();

    part_a(&map);
    part_b(&map);
}

fn part_a(map: &Grid) {
    let mut biodiversities: HashSet<u128> = HashSet::new();
    let mut state = map.clone();
    loop {
        let biodiversity = calc_biodiversity(&state);
        if !biodiversities.insert(biodiversity) {
            println!("First biodiversity to appear twice is: {}", biodiversity);
            break;
        }

        state = calc_new_state(&state);
    }
}

fn part_b(map: &Grid) {
    let mut state = HashMap::new();
    state.insert(0, map.clone());
    let iterations = 200;
    for _c in 0..iterations {
        state = calc_new_state_multidim(&state);
    }
    println!("After {} minutes, there are {} bugs present", iterations, count_bugs(&state));
}

fn calc_biodiversity(bugs: &Grid) -> u128 {
    let mut total = 0;
    let mut i = 1;
    for row in bugs {
        for col in row {
            if *col == '#' {
                total += i;
            }
            i *= 2;
        }
    }
    return total;
}

fn calc_new_state(bugs: &Grid) -> Grid {
    let mut new_bugs = vec![vec!['.'; 5]; 5];

    for y in 0..bugs.len() {
        for x in 0..bugs[y].len() {
            let mut adjacent_bugs = 0;
            if y > 0 && bugs[y - 1][x] == '#' {
                adjacent_bugs += 1
            }
            if y < (bugs.len() - 1) && bugs[y + 1][x] == '#' {
                adjacent_bugs += 1
            }
            if x > 0 && bugs[y][x - 1] == '#' {
                adjacent_bugs += 1
            }
            if x < (bugs[y].len() - 1) && bugs[y][x + 1] == '#' {
                adjacent_bugs += 1
            }

            if bugs[y][x] == '.' {
                // Becomes infested?
                new_bugs[y][x] = if adjacent_bugs == 1 || adjacent_bugs == 2 {
                    '#'
                } else {
                    '.'
                };
            } else {
                // Bug dies?
                new_bugs[y][x] = if adjacent_bugs == 1 { '#' } else { '.' };
            }
        }
    }
    return new_bugs;
}

fn count_bugs(bugs: &HashMap<i32, Grid>) -> u128 {
    bugs.values()
        .map(|g| {
            let mut sum = 0;
            for y in 0..g.len() {
                for x in 0..g[y].len() {
                    if g[y][x] == '#' {
                        sum += 1;
                    }
                }
            }
            sum
        })
        .sum()
}

fn calc_new_state_multidim(dim_bugs: &HashMap<i32, Grid>) -> HashMap<i32, Grid> {
    let mut new_dim_bugs = HashMap::new();

    let from = *dim_bugs.keys().min().unwrap() - 1;
    let to = *dim_bugs.keys().max().unwrap() + 1;

    for dim in from..=to {
        let bugs = if dim_bugs.contains_key(&dim) {
            dim_bugs.get(&dim).unwrap().clone()
        } else {
            vec![vec!['.'; 5]; 5]
        };

        let mut new_bugs = vec![vec![' '; 5]; 5];
        for y in 0..bugs.len() {
            for x in 0..bugs[y].len() {
                let mut adjacent_bugs = 0;

                if x == 0 {
                    adjacent_bugs += is_bug(dim_bugs, dim + 1, 2, 1)
                }
                if x == 0 && bugs[y][x + 1] == '#' {
                    adjacent_bugs += 1
                }

                if x == 1 && bugs[y][x - 1] == '#' {
                    adjacent_bugs += 1
                }
                if x == 1 && y != 2 && bugs[y][x + 1] == '#' {
                    adjacent_bugs += 1
                }
                if x == 1 && y == 2 {
                    adjacent_bugs += sum_x(dim_bugs, dim - 1, 0)
                }

                if x == 2 && y != 2 && bugs[y][x - 1] == '#' {
                    adjacent_bugs += 1
                }
                if x == 2 && y != 2 && bugs[y][x + 1] == '#' {
                    adjacent_bugs += 1
                }

                if x == 3 && bugs[y][x + 1] == '#' {
                    adjacent_bugs += 1
                }
                if x == 3 && y != 2 && bugs[y][x - 1] == '#' {
                    adjacent_bugs += 1
                }
                if x == 3 && y == 2 {
                    adjacent_bugs += sum_x(dim_bugs, dim - 1, 4)
                }

                if x == 4 && bugs[y][x - 1] == '#' {
                    adjacent_bugs += 1
                }
                if x == 4 {
                    adjacent_bugs += is_bug(dim_bugs, dim + 1, 2, 3)
                }

                if y == 0 {
                    adjacent_bugs += is_bug(dim_bugs, dim + 1, 1, 2)
                }
                if y == 0 && bugs[y + 1][x] == '#' {
                    adjacent_bugs += 1
                }

                if y == 1 && bugs[y - 1][x] == '#' {
                    adjacent_bugs += 1
                }
                if y == 1 && x != 2 && bugs[y + 1][x] == '#' {
                    adjacent_bugs += 1
                }
                if y == 1 && x == 2 {
                    adjacent_bugs += sum_y(dim_bugs, dim - 1, 0)
                }

                if y == 2 && x != 2 && bugs[y - 1][x] == '#' {
                    adjacent_bugs += 1
                }
                if y == 2 && x != 2 && bugs[y + 1][x] == '#' {
                    adjacent_bugs += 1
                }

                if y == 3 && bugs[y + 1][x] == '#' {
                    adjacent_bugs += 1
                }
                if y == 3 && x != 2 && bugs[y - 1][x] == '#' {
                    adjacent_bugs += 1
                }
                if y == 3 && x == 2 {
                    adjacent_bugs += sum_y(dim_bugs, dim - 1, 4)
                }

                if y == 4 && bugs[y - 1][x] == '#' {
                    adjacent_bugs += 1
                }
                if y == 4 {
                    adjacent_bugs += is_bug(dim_bugs, dim + 1, 3, 2)
                }

                if bugs[y][x] == '.' {
                    // Becomes infested?
                    new_bugs[y][x] = if adjacent_bugs == 1 || adjacent_bugs == 2 {
                        '#'
                    } else {
                        '.'
                    };
                } else {
                    // Bug dies?
                    new_bugs[y][x] = if adjacent_bugs == 1 { '#' } else { '.' };
                }
            }
        }
        new_dim_bugs.insert(dim, new_bugs);
    }

    return new_dim_bugs;
}

fn is_bug(dim_bugs: &HashMap<i32, Grid>, dim: i32, y: usize, x: usize) -> i32 {
    if !dim_bugs.contains_key(&dim) {
        return 0;
    }
    let g = dim_bugs.get(&dim).unwrap();
    return if g[y][x] == '#' { 1 } else { 0 };
}

fn sum_y(dim_bugs: &HashMap<i32, Grid>, dim: i32, y: usize) -> i32 {
    if !dim_bugs.contains_key(&dim) {
        return 0;
    }
    let g = dim_bugs.get(&dim).unwrap();
    let mut sum = 0;
    for x in 0..5 {
        if g[y][x] == '#' {
            sum += 1;
        }
    }
    return sum;
}

fn sum_x(dim_bugs: &HashMap<i32, Grid>, dim: i32, x: usize) -> i32 {
    if !dim_bugs.contains_key(&dim) {
        return 0;
    }
    let g = dim_bugs.get(&dim).unwrap();
    let mut sum = 0;
    for y in 0..5 {
        if g[y][x] == '#' {
            sum += 1;
        }
    }
    return sum;
}

fn _print_state(dim_bugs: &HashMap<i32, Grid>) {
    let from = *dim_bugs.keys().min().unwrap();
    let to = *dim_bugs.keys().max().unwrap();

    for dim in from..=to {
        println!("Dim {}:", dim);
        for y in 0..5 {
            for x in 0..5 {
                print!("{}", if x == 2 && y == 2 { '?' } else { dim_bugs[&dim][y][x] });
            }
            println!();
        }
        println!();
    }
}
