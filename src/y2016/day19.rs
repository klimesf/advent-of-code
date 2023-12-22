pub(crate) fn day19() {
    println!("{}", part_a(3012210));
    println!("{}", part_b(3012210));
}

fn part_a(elves_count: usize) -> usize {
    let mut state = vec!();
    for _ in 0..elves_count {
        state.push(1);
    }

    let mut i = 0;
    loop {
        while state[i] == 0 {
            i = (i + 1) % elves_count;
        }
        let mut j = (i + 1) % elves_count;
        while state[j] == 0 {
            j = (j + 1) % elves_count;
        }

        if i == j { break }
        state[i] = state[i] + state[j];
        state[j] = 0;
        i = j;
    }

    i + 1 // elves are 1-indexed
}

fn part_b(elves_count: usize) -> usize {
    // let mut state = vec!();
    // for i in 0..elves_count {
    //     state.push(i + 1);
    // }
    //
    // let mut curr_player = 0;
    // while state.len() > 1 {
    //     if curr_player >= state.len() {
    //         curr_player = 0;
    //     }
    //     let from = (curr_player + (state.len() / 2)) % state.len();
    //     // println!("elf {} ({}) takes from elf {} ({})", state[curr_player], curr_player, state[from], from);
    //     let mut new_state = vec!();
    //     for i in 0..state.len() {
    //         if i == from { continue }
    //         new_state.push(state[i]);
    //     }
    //     if from > curr_player {
    //         curr_player = curr_player + 1;
    //     }
    //     state = new_state;
    // }
    //
    // state[0]

    let mut i = 1;
    while i * 3 < elves_count { i *= 3 }
    return elves_count - i
}

#[cfg(test)]
mod day19_tests {
    use crate::y2016::day19::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(3, part_a(5));

        assert_eq!(1, part_b(4));
        assert_eq!(2, part_b(5));
        assert_eq!(3, part_b(6));
    }

    #[test]
    fn input_works() {
        assert_eq!(1830117, part_a(3012210));
        assert_eq!(1417887, part_b(3012210));
    }

    #[test]
    fn theory_works() {
        for i in 2..=1000 {
            println!("{:3}: {:3}", i, part_b(i));
        }
    }
}
