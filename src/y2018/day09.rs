use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

pub(crate) fn day09() {
    let players = 446;
    let last_marble = 71522;
    println!("{}", get_highscore(players, last_marble));

    let players = 446;
    let last_marble = 71522 * 100;
    println!("{}", get_highscore(players, last_marble));
}

fn get_highscore(players: usize, last_marble: usize) -> usize {
    let mut circle = VecDeque::with_capacity(last_marble);
    circle.push_front(0);
    let mut elf = 0;

    let mut scores: HashMap<usize, usize> = HashMap::new();

    for marble_queue in 1..=last_marble {
        if marble_queue % 23 == 0 {
            for _ in 0..7 {
                // Rotate counter clockwise 7 times
                let i = circle.pop_back().unwrap();
                circle.push_front(i);
            }
            let removed = circle.pop_back().unwrap();
            *scores.entry(elf).or_insert(0) += marble_queue + removed;

            // Rotate clockwise
            let x = circle.pop_front().unwrap();
            circle.push_back(x);
        } else {
            let i = circle.pop_front().unwrap();
            circle.push_back(i); // Rotate clockwise
            circle.push_back(marble_queue);
        }

        elf = (elf + 1) % players;
    }

    return match scores.values().max() {
        Some(high_score) => *high_score,
        None => 0,
    };
}

#[allow(dead_code)]
fn print_circle(circle: &VecDeque<usize>, elf: &usize) {
    println!("[{}] {}", elf, circle.iter().join(" "));
}

#[cfg(test)]
mod day09_tests {
    use crate::y2018::day09::get_highscore;

    #[test]
    fn get_highscore_works() {
        assert_eq!(32, get_highscore(9, 25));
        assert_eq!(8317, get_highscore(10, 1618));
        assert_eq!(146373, get_highscore(13, 7999));
        assert_eq!(2764, get_highscore(17, 1104));
        assert_eq!(54718, get_highscore(21, 6111));
        assert_eq!(37305, get_highscore(30, 5807));
    }
}
