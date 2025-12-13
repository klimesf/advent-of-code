use std::fs;

pub(crate) fn day15() {
    println!(
        "{}",
        part_a(fs::read_to_string("input/2023/day15/input.txt").unwrap())
    );
    println!(
        "{}",
        part_b(fs::read_to_string("input/2023/day15/input.txt").unwrap())
    );
}

fn part_a(input: String) -> u32 {
    input
        .split(",")
        .map(|seq| {
            let mut ans = 0;
            for c in seq.chars() {
                let ci = c as u32;
                ans += ci;
                ans *= 17;
                ans = ans % 256;
            }
            ans
        })
        .sum()
}

fn part_b(input: String) -> usize {
    let mut boxes: Vec<Vec<(&str, usize)>> = vec![vec!(); 256];
    input.split(",").for_each(|seq| {
        let mut label = String::new();
        let mut ans = 0;
        for c in seq.chars() {
            if c == '-' || c == '=' {
                break;
            }
            label.push(c);

            let ci = c as usize;
            ans += ci;
            ans *= 17;
            ans = ans % 256;
        }

        if seq.ends_with("-") {
            boxes[ans] = boxes[ans]
                .iter()
                .filter(|(l2, _)| l2 != &label.as_str())
                .map(|v| *v)
                .collect();
        } else {
            let (label, focal_length) = seq.split_once("=").unwrap();
            if let Some(pos) = boxes[ans].iter().position(|(l2, _)| l2 == &label) {
                boxes[ans][pos] = (label, focal_length.parse::<usize>().unwrap());
            } else {
                boxes[ans].push((label, focal_length.parse::<usize>().unwrap()));
            }
        }
    });

    let mut ans = 0;
    for box_i in 0..256 {
        for slot_i in 0..boxes[box_i].len() {
            let (_label, focal_length) = boxes[box_i][slot_i];
            ans += (box_i + 1) * (slot_i + 1) * focal_length;
        }
    }
    ans
}

#[cfg(test)]
mod day15_tests {
    use std::fs;

    use crate::y2023::day15::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(
            1320,
            part_a(fs::read_to_string("input/2023/day15/test.txt").unwrap())
        );
        assert_eq!(
            145,
            part_b(fs::read_to_string("input/2023/day15/test.txt").unwrap())
        );
    }

    #[test]
    fn input_works() {
        assert_eq!(
            495972,
            part_a(fs::read_to_string("input/2023/day15/input.txt").unwrap())
        );
        assert_eq!(
            245223,
            part_b(fs::read_to_string("input/2023/day15/input.txt").unwrap())
        );
    }
}
