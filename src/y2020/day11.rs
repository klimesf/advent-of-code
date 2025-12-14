use crate::utils::grid::{
    Grid, DOWN, DOWN_LEFT, DOWN_RIGHT, LEFT, P, RIGHT, UP, UP_LEFT, UP_RIGHT,
};
use std::fs;

pub(crate) fn day11() {
    println!(
        "{}",
        part_a(fs::read_to_string("input/2020/day11/input.txt").unwrap())
    );
    println!(
        "{}",
        part_b(fs::read_to_string("input/2020/day11/input.txt").unwrap())
    );
}

fn part_a(input: String) -> usize {
    let mut map = Grid::parse(&input.trim());

    loop {
        let mut changed = 0;
        let mut new_map = map.clone();

        for x in 0..map.x_len {
            for y in 0..map.y_len {
                let p = P::new(x, y);

                let mut occupied = 0;
                for dir in [
                    UP, UP_RIGHT, RIGHT, DOWN_RIGHT, DOWN, DOWN_LEFT, LEFT, UP_LEFT,
                ] {
                    let pos = p + dir;
                    if !map.contains(&pos) {
                        continue;
                    }
                    if map[pos] == b'#' {
                        occupied += 1;
                    }
                }

                match map[p] {
                    b'L' => {
                        if occupied == 0 {
                            new_map[(x, y)] = b'#';
                            changed += 1;
                        }
                    }
                    b'#' => {
                        if occupied >= 4 {
                            new_map[(x, y)] = b'L';
                            changed += 1;
                        }
                    }
                    b'.' => {}
                    _ => panic!("Invalid character '{}'", map[p]),
                }
            }
        }

        map = new_map;
        if changed == 0 {
            break;
        }
    }

    map.items.iter().filter(|&&c| c == b'#').count()
}

fn part_b(input: String) -> usize {
    let mut map = Grid::parse(&input.trim());

    loop {
        let mut changed = 0;
        let mut new_map = map.clone();

        for x in 0..map.x_len {
            for y in 0..map.y_len {
                let p = P::new(x, y);

                let mut occupied = 0;
                for dir in [
                    UP, UP_RIGHT, RIGHT, DOWN_RIGHT, DOWN, DOWN_LEFT, LEFT, UP_LEFT,
                ] {
                    let mut pos = p + dir;
                    loop {
                        if !map.contains(&pos) {
                            break;
                        }
                        match map[pos] {
                            b'L' => break,
                            b'#' => {
                                occupied += 1;
                                break;
                            }
                            b'.' => pos += dir,
                            _ => panic!("Invalid character '{}'", map[pos]),
                        }
                    }
                }

                match map[p] {
                    b'L' => {
                        if occupied == 0 {
                            new_map[(x, y)] = b'#';
                            changed += 1;
                        }
                    }
                    b'#' => {
                        if occupied >= 5 {
                            new_map[(x, y)] = b'L';
                            changed += 1;
                        }
                    }
                    b'.' => {}
                    _ => panic!("Invalid character '{}'", map[p]),
                }
            }
        }

        map = new_map;
        if changed == 0 {
            break;
        }
    }

    map.items.iter().filter(|&&c| c == b'#').count()
}

#[cfg(test)]
mod day11_tests {
    use std::fs;

    use crate::y2020::day11::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(
            37,
            part_a(fs::read_to_string("input/2020/day11/test.txt").unwrap())
        );
        assert_eq!(
            26,
            part_b(fs::read_to_string("input/2020/day11/test.txt").unwrap())
        );
    }

    #[test]
    fn input_works() {
        assert_eq!(
            2438,
            part_a(fs::read_to_string("input/2020/day11/input.txt").unwrap())
        );
        assert_eq!(
            2174,
            part_b(fs::read_to_string("input/2020/day11/input.txt").unwrap())
        );
    }
}
