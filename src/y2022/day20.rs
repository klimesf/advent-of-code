use std::fs;

pub(crate) fn day20() {
    let input = fs::read_to_string("input/2022/day20/input.txt").unwrap();
    let nums: Vec<i32> = input.lines().into_iter().map(|line| line.parse::<i32>().unwrap()).collect();

    part_a(nums.clone());
    part_b(nums.clone());
}

fn part_a(nums: Vec<i32>) {
    let mut positions: Vec<usize> = (0..nums.len()).collect();

    for i in 0..nums.len() {
        let num = nums[i];
        let pos = positions.iter().position(|x| *x == i).unwrap();
        positions.remove(pos);
        let new_pos = (pos as i32 + num).rem_euclid(positions.len() as i32) as usize;
        positions.insert(new_pos, i);
    }

    let zero_start = nums.iter().position(|x| *x == 0).unwrap();
    let zero_pos = positions.iter().position(|p| *p == zero_start).unwrap();
    let ans: i32 = vec!(1000, 2000, 3000).iter().map(|pos| nums[positions[(zero_pos + pos) % positions.len()]]).sum();
    println!("{}", ans);
}

fn part_b(input_nums: Vec<i32>) {
    let nums: Vec<i64> = input_nums.iter().map(|num| *num as i64 * 811589153).collect();
    let mut positions: Vec<usize> = (0..nums.len()).collect();

    for _ in 0..10 {
        for i in 0..nums.len() {
            let num = nums[i];
            let pos = positions.iter().position(|x| *x == i).unwrap();
            positions.remove(pos);
            let new_pos = (pos as i64 + num).rem_euclid(positions.len() as i64) as usize;
            positions.insert(new_pos, i);
        }
    }

    let zero_start = nums.iter().position(|x| *x == 0).unwrap();
    let zero_pos = positions.iter().position(|p| *p == zero_start).unwrap();
    let ans: i64 = vec!(1000, 2000, 3000).iter().map(|pos| nums[positions[(zero_pos + pos) % positions.len()]]).sum();
    println!("{}", ans);
}
