use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;

pub(crate) fn day15() {
    println!("{}", part_a(fs::read_to_string("input/2018/day15/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2018/day15/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    let (mut map, mut units) = parse_input(input);

    let mut round = 0;
    'outer: loop {
        units.sort_by(|a, b| {
            let sa = a.r * map[0].len() + a.c;
            let sb = b.r * map[0].len() + b.c;
            sa.cmp(&sb)
        });

        'unit_loop: for i in 0..units.len() {
            // println!("-- -- unit {}", i);
            let mut unit = units[i];
            if unit.hp == 0 {
                continue;
            }

            let targets: Vec<usize> = units
                .iter()
                .enumerate()
                .filter(|(_, target)| target.t != unit.t && target.hp > 0)
                .map(|(j, _)| j)
                .collect();

            if targets.len() == 0 {
                break 'outer;
            }

            let mut adjacent_targets = count_adjacent_enemies(&map, &unit);
            if adjacent_targets == 0 {
                // Move if not in range of a target
                let mut in_range: Vec<(usize, usize)> = vec![];
                targets
                    .iter()
                    .map(|j| units[*j])
                    .map(|target| adjacent_reachable_fields(&map, target.r, target.c))
                    .for_each(|fields| fields.iter().for_each(|(r, c)| in_range.push((*r, *c))));

                // Find reachable
                let mut reachable = vec![];
                let mut stack = BinaryHeap::new();
                let mut visited = HashSet::new();
                stack.push(Pos {
                    r: unit.r,
                    c: unit.c,
                    dist: 0,
                });

                while let Some(pos) = stack.pop() {
                    if !visited.insert((pos.r, pos.c)) {
                        continue;
                    }
                    if in_range.contains(&(pos.r, pos.c)) {
                        reachable.push((pos.r, pos.c, pos.dist))
                    }

                    for (r2, c2) in adjacent_reachable_fields(&map, pos.r, pos.c) {
                        stack.push(Pos {
                            r: r2,
                            c: c2,
                            dist: pos.dist + 1,
                        });
                    }
                }

                // Select nearest (ties resolved by reading order)
                let destination = reachable
                    .iter()
                    .sorted_by(|a, b| {
                        if a.2 == b.2 {
                            (a.0 * map[0].len() + a.1).cmp(&(b.0 * map[0].len() + b.1))
                        } else {
                            a.2.cmp(&b.2)
                        }
                    })
                    .find_or_first(|_| true);

                match destination {
                    None => {}
                    Some(dest) => {
                        // Select the step that is nearest (tie resolved by reading order)
                        let mut dest_dist: HashMap<(usize, usize), usize> = HashMap::new();
                        let mut stack = BinaryHeap::new();
                        stack.push(Pos {
                            r: dest.0,
                            c: dest.1,
                            dist: 0,
                        });

                        while let Some(pos) = stack.pop() {
                            if dest_dist.contains_key(&(pos.r, pos.c)) {
                                continue;
                            }
                            dest_dist.insert((pos.r, pos.c), pos.dist);

                            for (r2, c2) in adjacent_reachable_fields(&map, pos.r, pos.c) {
                                stack.push(Pos {
                                    r: r2,
                                    c: c2,
                                    dist: pos.dist + 1,
                                });
                            }
                        }

                        let move_to = *adjacent_reachable_fields(&map, unit.r, unit.c)
                            .iter()
                            .sorted_by(|a, b| {
                                if !dest_dist.contains_key(&(a.0, a.1)) {
                                    return Ordering::Greater;
                                }
                                if !dest_dist.contains_key(&(b.0, b.1)) {
                                    return Ordering::Less;
                                }

                                let da = dest_dist.get(&(a.0, a.1)).unwrap();
                                let db = dest_dist.get(&(b.0, b.1)).unwrap();

                                if da == db {
                                    (a.0 * map[0].len() + a.1).cmp(&(b.0 * map[0].len() + b.1))
                                } else {
                                    da.cmp(&db)
                                }
                            })
                            .find_or_first(|_| true)
                            .unwrap();

                        // Move one step, don't forget to change map
                        map[unit.r][unit.c] = '.';
                        map[move_to.0][move_to.1] = match unit.t {
                            UnitType::ELF => 'E',
                            UnitType::GOBLIN => 'G',
                        };
                        unit.r = move_to.0;
                        unit.c = move_to.1;
                        units[i] = unit;
                    }
                }

                adjacent_targets = count_adjacent_enemies(&map, &unit);
            }

            // Attack adjacent target with lowest HP, ties resolved by reading order
            if adjacent_targets == 0 {
                continue 'unit_loop;
            } // If still not adjacent, continue

            let mut selected_target = 0;
            let mut min_hp = usize::MAX;
            let mut min_reading_order = usize::MAX;
            for j in &targets {
                let target = units[*j];
                if manhattan_dist(&unit, &target) != 1 {
                    continue;
                }
                let target_reading_order = (target.r * map[0].len()) + target.c;
                if target.hp < min_hp || target.hp == min_hp && target_reading_order < min_reading_order {
                    selected_target = *j;
                    min_hp = target.hp;
                    min_reading_order = target_reading_order
                }
            }

            let mut target = units[selected_target];
            if target.hp <= 3 {
                target.hp = 0;
                map[target.r][target.c] = '.';
            } else {
                target.hp -= 3;
            }
            units[selected_target] = target;
        }
        round += 1;
    }

    let elves_hps = units
        .iter()
        .filter(|u| u.t == UnitType::ELF)
        .map(|u| u.hp)
        .sum::<usize>();
    let goblin_hps = units
        .iter()
        .filter(|u| u.t == UnitType::GOBLIN)
        .map(|u| u.hp)
        .sum::<usize>();

    return round * (goblin_hps + elves_hps);
}

fn part_b(input: String) -> usize {
    let (og_map, og_units) = parse_input(input);

    let mut bonus = 1;
    'bonus: loop {
        let mut map = og_map.clone();
        let mut units = og_units.clone();

        let mut round = 0;
        'outer: loop {
            units.sort_by(|a, b| {
                let sa = a.r * map[0].len() + a.c;
                let sb = b.r * map[0].len() + b.c;
                sa.cmp(&sb)
            });

            'unit_loop: for i in 0..units.len() {
                // println!("-- -- unit {}", i);
                let mut unit = units[i];
                if unit.hp == 0 {
                    continue;
                }

                let targets: Vec<usize> = units
                    .iter()
                    .enumerate()
                    .filter(|(_, target)| target.t != unit.t && target.hp > 0)
                    .map(|(j, _)| j)
                    .collect();

                if targets.len() == 0 {
                    break 'outer;
                }

                let mut adjacent_targets = count_adjacent_enemies(&map, &unit);
                if adjacent_targets == 0 {
                    // Move if not in range of a target
                    let mut in_range: Vec<(usize, usize)> = vec![];
                    targets
                        .iter()
                        .map(|j| units[*j])
                        .map(|target| adjacent_reachable_fields(&map, target.r, target.c))
                        .for_each(|fields| fields.iter().for_each(|(r, c)| in_range.push((*r, *c))));

                    // Find reachable
                    let mut reachable = vec![];
                    let mut stack = BinaryHeap::new();
                    let mut visited = HashSet::new();
                    stack.push(Pos {
                        r: unit.r,
                        c: unit.c,
                        dist: 0,
                    });

                    while let Some(pos) = stack.pop() {
                        if !visited.insert((pos.r, pos.c)) {
                            continue;
                        }
                        if in_range.contains(&(pos.r, pos.c)) {
                            reachable.push((pos.r, pos.c, pos.dist))
                        }

                        for (r2, c2) in adjacent_reachable_fields(&map, pos.r, pos.c) {
                            stack.push(Pos {
                                r: r2,
                                c: c2,
                                dist: pos.dist + 1,
                            });
                        }
                    }

                    // Select nearest (ties resolved by reading order)
                    let destination = reachable
                        .iter()
                        .sorted_by(|a, b| {
                            if a.2 == b.2 {
                                (a.0 * map[0].len() + a.1).cmp(&(b.0 * map[0].len() + b.1))
                            } else {
                                a.2.cmp(&b.2)
                            }
                        })
                        .find_or_first(|_| true);

                    match destination {
                        None => {}
                        Some(dest) => {
                            // Select the step that is nearest (tie resolved by reading order)
                            let mut dest_dist: HashMap<(usize, usize), usize> = HashMap::new();
                            let mut stack = BinaryHeap::new();
                            stack.push(Pos {
                                r: dest.0,
                                c: dest.1,
                                dist: 0,
                            });

                            while let Some(pos) = stack.pop() {
                                if dest_dist.contains_key(&(pos.r, pos.c)) {
                                    continue;
                                }
                                dest_dist.insert((pos.r, pos.c), pos.dist);

                                for (r2, c2) in adjacent_reachable_fields(&map, pos.r, pos.c) {
                                    stack.push(Pos {
                                        r: r2,
                                        c: c2,
                                        dist: pos.dist + 1,
                                    });
                                }
                            }

                            let move_to = *adjacent_reachable_fields(&map, unit.r, unit.c)
                                .iter()
                                .sorted_by(|a, b| {
                                    if !dest_dist.contains_key(&(a.0, a.1)) {
                                        return Ordering::Greater;
                                    }
                                    if !dest_dist.contains_key(&(b.0, b.1)) {
                                        return Ordering::Less;
                                    }

                                    let da = dest_dist.get(&(a.0, a.1)).unwrap();
                                    let db = dest_dist.get(&(b.0, b.1)).unwrap();

                                    if da == db {
                                        (a.0 * map[0].len() + a.1).cmp(&(b.0 * map[0].len() + b.1))
                                    } else {
                                        da.cmp(&db)
                                    }
                                })
                                .find_or_first(|_| true)
                                .unwrap();

                            // Move one step, don't forget to change map
                            map[unit.r][unit.c] = '.';
                            map[move_to.0][move_to.1] = match unit.t {
                                UnitType::ELF => 'E',
                                UnitType::GOBLIN => 'G',
                            };
                            unit.r = move_to.0;
                            unit.c = move_to.1;
                            units[i] = unit;
                        }
                    }

                    adjacent_targets = count_adjacent_enemies(&map, &unit);
                }

                // Attack adjacent target with lowest HP, ties resolved by reading order
                if adjacent_targets == 0 {
                    continue 'unit_loop;
                } // If still not adjacent, continue

                let mut selected_target = 0;
                let mut min_hp = usize::MAX;
                let mut min_reading_order = usize::MAX;
                for j in &targets {
                    let target = units[*j];
                    if manhattan_dist(&unit, &target) != 1 {
                        continue;
                    }
                    let target_reading_order = (target.r * map[0].len()) + target.c;
                    if target.hp < min_hp || target.hp == min_hp && target_reading_order < min_reading_order {
                        selected_target = *j;
                        min_hp = target.hp;
                        min_reading_order = target_reading_order
                    }
                }

                let mut target = units[selected_target];
                let attack = match unit.t {
                    UnitType::ELF => 3 + bonus,
                    UnitType::GOBLIN => 3,
                };

                if target.hp <= attack {
                    target.hp = 0;
                    map[target.r][target.c] = '.';

                    if target.t == UnitType::ELF {
                        bonus += 1;
                        continue 'bonus;
                    }
                } else {
                    target.hp -= attack;
                }
                units[selected_target] = target;
            }
            round += 1;
        }

        let elves_hps = units
            .iter()
            .filter(|u| u.t == UnitType::ELF)
            .map(|u| u.hp)
            .sum::<usize>();
        let goblin_hps = units
            .iter()
            .filter(|u| u.t == UnitType::GOBLIN)
            .map(|u| u.hp)
            .sum::<usize>();

        return round * (goblin_hps + elves_hps);
    }
}

