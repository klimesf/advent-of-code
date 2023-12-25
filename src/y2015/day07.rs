use std::collections::{HashMap, VecDeque};
use std::fs;

pub(crate) fn day07() {
    println!("{}", part_a(fs::read_to_string("input/2015/day07/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2015/day07/input.txt").unwrap()));
}

fn part_a(input: String) -> u16 {
    let mut wires: HashMap<String, u16> = HashMap::new();
    let mut instructions = VecDeque::new();
    input.lines().for_each(|line| instructions.push_back(line.to_string()));

    wire_it_up(&mut wires, &mut instructions)
}

fn part_b(input: String) -> u16 {
    let mut wires: HashMap<String, u16> = HashMap::new();
    let mut instructions = VecDeque::new();
    input.lines()
        .filter(|line| !line.ends_with(" -> b"))
        .for_each(|line| instructions.push_back(line.to_string()));
    wires.insert("b".to_string(), 16076);
    wire_it_up(&mut wires, &mut instructions)
}

fn wire_it_up(wires: &mut HashMap<String, u16>, instructions: &mut VecDeque<String>) -> u16 {
    while let Some(line) = instructions.pop_front() {
        let (instr, dest) = line.split_once(" -> ").unwrap();

        if instr.contains("AND") {
            let (l, r) = instr.split_once(" AND ").unwrap();
            if let Some((lv, rv)) = get_values(wires, l, r) {
                wires.insert(dest.to_string(), lv & rv);
            } else {
                instructions.push_back(line);
            }
        } else if instr.contains("OR") {
            let (l, r) = instr.split_once(" OR ").unwrap();
            if let Some((lv, rv)) = get_values(wires, l, r) {
                wires.insert(dest.to_string(), lv | rv);
            } else {
                instructions.push_back(line);
            }
        } else if instr.contains("RSHIFT") {
            let (l, r) = instr.split_once(" RSHIFT ").unwrap();
            if let Some((lv, rv)) = get_values(wires, l, r) {
                wires.insert(dest.to_string(), lv >> rv);
            } else {
                instructions.push_back(line);
            }
        } else if instr.contains("LSHIFT") {
            let (l, r) = instr.split_once(" LSHIFT ").unwrap();
            if let Some((lv, rv)) = get_values(wires, l, r) {
                wires.insert(dest.to_string(), lv << rv);
            } else {
                instructions.push_back(line);
            }
        } else if instr.contains("NOT") {
            if let Some(v) = get_value(wires, instr[4..].to_string().as_str()) {
                wires.insert(dest.to_string(), !v);
            } else {
                instructions.push_back(line);
            }
        } else {
            if let Some(v) = get_value(wires, instr) {
                wires.insert(dest.to_string(), v);
            } else {
                instructions.push_back(line);
            }
        }
    }

    *wires.get("a").unwrap_or(&0)
}

fn get_values(wires: &HashMap<String, u16>, l: &str, r: &str) -> Option<(u16, u16)> {
    if let Some(lv) = get_value(wires, l) {
        if let Some(rv) = get_value(wires, r) {
            return Some((lv, rv));
        }
    }
    None
}

fn get_value(wires: &HashMap<String, u16>, s: &str) -> Option<u16> {
    if let Ok(ln) = s.parse::<u16>() {
        Some(ln)
    } else {
        wires.get(s).map(|n| *n)
    }
}

#[cfg(test)]
mod day07_tests {
    use std::fs;

    use crate::y2015::day07::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(0, part_a(fs::read_to_string("input/2015/day07/test.txt").unwrap()));
        assert_eq!(0, part_b(fs::read_to_string("input/2015/day07/test.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(16076, part_a(fs::read_to_string("input/2015/day07/input.txt").unwrap()));
        assert_eq!(2797, part_b(fs::read_to_string("input/2015/day07/input.txt").unwrap()));
    }
}
