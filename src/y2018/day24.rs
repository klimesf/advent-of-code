use crate::utils::toolbox::parse_usize;
use itertools::Itertools;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

pub(crate) fn day24() {
    println!("{}", part_a(fs::read_to_string("input/2018/day24/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2018/day24/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    let (immune_input, infection_input) = input.split_once("\n\n").unwrap();
    let mut immune_system: Vec<Group> = immune_input
        .lines()
        .enumerate()
        .filter(|(_, line)| !line.starts_with("Immune System:"))
        .map(|(i, line)| create_group(i, line))
        .collect();

    let mut infection: Vec<Group> = infection_input
        .lines()
        .enumerate()
        .filter(|(_, line)| !line.starts_with("Infection:"))
        .map(|(i, line)| create_group(i, line))
        .collect();

    // println!("{:?}", immune_system);
    // println!("{:?}", infection);

    loop {
        let immune_system_sum = immune_system.iter().map(|g| g.units).sum();
        let infection_sum = infection.iter().map(|g| g.units).sum();

        if immune_system_sum == 0 {
            return infection_sum;
        }
        if infection_sum == 0 {
            return immune_system_sum;
        }

        immune_system.sort_by(sort_groups_by_effective_power);
        infection.sort_by(sort_groups_by_effective_power);

        let mut infection_to_immune_system = HashMap::new();
        for attacker in &infection {
            if attacker.units < 1 {
                continue;
            }
            let mut max = (0, 0, 0, 0);
            for defender in &immune_system {
                if defender.units < 1
                    || defender.immunity.contains(&attacker.attack_type)
                    || infection_to_immune_system.values().contains(&defender.id)
                {
                    continue;
                }
                let damage = attacker.calc_damage(defender);
                // println!("Infection group {} would deal defending group {} {} damage", attacker.id, defender.id, damage);

                if damage > max.0
                    || (damage == max.0 && defender.effective_power() > max.1)
                    || (damage == max.0 && defender.effective_power() == max.1 && defender.initiative > max.1)
                {
                    max = (damage, defender.effective_power(), defender.initiative, defender.id);
                }
            }

            if max != (0, 0, 0, 0) {
                infection_to_immune_system.insert(attacker.id, max.3);
            }
        }

        // println!();

        let mut immune_system_to_infection = HashMap::new();
        for attacker in &immune_system {
            if attacker.units < 1 {
                continue;
            }
            let mut max = (0, 0, 0, 0);
            for defender in &infection {
                if defender.units < 1
                    || defender.immunity.contains(&attacker.attack_type)
                    || immune_system_to_infection.values().contains(&defender.id)
                {
                    continue;
                }
                let damage = attacker.calc_damage(defender);
                // println!("Immune system group {} would deal defending group {} {} damage", attacker.id, defender.id, damage);

                if damage > max.0
                    || (damage == max.0 && defender.effective_power() > max.1)
                    || (damage == max.0 && defender.effective_power() == max.1 && defender.initiative > max.1)
                {
                    max = (damage, defender.effective_power(), defender.initiative, defender.id);
                }

                if max != (0, 0, 0, 0) {
                    immune_system_to_infection.insert(attacker.id, max.3);
                }
            }
        }

        // println!();
        immune_system.sort_by(sort_groups_by_initiative);
        infection.sort_by(sort_groups_by_initiative);
        let mut i = 0;
        let mut j = 0;
        loop {
            if i >= immune_system.len() && j >= infection.len() {
                break;
            }
            if i >= immune_system.len() {
                // infection go
                let attacker = &infection[j];
                if let Some(defender_id) = infection_to_immune_system.get(&attacker.id) {
                    let (defender_i, _) = immune_system.iter().find_position(|g| g.id == *defender_id).unwrap();
                    let defender = &mut immune_system[defender_i];
                    // print!("Infection group {} attacks defending group {}, ", attacker.id, defender.id);
                    attacker.apply_damage(defender);
                }
                j += 1;
            } else if j >= infection.len() {
                // immune system go
                let attacker = &immune_system[i];
                if let Some(defender_id) = immune_system_to_infection.get(&attacker.id) {
                    let (defender_i, _) = infection.iter().find_position(|g| g.id == *defender_id).unwrap();
                    let defender = &mut infection[defender_i];
                    // print!("Immune system group {} attacks defending group {}, ", attacker.id, defender.id);
                    attacker.apply_damage(defender);
                }
                i += 1;
            } else {
                let a = &immune_system[i];
                let b = &infection[j];
                if a.initiative > b.initiative {
                    let attacker = &immune_system[i];
                    if let Some(defender_id) = immune_system_to_infection.get(&attacker.id) {
                        let (defender_i, _) = infection.iter().find_position(|g| g.id == *defender_id).unwrap();
                        let defender = &mut infection[defender_i];
                        // print!("Immune system group {} attacks defending group {}, ", attacker.id, defender.id);
                        attacker.apply_damage(defender);
                    }
                    i += 1;
                } else {
                    let attacker = &infection[j];
                    if let Some(defender_id) = infection_to_immune_system.get(&attacker.id) {
                        let (defender_i, _) = immune_system.iter().find_position(|g| g.id == *defender_id).unwrap();
                        let defender = &mut immune_system[defender_i];
                        // print!("Infection group {} attacks defending group {}, ", attacker.id, defender.id);
                        attacker.apply_damage(defender);
                    }
                    j += 1;
                }
            }
        }

        // println!("---");
        // return 0;
    }
}

fn part_b(input: String) -> usize {
    let (immune_input, infection_input) = input.split_once("\n\n").unwrap();
    let immune_system_og: Vec<Group> = immune_input
        .lines()
        .enumerate()
        .filter(|(_, line)| !line.starts_with("Immune System:"))
        .map(|(i, line)| create_group(i, line))
        .collect();

    let infection_og: Vec<Group> = infection_input
        .lines()
        .enumerate()
        .filter(|(_, line)| !line.starts_with("Infection:"))
        .map(|(i, line)| create_group(i, line))
        .collect();

    'outer: for boost in 55..10000 {
        // println!("{}", boost);

        let mut immune_system = vec![];
        immune_system_og.iter().for_each(|g| {
            let mut ng = g.clone();
            ng.attack += boost;
            immune_system.push(ng);
        });
        let mut infection = infection_og.clone();

        loop {
            let immune_system_sum = immune_system.iter().map(|g| g.units).sum();
            let infection_sum: usize = infection.iter().map(|g| g.units).sum();

            if immune_system_sum == 0 {
                continue 'outer;
            }
            if infection_sum == 0 {
                return immune_system_sum;
            }

            immune_system.sort_by(sort_groups_by_effective_power);
            infection.sort_by(sort_groups_by_effective_power);

            let mut infection_to_immune_system = HashMap::new();
            for attacker in &infection {
                if attacker.units < 1 {
                    continue;
                }
                let mut max = (0, 0, 0, 0);
                for defender in &immune_system {
                    if defender.units < 1
                        || defender.immunity.contains(&attacker.attack_type)
                        || infection_to_immune_system.values().contains(&defender.id)
                    {
                        continue;
                    }
                    let damage = attacker.calc_damage(defender);
                    // println!("Infection group {} would deal defending group {} {} damage", attacker.id, defender.id, damage);

                    if damage > max.0
                        || (damage == max.0 && defender.effective_power() > max.1)
                        || (damage == max.0 && defender.effective_power() == max.1 && defender.initiative > max.1)
                    {
                        max = (damage, defender.effective_power(), defender.initiative, defender.id);
                    }
                }

                if max != (0, 0, 0, 0) {
                    infection_to_immune_system.insert(attacker.id, max.3);
                }
            }

            // println!();

            let mut immune_system_to_infection = HashMap::new();
            for attacker in &immune_system {
                if attacker.units < 1 {
                    continue;
                }
                let mut max = (0, 0, 0, 0);
                for defender in &infection {
                    if defender.units < 1
                        || defender.immunity.contains(&attacker.attack_type)
                        || immune_system_to_infection.values().contains(&defender.id)
                    {
                        continue;
                    }
                    let damage = attacker.calc_damage(defender);
                    // println!("Immune system group {} would deal defending group {} {} damage", attacker.id, defender.id, damage);

                    if damage > max.0
                        || (damage == max.0 && defender.effective_power() > max.1)
                        || (damage == max.0 && defender.effective_power() == max.1 && defender.initiative > max.1)
                    {
                        max = (damage, defender.effective_power(), defender.initiative, defender.id);
                    }

                    if max != (0, 0, 0, 0) {
                        immune_system_to_infection.insert(attacker.id, max.3);
                    }
                }
            }

            // println!();
            immune_system.sort_by(sort_groups_by_initiative);
            infection.sort_by(sort_groups_by_initiative);
            let mut i = 0;
            let mut j = 0;
            loop {
                if i >= immune_system.len() && j >= infection.len() {
                    break;
                }
                if i >= immune_system.len() {
                    // infection go
                    let attacker = &infection[j];
                    if let Some(defender_id) = infection_to_immune_system.get(&attacker.id) {
                        let (defender_i, _) = immune_system.iter().find_position(|g| g.id == *defender_id).unwrap();
                        let defender = &mut immune_system[defender_i];
                        // print!("Infection group {} attacks defending group {}, ", attacker.id, defender.id);
                        attacker.apply_damage(defender);
                    }
                    j += 1;
                } else if j >= infection.len() {
                    // immune system go
                    let attacker = &immune_system[i];
                    if let Some(defender_id) = immune_system_to_infection.get(&attacker.id) {
                        let (defender_i, _) = infection.iter().find_position(|g| g.id == *defender_id).unwrap();
                        let defender = &mut infection[defender_i];
                        // print!("Immune system group {} attacks defending group {}, ", attacker.id, defender.id);
                        attacker.apply_damage(defender);
                    }
                    i += 1;
                } else {
                    let a = &immune_system[i];
                    let b = &infection[j];
                    if a.initiative > b.initiative {
                        let attacker = &immune_system[i];
                        if let Some(defender_id) = immune_system_to_infection.get(&attacker.id) {
                            let (defender_i, _) = infection.iter().find_position(|g| g.id == *defender_id).unwrap();
                            let defender = &mut infection[defender_i];
                            // print!("Immune system group {} attacks defending group {}, ", attacker.id, defender.id);
                            attacker.apply_damage(defender);
                        }
                        i += 1;
                    } else {
                        let attacker = &infection[j];
                        if let Some(defender_id) = infection_to_immune_system.get(&attacker.id) {
                            let (defender_i, _) = immune_system.iter().find_position(|g| g.id == *defender_id).unwrap();
                            let defender = &mut immune_system[defender_i];
                            // print!("Infection group {} attacks defending group {}, ", attacker.id, defender.id);
                            attacker.apply_damage(defender);
                        }
                        j += 1;
                    }
                }
            }
        }
    }
    panic!("no boost found")
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct Group {
    id: usize,
    units: usize,
    hp: usize,
    attack: usize,
    attack_type: String,
    immunity: Vec<String>,
    weakness: Vec<String>,
    initiative: usize,
}

impl Group {
    fn effective_power(&self) -> usize {
        self.units * self.attack
    }

    fn calc_damage(&self, defender: &Group) -> usize {
        let mut damage = self.attack * self.units;
        if defender.weakness.contains(&self.attack_type) {
            damage *= 2;
        }
        damage
    }

    fn apply_damage(&self, defender: &mut Group) {
        let damage = self.calc_damage(defender);
        let units_lost = damage / defender.hp;
        if units_lost > defender.units {
            // println!("killing {} units", defender.units);
            defender.units = 0;
        } else {
            // println!("killing {} units", units_lost);
            defender.units -= units_lost;
        }
    }
}

fn sort_groups_by_effective_power(a: &Group, b: &Group) -> Ordering {
    if a.effective_power() == b.effective_power() {
        b.initiative.cmp(&a.initiative)
    } else {
        b.effective_power().cmp(&a.effective_power())
    }
}

fn sort_groups_by_initiative(a: &Group, b: &Group) -> Ordering {
    b.initiative.cmp(&a.initiative)
}

fn create_group(i: usize, s: &str) -> Group {
    let re = Regex::new("^([0-9]+) units each with ([0-9]+) hit points (.+)?with an attack that does ([0-9]+) (.+) damage at initiative ([0-9]+)$").unwrap();
    let caps = re.captures(s).unwrap();

    let (immunity, weakness) = if let Some(a) = caps.get(3) {
        get_immunity_and_weakness(a.as_str())
    } else {
        (vec![], vec![])
    };

    Group {
        id: i,
        units: parse_usize(caps.get(1)),
        hp: parse_usize(caps.get(2)),
        attack: parse_usize(caps.get(4)),
        attack_type: caps.get(5).unwrap().as_str().to_string(),
        immunity: immunity,
        weakness: weakness,
        initiative: parse_usize(caps.get(6)),
    }
}

fn get_immunity_and_weakness(s: &str) -> (Vec<String>, Vec<String>) {
    if s.contains(";") {
        let (immune_to, weak_to) = s.split_once("; ").unwrap();
        let immunity: Vec<String> = immune_to[11..immune_to.len()]
            .split(", ")
            .map(|i| i.to_string())
            .collect();
        let weakness: Vec<String> = weak_to[8..weak_to.len() - 2]
            .split(", ")
            .map(|i| i.to_string())
            .collect();
        return (immunity, weakness);
    } else if s.starts_with("(weak to") {
        let weakness: Vec<String> = s[9..s.len() - 2].split(", ").map(|i| i.to_string()).collect();
        return (vec![], weakness);
    } else if s.starts_with("(immune to") {
        let immunity: Vec<String> = s[11..s.len() - 2].split(", ").map(|i| i.to_string()).collect();
        return (immunity, vec![]);
    } else {
        panic!("don't know how to parse: {}", s)
    }
}

#[cfg(test)]
mod day24_tests {
    use std::fs;

    use crate::y2018::day24::{get_immunity_and_weakness, part_a, part_b};

    #[test]
    fn parsing_works() {
        assert_eq!((vec!(), vec! {"fire".to_string()}), get_immunity_and_weakness("(weak to fire) "));
        assert_eq!(
            (vec!(), vec! {"cold".to_string(), "slashing".to_string()}),
            get_immunity_and_weakness("(weak to cold, slashing) ")
        );
        assert_eq!(
            (vec! {"fire".to_string(), "cold".to_string()}, vec! {"bludgeoning".to_string()}),
            get_immunity_and_weakness("(immune to fire, cold; weak to bludgeoning) ")
        );
    }

    #[test]
    fn test_works() {
        assert_eq!(5216, part_a(fs::read_to_string("input/2018/day24/test.txt").unwrap()));
        assert_eq!(51, part_b(fs::read_to_string("input/2018/day24/test.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(29865, part_a(fs::read_to_string("input/2018/day24/input.txt").unwrap()));
        assert_eq!(2444, part_b(fs::read_to_string("input/2018/day24/input.txt").unwrap()));
    }
}
