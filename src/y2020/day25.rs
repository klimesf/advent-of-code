use std::fs;

pub(crate) fn day25() {
    println!("{}", part_a(fs::read_to_string("input/2020/day25/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    let (door_s, card_s) = input.split_once("\n").unwrap();
    let door_pk = door_s.parse::<usize>().unwrap();
    let card_pk = card_s.parse::<usize>().unwrap();

    let mut value = 1;
    let mut door_loop_size = 0;
    loop {
        value *= 7;
        value %= 20201227;
        door_loop_size += 1;
        if value == door_pk {
            break;
        }
    }

    value = 1;
    for _ in 0..door_loop_size {
        value *= card_pk;
        value %= 20201227;
    }

    value
}

#[cfg(test)]
mod day25_tests {
    use std::fs;

    use crate::y2020::day25::part_a;

    #[test]
    fn test_works() {
        assert_eq!(14897079, part_a(fs::read_to_string("input/2020/day25/test.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(6408263, part_a(fs::read_to_string("input/2020/day25/input.txt").unwrap()));
    }
}
