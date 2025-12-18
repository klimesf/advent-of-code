use std::fs;

pub(crate) fn day21() {
    println!("{}", part_a(fs::read_to_string("input/2015/day21/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2015/day21/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    let (boss_hp, boss_dmg, boss_armor, combinations) = create_combinations(input);

    let mut min = usize::MAX;
    for player in combinations {
        let player_hp = 100;
        let player_dmg_to_boss = 1.max(player.1 - boss_armor);
        let boss_dmg_to_player = 1.max(boss_dmg - player.2);

        let player_dies_in = (player_hp + boss_dmg_to_player - 1) / boss_dmg_to_player;
        let boss_dies_in = (boss_hp + player_dmg_to_boss - 1) / player_dmg_to_boss;
        if player_dies_in >= boss_dies_in {
            min = player.0.min(min)
        }
    }
    min
}

fn part_b(input: String) -> usize {
    let (boss_hp, boss_dmg, boss_armor, combinations) = create_combinations(input);

    let mut max = 0;
    for player in combinations {
        let player_hp = 100;
        let player_dmg_to_boss = 1.max(player.1 - boss_armor);
        let boss_dmg_to_player = 1.max(boss_dmg - player.2);

        let player_dies_in = (player_hp + boss_dmg_to_player - 1) / boss_dmg_to_player;
        let boss_dies_in = (boss_hp + player_dmg_to_boss - 1) / player_dmg_to_boss;
        if player_dies_in < boss_dies_in {
            max = player.0.max(max)
        }
    }
    max
}

fn create_combinations(input: String) -> (i32, i32, i32, Vec<(usize, i32, i32)>) {
    let lines: Vec<&str> = input.lines().collect();
    let (_, right) = lines[0].split_once(": ").unwrap();
    let boss_hp = right.parse::<i32>().unwrap();
    let (_, right) = lines[1].split_once(": ").unwrap();
    let boss_dmg = right.parse::<i32>().unwrap();
    let (_, right) = lines[2].split_once(": ").unwrap();
    let boss_armor = right.parse::<i32>().unwrap();

    let weapons = vec![(8, 4, 0), (10, 5, 0), (25, 6, 0), (40, 7, 0), (74, 8, 0)];

    let armors = vec![
        (13, 0, 1),
        (31, 0, 2),
        (53, 0, 3),
        (75, 0, 4),
        (102, 0, 5),
        (0, 0, 0), // Dummy
    ];

    let rings = vec![
        (25, 1, 0),
        (50, 2, 0),
        (100, 3, 0),
        (20, 0, 1),
        (40, 0, 2),
        (80, 0, 3),
        (0, 0, 0), // Dummy
        (0, 0, 0), // Dummy
    ];

    let mut combinations = vec![];
    for wi in 0..weapons.len() {
        for ai in 0..armors.len() {
            for r1i in 0..rings.len() {
                for r2i in 0..rings.len() {
                    if r1i == r2i {
                        continue;
                    }

                    combinations.push((
                        weapons[wi].0 + armors[ai].0 + rings[r1i].0 + rings[r2i].0,
                        weapons[wi].1 + armors[ai].1 + rings[r1i].1 + rings[r2i].1,
                        weapons[wi].2 + armors[ai].2 + rings[r1i].2 + rings[r2i].2,
                    ));
                }
            }
        }
    }
    (boss_hp, boss_dmg, boss_armor, combinations)
}

#[cfg(test)]
mod day21_tests {
    use std::fs;

    use crate::y2015::day21::{part_a, part_b};

    #[test]
    fn input_works() {
        assert_eq!(121, part_a(fs::read_to_string("input/2015/day21/input.txt").unwrap()));
        assert_eq!(201, part_b(fs::read_to_string("input/2015/day21/input.txt").unwrap()));
    }
}
