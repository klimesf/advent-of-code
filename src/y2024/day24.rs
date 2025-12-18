use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::fs;

pub fn day24(print_usize: fn(usize), print_string: fn(String)) {
    print_usize(part_a(fs::read_to_string("input/2024/day24/input.txt").unwrap()));
    print_string(part_b(fs::read_to_string("input/2024/day24/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    let (svars, sinstr) = input.split_once("\n\n").unwrap();
    let mut variables: HashMap<&str, usize> = HashMap::new();

    svars.lines().for_each(|line| {
        let (name, value) = line.split_once(": ").unwrap();
        variables.insert(name, value.parse().unwrap());
    });

    let mut instructions = VecDeque::new();
    sinstr.lines().for_each(|line| {
        let (op, to) = line.split_once(" -> ").unwrap();
        let op_parts = op.split_whitespace().collect::<Vec<&str>>();

        let left = op_parts.get(0).unwrap();
        let middle = op_parts.get(1).unwrap();
        let right = op_parts.get(2).unwrap();

        instructions.push_back((*left, *middle, *right, to));
    });

    eval_wires(variables.clone(), instructions.clone())
}

fn part_b(input: String) -> String {
    let (svars, sinstr) = input.split_once("\n\n").unwrap();
    let mut variables: HashMap<&str, usize> = HashMap::new();

    svars.lines().for_each(|line| {
        let (name, value) = line.split_once(": ").unwrap();
        variables.insert(name, value.parse().unwrap());
    });

    let mut instructions = vec![];
    sinstr.lines().for_each(|line| {
        let (op, to) = line.split_once(" -> ").unwrap();
        let op_parts = op.split_whitespace().collect::<Vec<&str>>();

        let left = op_parts.get(0).unwrap();
        let middle = op_parts.get(1).unwrap();
        let right = op_parts.get(2).unwrap();

        instructions.push((*left, *middle, *right, to));
    });

    let x_str = variables
        .iter()
        .filter(|(k, _)| k.starts_with("x"))
        .sorted_by(|(k1, _), (k2, _)| k1.cmp(k2))
        .map(|(_, v)| v.to_string())
        .rev()
        .join("");
    let _x = usize::from_str_radix(x_str.as_str(), 2).unwrap();

    let y_str = variables
        .iter()
        .filter(|(k, _)| k.starts_with("y"))
        .sorted_by(|(k1, _), (k2, _)| k1.cmp(k2))
        .map(|(_, v)| v.to_string())
        .rev()
        .join("");
    let _y = usize::from_str_radix(y_str.as_str(), 2).unwrap();

    // 124154011875465 * 40320, wtf
    // for comb in (0..instructions.len()).combinations(8) {
    //     for _pair in comb.iter().permutations(8) {
    //     }
    // }

    // I'm going to solve this manually using graphviz
    let mut out = String::new();
    out.push_str("digraph {\n");
    instructions.iter().for_each(|(left, middle, right, to)| {
        out.push_str(format!("    {} -> {} [label=\"{}\"];\n", left, to, middle).as_str());
        out.push_str(format!("    {} -> {} [label=\"{}\"];\n", right, to, middle).as_str());
    });
    out.push_str("}\n");

    // fs::write("input/2024/day24/input.dot", out).unwrap();
    // dot -Tsvg -o output.svg input/2024/day24/input.dot

    // The adder schema:
    // x__2 AND y__2 -> ovf2
    // x__2 XOR y__2 -> buf2
    // buf2 XOR ovf1 -> z__2
    // buf2 AND ovf1 -> xx12
    // ovf2  OR xx12 -> ????

    // What I found in my input:
    // z07 swap jbm
    // z13 swap hsw
    // z18 swap skf
    // nvr swap wkr

    for pair in [["z07", "bjm"], ["z13", "hsw"], ["z18", "skf"], ["nvr", "wkr"]] {
        let mut swap_from = 0;
        let mut swap_from_instr = ("", "", "", "");
        for i in 0..instructions.len() {
            let (left, op, middle, to) = instructions[i];
            if to == pair[0] {
                swap_from = i;
                swap_from_instr = (left, op, middle, pair[1]);
                break;
            }
        }

        let mut swap_to = 0;
        let mut swap_to_instr = ("", "", "", "");
        for i in 0..instructions.len() {
            let (left, op, middle, to) = instructions[i];
            if to == pair[1] {
                swap_to = i;
                swap_to_instr = (left, op, middle, pair[0]);
                break;
            }
        }

        instructions[swap_from] = swap_from_instr;
        instructions[swap_to] = swap_to_instr;
    }

    // You can eval here

    ["z07", "bjm", "hsw", "skf", "z13", "z18", "nvr", "wkr"]
        .iter()
        .sorted()
        .join(",")
}

fn eval_wires<'a>(
    mut variables: HashMap<&'a str, usize>,
    mut instructions: VecDeque<(&str, &str, &str, &'a str)>,
) -> usize {
    while let Some((left, op, right, to)) = instructions.pop_front() {
        if !variables.contains_key(left) || !variables.contains_key(right) {
            instructions.push_back((left, op, right, to));
            continue;
        }

        match op {
            "AND" => {
                if variables[left] == 1 && variables[right] == 1 {
                    variables.insert(to, 1);
                } else {
                    variables.insert(to, 0);
                }
            }
            "OR" => {
                if variables[left] == 1 || variables[right] == 1 {
                    variables.insert(to, 1);
                } else {
                    variables.insert(to, 0);
                }
            }
            "XOR" => {
                if variables[left] != variables[right] {
                    variables.insert(to, 1);
                } else {
                    variables.insert(to, 0);
                }
            }
            _ => panic!(),
        }
    }

    let bin = variables
        .iter()
        .filter(|(k, _)| k.starts_with("z"))
        .sorted_by(|(k1, _), (k2, _)| k1.cmp(k2))
        .map(|(_, v)| v.to_string())
        .rev()
        .join("");

    usize::from_str_radix(bin.as_str(), 2).unwrap()
}

#[cfg(test)]
mod day24_tests {
    use std::fs;

    use crate::y2024::day24::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(4, part_a(fs::read_to_string("input/2024/day24/test.txt").unwrap()));
        assert_eq!(2024, part_a(fs::read_to_string("input/2024/day24/test_2.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(56729630917616, part_a(fs::read_to_string("input/2024/day24/input.txt").unwrap()));
        assert_eq!(
            "bjm,hsw,nvr,skf,wkr,z07,z13,z18",
            part_b(fs::read_to_string("input/2024/day24/input.txt").unwrap())
        );
    }
}
