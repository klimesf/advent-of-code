use std::fs;

pub(crate) fn day18() {
    println!(
        "{}",
        part_a(
            fs::read_to_string("input/2015/day18/input.txt").unwrap(),
            100
        )
    );
    println!(
        "{}",
        part_b(
            fs::read_to_string("input/2015/day18/input.txt").unwrap(),
            100
        )
    );
}

fn part_a(input: String, steps: usize) -> usize {
    let mut lights: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let size = lights.len() - 1;

    for _ in 0..steps {
        let mut new_lights = lights.clone();
        for r in 0..=size {
            for c in 0..=size {
                let mut neighbor_count = 0;
                for dr in [-1, 0, 1] {
                    for dc in [-1, 0, 1] {
                        if dr == 0 && dc == 0 {
                            continue;
                        }
                        if r as i32 + dr < 0 || r as i32 + dr > size as i32 {
                            continue;
                        }
                        if c as i32 + dc < 0 || c as i32 + dc > size as i32 {
                            continue;
                        }
                        if lights[(r as i32 + dr) as usize][(c as i32 + dc) as usize] == '#' {
                            neighbor_count += 1
                        }
                    }
                }

                if lights[r][c] == '#' {
                    new_lights[r][c] = if neighbor_count == 2 || neighbor_count == 3 {
                        '#'
                    } else {
                        '.'
                    }
                } else {
                    new_lights[r][c] = if neighbor_count == 3 { '#' } else { '.' }
                }
            }
        }
        lights = new_lights;
    }

    lights
        .iter()
        .map(|row| row.iter().filter(|c| **c == '#').count())
        .sum()
}

fn part_b(input: String, steps: usize) -> usize {
    let mut lights: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let size = lights.len() - 1;
    lights[0][0] = '#';
    lights[0][size] = '#';
    lights[size][0] = '#';
    lights[size][size] = '#';

    for _ in 0..steps {
        let mut new_lights = lights.clone();
        for r in 0..=size {
            for c in 0..=size {
                if (r == 0 && c == 0)
                    || (r == 0 && c == size)
                    || (r == size && c == 0)
                    || (r == size && c == size)
                {
                    new_lights[r][c] = '#';
                    continue;
                }

                let mut neighbor_count = 0;
                for dr in [-1, 0, 1] {
                    for dc in [-1, 0, 1] {
                        if dr == 0 && dc == 0 {
                            continue;
                        }
                        if r as i32 + dr < 0 || r as i32 + dr > size as i32 {
                            continue;
                        }
                        if c as i32 + dc < 0 || c as i32 + dc > size as i32 {
                            continue;
                        }
                        if lights[(r as i32 + dr) as usize][(c as i32 + dc) as usize] == '#' {
                            neighbor_count += 1
                        }
                    }
                }

                if lights[r][c] == '#' {
                    new_lights[r][c] = if neighbor_count == 2 || neighbor_count == 3 {
                        '#'
                    } else {
                        '.'
                    }
                } else {
                    new_lights[r][c] = if neighbor_count == 3 { '#' } else { '.' }
                }
            }
        }
        lights = new_lights;
    }

    lights
        .iter()
        .map(|row| row.iter().filter(|c| **c == '#').count())
        .sum()
}

#[cfg(test)]
mod day18_tests {
    use std::fs;

    use crate::y2015::day18::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(
            4,
            part_a(fs::read_to_string("input/2015/day18/test.txt").unwrap(), 4)
        );
        assert_eq!(
            17,
            part_b(fs::read_to_string("input/2015/day18/test.txt").unwrap(), 5)
        );
    }

    #[test]
    fn input_works() {
        assert_eq!(
            814,
            part_a(
                fs::read_to_string("input/2015/day18/input.txt").unwrap(),
                100
            )
        );
        assert_eq!(
            924,
            part_b(
                fs::read_to_string("input/2015/day18/input.txt").unwrap(),
                100
            )
        );
    }
}
