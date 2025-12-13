use std::collections::HashMap;
use std::fs;

pub(crate) fn day08() {
    let input = fs::read_to_string("input/2018/day08/input.txt").unwrap();
    let tree: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut license_number = vec![];

    get_license_number(&tree, &mut 0, &mut license_number);
    println!("{}", license_number.iter().sum::<usize>());

    let root_value = get_license_number_with_extra_steps(&tree, &mut 0);
    println!("{}", root_value);
}

fn get_license_number(tree: &Vec<usize>, i: &mut usize, license_number: &mut Vec<usize>) {
    let child_cnt: usize = tree[*i];
    let metadata_cnt: usize = tree[*i + 1];
    *i += 2;

    for _ in 0..child_cnt {
        get_license_number(tree, i, license_number);
    }

    for _ in 0..metadata_cnt {
        license_number.push(tree[*i]);
        *i += 1;
    }
}

fn get_license_number_with_extra_steps(tree: &Vec<usize>, i: &mut usize) -> usize {
    let child_cnt: usize = tree[*i];
    let metadata_cnt: usize = tree[*i + 1];
    *i += 2;

    let mut child_map = HashMap::new();
    for child_id in 1..=child_cnt {
        let child_value = get_license_number_with_extra_steps(tree, i);
        child_map.insert(child_id, child_value);
    }

    let mut sum = 0;
    if child_cnt == 0 {
        for _ in 0..metadata_cnt {
            sum += tree[*i];
            *i += 1;
        }
    } else {
        for _ in 0..metadata_cnt {
            let child_id = tree[*i];
            sum += *child_map.entry(child_id).or_insert(0);
            *i += 1;
        }
    }
    sum
}
