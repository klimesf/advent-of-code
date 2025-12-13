use std::collections::HashMap;
use std::fs;

pub fn day12(print: fn(usize)) {
    print(part_a(
        fs::read_to_string("input/2024/day12/input.txt").unwrap(),
    ));
    print(part_b(
        fs::read_to_string("input/2024/day12/input.txt").unwrap(),
    ));
}

fn part_a(input: String) -> usize {
    let map = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let regions = build_regions(&map);

    let mut perimeter = HashMap::new();
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            *perimeter.entry(regions[i][j]).or_insert(0) += 1;
        }
    }

    let mut total = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if i > 0 && regions[i - 1][j] != regions[i][j] {
                total += perimeter.get(&regions[i][j]).unwrap()
            } else if i == 0 {
                total += perimeter.get(&regions[i][j]).unwrap()
            }

            if j > 0 && regions[i][j - 1] != regions[i][j] {
                total += perimeter.get(&regions[i][j]).unwrap()
            } else if j == 0 {
                total += perimeter.get(&regions[i][j]).unwrap()
            }

            if i < map.len() - 1 && regions[i + 1][j] != regions[i][j] {
                total += perimeter.get(&regions[i][j]).unwrap()
            } else if i == map.len() - 1 {
                total += perimeter.get(&regions[i][j]).unwrap()
            }

            if j < map[i].len() - 1 && regions[i][j + 1] != regions[i][j] {
                total += perimeter.get(&regions[i][j]).unwrap()
            } else if j == map[i].len() - 1 {
                total += perimeter.get(&regions[i][j]).unwrap()
            }
        }
    }

    total
}

fn part_b(input: String) -> usize {
    let map = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let regions = build_regions(&map);

    let mut sides = HashMap::new();
    for i in 0..=map.len() {
        for j in 0..=map[0].len() {
            let up_left = if i == 0 || j == 0 {
                -1
            } else {
                regions[i - 1][j - 1]
            };
            let up = if i == 0 || j == map[0].len() {
                -1
            } else {
                regions[i - 1][j]
            };
            let left = if j == 0 || i == map.len() {
                -1
            } else {
                regions[i][j - 1]
            };
            let here = if i == map.len() || j == map[0].len() {
                -1
            } else {
                regions[i][j]
            };

            // _*
            // *A
            if here != -1 && here != up && here != left {
                *sides.entry(regions[i][j]).or_insert(0) += 1;
            }

            // *_
            // A*
            if left != -1 && left != up_left && left != here {
                *sides.entry(regions[i][j - 1]).or_insert(0) += 1;
            }

            // A*
            // *_
            if up_left != -1 && up_left != up && up_left != left {
                *sides.entry(regions[i - 1][j - 1]).or_insert(0) += 1;
            }

            // *A
            // _*
            if up != -1 && up != up_left && up != here {
                *sides.entry(regions[i - 1][j]).or_insert(0) += 1;
            }

            // AA
            // A_
            if up_left != -1 && up_left == up && up_left == left && up_left != here {
                *sides.entry(regions[i - 1][j - 1]).or_insert(0) += 1;
            }

            // AA
            // _A
            if up != -1 && up == up_left && up == here && up != left {
                *sides.entry(regions[i - 1][j]).or_insert(0) += 1;
            }

            // _A
            // AA
            if here != -1 && here == up && here == left && here != up_left {
                *sides.entry(regions[i][j]).or_insert(0) += 1;
            }

            // A_
            // AA
            if left != -1 && left == up_left && left == here && left != up {
                *sides.entry(regions[i][j - 1]).or_insert(0) += 1;
            }
        }
    }

    let mut total = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            total += sides[&regions[i][j]];
        }
    }

    total
}

fn build_regions(map: &Vec<Vec<char>>) -> Vec<Vec<i32>> {
    let mut regions: Vec<Vec<i32>> = vec![vec![-1; map[0].len()]; map.len()];

    let mut id = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if regions[i][j] > -1 && regions[i][j] < id {
                continue;
            }

            regions[i][j] = id;
            let mut stack = vec![];

            if i > 0 {
                stack.push((i - 1, j, map[i][j]));
            }
            if j > 0 {
                stack.push((i, j - 1, map[i][j]));
            }
            if i < map.len() - 1 {
                stack.push((i + 1, j, map[i][j]));
            }
            if j < map[i].len() - 1 {
                stack.push((i, j + 1, map[i][j]));
            }

            while let Some((x, y, prev)) = stack.pop() {
                if map[x][y] != prev {
                    continue;
                }
                if regions[x][y] > -1 && regions[x][y] < id {
                    continue;
                }

                regions[x][y] = id;
                if x > 0 && regions[x - 1][y] == -1 {
                    stack.push((x - 1, y, map[x][y]));
                }
                if y > 0 && regions[x][y - 1] == -1 {
                    stack.push((x, y - 1, map[x][y]));
                }
                if x < map.len() - 1 && regions[x + 1][y] == -1 {
                    stack.push((x + 1, y, map[x][y]));
                }
                if y < map[y].len() - 1 && regions[x][y + 1] == -1 {
                    stack.push((x, y + 1, map[x][y]));
                }
            }
            id += 1;
        }
    }
    regions
}

#[cfg(test)]
mod day12_tests {
    use std::fs;

    use crate::y2024::day12::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(
            140,
            part_a(fs::read_to_string("input/2024/day12/test.txt").unwrap())
        );
        assert_eq!(
            772,
            part_a(fs::read_to_string("input/2024/day12/test_2.txt").unwrap())
        );
        assert_eq!(
            80,
            part_b(fs::read_to_string("input/2024/day12/test.txt").unwrap())
        );
        assert_eq!(
            436,
            part_b(fs::read_to_string("input/2024/day12/test_2.txt").unwrap())
        );
    }

    #[test]
    fn input_works() {
        assert_eq!(
            1431440,
            part_a(fs::read_to_string("input/2024/day12/input.txt").unwrap())
        );
        assert_eq!(
            869070,
            part_b(fs::read_to_string("input/2024/day12/input.txt").unwrap())
        );
    }
}
