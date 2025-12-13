use std::fs;

pub(crate) fn day21() {
    let input = fs::read_to_string("input/2017/day21/input.txt").unwrap();
    let rules = parse(input.as_str());
    println!("{:?}", enhance(5, &rules));
    println!("{:?}", enhance(18, &rules));
}

fn parse(input: &str) -> Vec<(Vec<Vec<char>>, Vec<Vec<char>>)> {
    input
        .lines()
        .map(|line| {
            let (pattern_str, result_str) = line.split_once(" => ").unwrap();
            let pattern = pattern_str
                .split("/")
                .map(|row| row.chars().collect())
                .collect::<Vec<Vec<char>>>();
            let result = result_str
                .split("/")
                .map(|row| row.chars().collect())
                .collect::<Vec<Vec<char>>>();
            (pattern, result)
        })
        .flat_map(|(pattern, result)| {
            let mut res = vec![];

            let mut p = pattern;
            res.push((p.clone(), result.clone()));
            res.push((flip(&p), result.clone()));

            // Rotate to 90, 180 and 270
            for _ in 0..3 {
                p = rotate(&p);
                res.push((p.clone(), result.clone()));
                res.push((flip(&p), result.clone()));
            }

            res
        })
        .collect()
}

fn rotate(image: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    if image.len() == 2 {
        let mut result = vec![vec![' '; 2]; 2];
        result[0][0] = image[1][0];
        result[0][1] = image[0][0];
        result[1][0] = image[1][1];
        result[1][1] = image[0][1];
        result
    } else if image.len() == 3 {
        let mut result = vec![vec![' '; 3]; 3];
        result[0][0] = image[2][0];
        result[0][1] = image[1][0];
        result[0][2] = image[0][0];
        result[1][0] = image[2][1];
        result[1][1] = image[1][1];
        result[1][2] = image[0][1];
        result[2][0] = image[2][2];
        result[2][1] = image[1][2];
        result[2][2] = image[0][2];
        result
    } else {
        panic!("Unsupported image size {}", image.len())
    }
}

fn flip(image: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    image
        .iter()
        .map(|row| {
            let mut res = row.clone();
            res.reverse();
            res
        })
        .collect()
}

fn enhance(iterations: i32, rules: &Vec<(Vec<Vec<char>>, Vec<Vec<char>>)>) -> usize {
    let mut image = vec![
        vec!['.', '#', '.'],
        vec!['.', '.', '#'],
        vec!['#', '#', '#'],
    ];

    for _ in 0..iterations {
        let inc: usize = if image.len() % 2 == 0 { 2 } else { 3 };
        let mut squares: Vec<Vec<Vec<char>>> = vec![];
        for dx in 0..(image.len() / inc) {
            for dy in 0..(image.len() / inc) {
                let mut square = vec![];
                for x in 0..inc {
                    let mut row = vec![];
                    for y in 0..inc {
                        row.push(image[x + inc * dx][y + inc * dy]);
                    }
                    square.push(row);
                }
                squares.push(square);
            }
        }

        let mut new_squares = vec![];
        'outer: for square in squares {
            for (pattern, result) in rules {
                if square == *pattern {
                    new_squares.push(result.clone());
                    continue 'outer;
                }
            }
            panic!("No matching pattern");
        }

        let mut new_image = vec![];
        for dx in 0..(image.len() / inc) {
            let mut new_rows = vec![vec!(); inc + 1];
            for dy in 0..(image.len() / inc) {
                let square = &new_squares[(image.len() / inc) * dx + dy];
                for x in 0..=inc {
                    for y in 0..=inc {
                        new_rows[x].push(square[x][y]);
                    }
                }
            }
            new_image.append(&mut new_rows);
        }

        image = new_image;
    }

    image
        .iter()
        .map(|row| row.iter().filter(|c| **c == '#').count())
        .sum()
}

#[cfg(test)]
mod day21_tests {
    use crate::y2017::day21::{enhance, flip, parse, rotate};
    use std::fs;

    #[test]
    fn rotate_len2_works() {
        let original = vec![vec!['.', '.'], vec!['.', '#']];
        let rot_90 = vec![vec!['.', '.'], vec!['#', '.']];
        let rot_180 = vec![vec!['#', '.'], vec!['.', '.']];
        let rot_270 = vec![vec!['.', '#'], vec!['.', '.']];

        assert_eq!(rot_90, rotate(&original));
        assert_eq!(rot_180, rotate(&rot_90));
        assert_eq!(rot_270, rotate(&rot_180));
        assert_eq!(original, rotate(&rot_270));
    }

    #[test]
    fn rotate_len3_works() {
        let original = vec![
            vec!['.', '#', '.'],
            vec!['.', '.', '#'],
            vec!['#', '#', '#'],
        ];
        let rot_90 = vec![
            vec!['#', '.', '.'],
            vec!['#', '.', '#'],
            vec!['#', '#', '.'],
        ];
        let rot_180 = vec![
            vec!['#', '#', '#'],
            vec!['#', '.', '.'],
            vec!['.', '#', '.'],
        ];
        let rot_270 = vec![
            vec!['.', '#', '#'],
            vec!['#', '.', '#'],
            vec!['.', '.', '#'],
        ];

        assert_eq!(rot_90, rotate(&original));
        assert_eq!(rot_180, rotate(&rot_90));
        assert_eq!(rot_270, rotate(&rot_180));
        assert_eq!(original, rotate(&rot_270));
    }

    #[test]
    fn flip_len2_works() {
        let original = vec![vec!['.', '.'], vec!['.', '#']];
        let flipped = vec![vec!['.', '.'], vec!['#', '.']];
        assert_eq!(flipped, flip(&original));
    }

    #[test]
    fn flip_len3_works() {
        let original = vec![
            vec!['.', '.', '#'],
            vec!['.', '#', '.'],
            vec!['#', '.', '.'],
        ];
        let flipped = vec![
            vec!['#', '.', '.'],
            vec!['.', '#', '.'],
            vec!['.', '.', '#'],
        ];
        assert_eq!(flipped, flip(&original));
    }

    #[test]
    fn test_works() {
        let input = fs::read_to_string("input/2017/day21/test.txt").unwrap();
        let rules = parse(input.as_str());
        assert_eq!(12, enhance(2, &rules));
    }

    #[test]
    fn input_works() {
        let input = fs::read_to_string("input/2017/day21/input.txt").unwrap();
        let rules = parse(input.as_str());
        assert_eq!(167, enhance(5, &rules));
        assert_eq!(2425195, enhance(18, &rules));
    }
}