fn parse_input(input: String) -> (Vec<Vec<char>>, Vec<Unit>) {
    let mut units = vec![];
    let map: Vec<Vec<char>> = input
        .lines()
        .enumerate()
        .map(|(r, line)| {
            line.chars()
                .enumerate()
                .map(|(c, pos)| {
                    if pos == 'G' {
                        units.push(Unit {
                            r,
                            c,
                            hp: 200,
                            t: UnitType::GOBLIN,
                        });
                        pos
                    } else if pos == 'E' {
                        units.push(Unit {
                            r,
                            c,
                            hp: 200,
                            t: UnitType::ELF,
                        });
                        pos
                    } else {
                        pos
                    }
                })
                .collect()
        })
        .collect();
    (map, units)
}

fn manhattan_dist(a: &Unit, b: &Unit) -> usize {
    a.r.max(b.r) - a.r.min(b.r) + a.c.max(b.c) - a.c.min(b.c)
}

/// Returns adjacent fields in reading order.
/// Considers only . fields.
fn adjacent_reachable_fields(map: &Vec<Vec<char>>, r: usize, c: usize) -> Vec<(usize, usize)> {
    let mut adjacent = vec![];
    if map[r - 1][c] == '.' {
        adjacent.push((r - 1, c))
    }
    if map[r][c - 1] == '.' {
        adjacent.push((r, c - 1));
    }
    if map[r][c + 1] == '.' {
        adjacent.push((r, c + 1));
    }
    if map[r + 1][c] == '.' {
        adjacent.push((r + 1, c))
    }
    adjacent
}

