use std::cmp::Ordering;
use std::collections::{BinaryHeap};

pub(crate) fn day22() {
    println!("{}", wizard_simulator_20xx(50, 500, 55, 8, false));
    println!("{}", wizard_simulator_20xx(50, 500, 55, 8, true));
}

fn wizard_simulator_20xx(player_hp: i32, player_mana: i32, boss_hp: i32, boss_dmg: i32, hard_difficulty: bool) -> usize {
    let mut stack = BinaryHeap::new();
    stack.push(GameState {
        player_hp,
        player_mana,
        boss_hp,
        shield_timer: 0,
        poison_timer: 0,
        recharge_timer: 0,
        turn: 0,
        mana_spent: 0,
    });

    let mut min = usize::MAX;
    while let Some(mut state) = stack.pop() {
        if hard_difficulty && state.turn % 2 == 0 {
            state.player_hp -= 1;
            if state.player_hp <= 0 { continue; }
        }

        let shield_effect = if state.shield_timer > 0 { 7 } else { 0 };
        let poison_effect = if state.poison_timer > 0 { 3 } else { 0 };
        let recharge_effect = if state.recharge_timer > 0 { 101 } else { 0 };
        state.shield_timer = 0.max(state.shield_timer - 1);
        state.poison_timer = 0.max(state.poison_timer - 1);
        state.recharge_timer = 0.max(state.recharge_timer - 1);

        state.player_mana += recharge_effect;
        state.boss_hp -= poison_effect;

        if state.player_hp <= 0 { continue; }
        if state.boss_hp <= 0 { min = state.mana_spent.min(min); break; }

        if state.turn % 2 == 0 {
            // Player move
            if state.player_mana >= 53 {
                // Magic missile
                stack.push(GameState {
                    player_hp: state.player_hp,
                    player_mana: state.player_mana - 53,
                    boss_hp: state.boss_hp - 4,
                    shield_timer: state.shield_timer,
                    poison_timer: state.poison_timer,
                    recharge_timer: state.recharge_timer,
                    mana_spent: state.mana_spent + 53,
                    turn: state.turn + 1,
                })
            }
            if state.player_mana >= 73 {
                // Drain
                stack.push(GameState {
                    player_hp: player_hp.min(state.player_hp + 2), // Cannot overheal
                    player_mana: state.player_mana - 73,
                    boss_hp: state.boss_hp - 2,
                    shield_timer: state.shield_timer,
                    poison_timer: state.poison_timer,
                    recharge_timer: state.recharge_timer,
                    mana_spent: state.mana_spent + 73,
                    turn: state.turn + 1,
                })
            }
            if state.player_mana >= 113 {
                // Shield
                stack.push(GameState {
                    player_hp: state.player_hp,
                    player_mana: state.player_mana - 113,
                    boss_hp: state.boss_hp,
                    shield_timer: 6,
                    poison_timer: state.poison_timer,
                    recharge_timer: state.recharge_timer,
                    mana_spent: state.mana_spent + 113,
                    turn: state.turn + 1,
                })
            }
            if state.player_mana >= 173 {
                // Poison
                stack.push(GameState {
                    player_hp: state.player_hp,
                    player_mana: state.player_mana - 173,
                    boss_hp: state.boss_hp,
                    shield_timer: state.shield_timer,
                    poison_timer: 6,
                    recharge_timer: state.recharge_timer,
                    mana_spent: state.mana_spent + 173,
                    turn: state.turn + 1,
                })
            }
            if state.player_mana >= 229 {
                // Recharge
                stack.push(GameState {
                    player_hp: state.player_hp,
                    player_mana: state.player_mana - 229,
                    boss_hp: state.boss_hp,
                    shield_timer: state.shield_timer,
                    poison_timer: state.poison_timer,
                    recharge_timer: 5,
                    mana_spent: state.mana_spent + 229,
                    turn: state.turn + 1,
                })
            }

        } else {
            stack.push(GameState {
                player_hp: state.player_hp - 1.max(boss_dmg - shield_effect),
                player_mana: state.player_mana,
                boss_hp: state.boss_hp,
                shield_timer: state.shield_timer,
                poison_timer: state.poison_timer,
                recharge_timer: state.recharge_timer,
                mana_spent: state.mana_spent,
                turn: state.turn + 1,
            });
        }
    }

    min
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct GameState {
    player_hp: i32,
    player_mana: i32,
    boss_hp: i32,
    shield_timer: i32,
    poison_timer: i32,
    recharge_timer: i32,
    mana_spent: usize,
    turn: usize,
}

impl Ord for GameState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.mana_spent.cmp(&self.mana_spent)
    }
}

impl PartialOrd for GameState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod day22_tests {
    use crate::y2015::day22::{wizard_simulator_20xx};

    #[test]
    fn test_works() {
        // assert_eq!(173 + 53, part_a(10, 250, 13, 8));
        // assert_eq!(229 + 113 + 73 + 173 + 53, part_a(10, 250, 14, 8));
        // assert_eq!(0, part_b());
    }

    #[test]
    fn input_works() {
        assert_eq!(953, wizard_simulator_20xx(50, 500, 55, 8, false));
        assert_eq!(0, wizard_simulator_20xx(50, 500, 55, 8, true));
    }
}
