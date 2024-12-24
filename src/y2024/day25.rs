use std::fs;

pub(crate) fn day25() {
    println!("{}", part_a(fs::read_to_string("input/2024/day25/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    let mut locks: Vec<Vec<usize>> = vec!();
    let mut keys: Vec<Vec<usize>> = vec!();

    input.split("\n\n").for_each(|schema| {
        let lines = schema.lines().collect::<Vec<&str>>();
        let matrix = lines.iter()
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<char>>>();

        if lines[0] == "#####" {
            let mut lock = vec!();
            for j in 0..5 {
                let mut height = 0;
                for i in 1..7 {
                    if matrix[i][j] == '#' {
                        height += 1;
                    }
                }
                lock.push(height);
            }
            locks.push(lock);
        } else if lines[6] == "#####" {
            let mut key = vec!();
            for j in 0..5 {
                let mut height = 0;
                for i in (0..6).rev() {
                    if matrix[i][j] == '#' {
                        height += 1;
                    }
                }
                key.push(height);
            }
            keys.push(key);
        } else {
            panic!();
        }
    });

    let mut ans = 0;
    for i in 0..locks.len() {
        for j in 0..keys.len() {
            let mut matches = true;
            let lock = &locks[i];
            let key = &keys[j];
            for c in 0..5 {
                if lock[c] + key[c] > 5 {
                    matches = false;
                    break;
                }
            }

            if matches {
                ans += 1;
            }
        }
    }
    ans
}

#[cfg(test)]
mod day25_tests {
    use std::fs;

    use crate::y2024::day25::part_a;

    #[test]
    fn test_works() {
        assert_eq!(3, part_a(fs::read_to_string("input/2024/day25/test.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(3690, part_a(fs::read_to_string("input/2024/day25/input.txt").unwrap()));
    }
}
