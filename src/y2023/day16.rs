use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use std::collections::HashSet;
use std::fs;

pub(crate) fn day16() {
    println!(
        "{}",
        part_a(fs::read_to_string("input/2023/day16/input.txt").unwrap())
    );
    println!(
        "{}",
        part_b(fs::read_to_string("input/2023/day16/input.txt").unwrap())
    );
}

fn part_a(input: String) -> usize {
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    calc_beam(&matrix, 0, 0, 90)
}

fn part_b(input: String) -> usize {
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut starts: Vec<(usize, usize, usize)> = vec![];
    for r in 0..matrix.len() {
        starts.push((r, 0, 90));
        starts.push((r, matrix[r].len() - 1, 270));
    }
    for c in 0..matrix.len() {
        starts.push((0, c, 180));
        starts.push((matrix.len() - 1, c, 0));
    }

    starts
        .par_iter()
        .map(|(sr, sc, sdir)| calc_beam(&matrix, *sr, *sc, *sdir))
        .max()
        .unwrap()
}

fn calc_beam(matrix: &Vec<Vec<char>>, sr: usize, sc: usize, sdir: usize) -> usize {
    let mut energized: HashSet<(usize, usize)> = HashSet::new();
    let mut seen: HashSet<(usize, usize, usize)> = HashSet::new();
    let mut beams = vec![];
    beams.push((sr, sc, sdir));
    for _i in 0..1000 {
        let mut new_beams: Vec<(usize, usize, usize)> = vec![];
        for (r, c, dir) in &beams {
            energized.insert((*r, *c));
            seen.insert((*r, *c, *dir));
            match dir {
                90 => {
                    let curr_c = matrix[*r][*c];
                    match curr_c {
                        '.' => {
                            if *c < (matrix[*r].len() - 1) {
                                new_beams.push((*r, c + 1, 90))
                            }
                        }
                        '-' => {
                            if *c < (matrix[*r].len() - 1) {
                                new_beams.push((*r, c + 1, 90))
                            }
                        }
                        '|' => {
                            if *r > 0 {
                                new_beams.push((r - 1, *c, 0))
                            }
                            if *r < matrix.len() - 1 {
                                new_beams.push((r + 1, *c, 180))
                            }
                        }
                        '/' => {
                            if *r > 0 {
                                new_beams.push((r - 1, *c, 0))
                            }
                        }
                        '\\' => {
                            if *r < (matrix.len() - 1) {
                                new_beams.push((r + 1, *c, 180))
                            }
                        }
                        _ => panic!("Unknown {}", c),
                    }
                }
                270 => {
                    let curr_c = matrix[*r][*c];
                    match curr_c {
                        '.' => {
                            if *c > 0 {
                                new_beams.push((*r, c - 1, 270))
                            }
                        }
                        '-' => {
                            if *c > 0 {
                                new_beams.push((*r, c - 1, 270))
                            }
                        }
                        '|' => {
                            if *r > 0 {
                                new_beams.push((r - 1, *c, 0))
                            }
                            if *r < matrix.len() - 1 {
                                new_beams.push((r + 1, *c, 180))
                            }
                        }
                        '/' => {
                            if *r < matrix.len() - 1 {
                                new_beams.push((r + 1, *c, 180))
                            }
                        }
                        '\\' => {
                            if *r > 0 {
                                new_beams.push((r - 1, *c, 0))
                            }
                        }
                        _ => panic!("Unknown {}", c),
                    }
                }
                0 => {
                    let curr_c = matrix[*r][*c];
                    match curr_c {
                        '.' => {
                            if *r > 0 {
                                new_beams.push((r - 1, *c, 0))
                            }
                        }
                        '-' => {
                            if *c > 0 {
                                new_beams.push((*r, c - 1, 270))
                            }
                            if *c < matrix[*r].len() - 1 {
                                new_beams.push((*r, c + 1, 90))
                            }
                        }
                        '|' => {
                            if *r > 0 {
                                new_beams.push((r - 1, *c, 0))
                            }
                        }
                        '/' => {
                            if *c < matrix[*r].len() - 1 {
                                new_beams.push((*r, c + 1, 90))
                            }
                        }
                        '\\' => {
                            if *c > 0 {
                                new_beams.push((*r, c - 1, 270))
                            }
                        }
                        _ => panic!("Unknown {}", c),
                    }
                }
                180 => {
                    let curr_c = matrix[*r][*c];
                    match curr_c {
                        '.' => {
                            if *r < matrix[*r].len() - 1 {
                                new_beams.push((r + 1, *c, 180))
                            }
                        }
                        '-' => {
                            if *c > 0 {
                                new_beams.push((*r, c - 1, 270))
                            }
                            if *c < matrix[*r].len() - 1 {
                                new_beams.push((*r, c + 1, 90))
                            }
                        }
                        '|' => {
                            if *r < matrix[*r].len() - 1 {
                                new_beams.push((r + 1, *c, 180))
                            }
                        }
                        '/' => {
                            if *c > 0 {
                                new_beams.push((*r, c - 1, 270))
                            }
                        }
                        '\\' => {
                            if *c < matrix[*r].len() - 1 {
                                new_beams.push((*r, c + 1, 90))
                            }
                        }
                        _ => panic!("Unknown {}", c),
                    }
                }
                _ => panic!("unknown dir {}", dir),
            }
        }

        beams = new_beams
            .iter()
            .filter(|(r, c, dir)| !seen.contains(&(*r, *c, *dir)))
            .map(|x| *x)
            .collect()
    }

    energized.len()
}

#[cfg(test)]
mod day16_tests {
    use std::fs;

    use crate::y2023::day16::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(
            46,
            part_a(fs::read_to_string("input/2023/day16/test.txt").unwrap())
        );
        assert_eq!(
            51,
            part_b(fs::read_to_string("input/2023/day16/test.txt").unwrap())
        );
    }

    #[test]
    fn input_works() {
        assert_eq!(
            6978,
            part_a(fs::read_to_string("input/2023/day16/input.txt").unwrap())
        );
        assert_eq!(
            7315,
            part_b(fs::read_to_string("input/2023/day16/input.txt").unwrap())
        );
    }
}