fn count_adjacent_enemies(map: &Vec<Vec<char>>, unit: &Unit) -> usize {
    let mut adjacent = 0;
    let enemy = match unit.t {
        UnitType::ELF => 'G',
        UnitType::GOBLIN => 'E',
    };
    if map[unit.r - 1][unit.c] == enemy {
        adjacent += 1
    }
    if map[unit.r][unit.c - 1] == enemy {
        adjacent += 1
    }
    if map[unit.r][unit.c + 1] == enemy {
        adjacent += 1
    }
    if map[unit.r + 1][unit.c] == enemy {
        adjacent += 1
    }
    adjacent
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Pos {
    r: usize,
    c: usize,
    dist: usize,
}

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Copy, Clone, Debug)]
struct Unit {
    r: usize,
    c: usize,
    hp: usize,
    t: UnitType,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum UnitType {
    ELF,
    GOBLIN,
}

#[cfg(test)]
mod day15_tests {
    use std::fs;

    use crate::y2018::day15::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(27828, part_a(fs::read_to_string("input/2018/day15/test.txt").unwrap()));
        assert_eq!(27730, part_a(fs::read_to_string("input/2018/day15/test_27730.txt").unwrap()));
        assert_eq!(36334, part_a(fs::read_to_string("input/2018/day15/test_36334.txt").unwrap()));
        assert_eq!(39514, part_a(fs::read_to_string("input/2018/day15/test_39514.txt").unwrap()));
        assert_eq!(27755, part_a(fs::read_to_string("input/2018/day15/test_27755.txt").unwrap()));
        assert_eq!(28944, part_a(fs::read_to_string("input/2018/day15/test_28944.txt").unwrap()));
        assert_eq!(18740, part_a(fs::read_to_string("input/2018/day15/test_18740.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(250648, part_a(fs::read_to_string("input/2018/day15/input.txt").unwrap()));
        assert_eq!(42224, part_b(fs::read_to_string("input/2018/day15/input.txt").unwrap()));
    }
}
