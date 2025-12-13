use std::collections::HashMap;
use std::fs;

pub fn day11(print: fn(usize)) {
    print(solve(
        fs::read_to_string("input/2024/day11/input.txt").unwrap(),
        25,
    ));
    print(solve(
        fs::read_to_string("input/2024/day11/input.txt").unwrap(),
        75,
    ));
}

fn solve(input: String, blinks: usize) -> usize {
    let stones = input
        .split_whitespace()
        .map(|stone| stone.parse().unwrap())
        .collect::<Vec<usize>>();

    let mut cache = HashMap::new();
    stones
        .iter()
        .map(|stone| recursive_blink(*stone, blinks, &mut cache))
        .sum()
}

fn recursive_blink(
    stone: usize,
    blinks: usize,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if blinks == 0 {
        return 1;
    }
    if cache.contains_key(&(stone, blinks)) {
        return *cache.get(&(stone, blinks)).unwrap();
    }

    let mut total = 0;

    if stone == 0 {
        // Stone is 0 => new stone 1
        total += recursive_blink(1, blinks - 1, cache);
    } else {
        // don't use formatting into strings like some Kotlin bozo
        let digits = (0..)
            .take_while(|digit| 10_usize.pow(*digit) <= stone)
            .count();

        if digits % 2 == 0 {
            // Stone has an even number of digits, split into two
            let divisor = 10_usize.pow(digits as u32 / 2);
            total += recursive_blink(stone % divisor, blinks - 1, cache); // Left
            total += recursive_blink(stone / divisor, blinks - 1, cache); // Right
        } else {
            // Stone has an odd number of digits, multiply by 2024
            total += recursive_blink(stone * 2024, blinks - 1, cache);
        }
    }

    cache.insert((stone, blinks), total);
    total
}

#[cfg(test)]
mod day11_tests {
    use std::fs;

    use crate::y2024::day11::solve;

    #[test]
    fn test_works() {
        assert_eq!(
            55312,
            solve(fs::read_to_string("input/2024/day11/test.txt").unwrap(), 25)
        );
    }

    #[test]
    fn input_works() {
        assert_eq!(
            198089,
            solve(
                fs::read_to_string("input/2024/day11/input.txt").unwrap(),
                25
            )
        );
        assert_eq!(
            236302670835517,
            solve(
                fs::read_to_string("input/2024/day11/input.txt").unwrap(),
                75
            )
        );
    }
}
