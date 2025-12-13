use rayon::iter::ParallelIterator;
use rayon::iter::{IntoParallelRefIterator, ParallelBridge};
use regex::{Match, Regex};
use std::fs;

pub(crate) fn day19() {
    let input = fs::read_to_string("input/2022/day19/input.txt").unwrap();
    let re = Regex::new(r"^Blueprint ([0-9]+): Each ore robot costs ([0-9]+) ore. Each clay robot costs ([0-9]+) ore. Each obsidian robot costs ([0-9]+) ore and ([0-9]+) clay. Each geode robot costs ([0-9]+) ore and ([0-9]+) obsidian.$").unwrap();
    let mut blueprints = vec![];
    for line in input.lines().into_iter() {
        let g = re.captures(line).unwrap();
        let costs = (
            parse_i32(g.get(1)),
            (parse_i32(g.get(2)), 0, 0),
            (parse_i32(g.get(3)), 0, 0),
            (parse_i32(g.get(4)), parse_i32(g.get(5)), 0),
            (parse_i32(g.get(6)), 0, parse_i32(g.get(7))),
        );
        blueprints.push(costs);
    }

    let ans = blueprints
        .par_iter()
        .map(|(id, ore_cost, clay_cost, obsidian_cost, geode_cost)| {
            solve(24, *ore_cost, *clay_cost, *obsidian_cost, *geode_cost) * id
        })
        .sum::<i32>();
    println!("{}", ans);

    let ans = blueprints
        .iter()
        .take(3)
        .par_bridge()
        .map(|(_, ore_cost, clay_cost, obsidian_cost, geode_cost)| {
            solve(32, *ore_cost, *clay_cost, *obsidian_cost, *geode_cost)
        })
        .product::<i32>();
    println!("{}", ans);
}

fn parse_i32(g: Option<Match>) -> i32 {
    return g.map_or(0, |m| m.as_str().parse().unwrap());
}

type Cost = (i32, i32, i32);

fn solve(time: i32, ore_cost: Cost, clay_cost: Cost, obsidian_cost: Cost, geode_cost: Cost) -> i32 {
    let robot_ctr = [1, 0, 0, 0];
    let resource_ctr = [0; 4];
    let max_ore_cost = vec![ore_cost.0, clay_cost.0, obsidian_cost.0, geode_cost.0]
        .into_iter()
        .max()
        .unwrap();
    let max_clay_cost = vec![ore_cost.1, clay_cost.1, obsidian_cost.1, geode_cost.1]
        .into_iter()
        .max()
        .unwrap();
    let mut states = vec![];
    states.push((0, robot_ctr.clone(), resource_ctr.clone()));

    let mut max = 0;
    while !states.is_empty() {
        let (minute, robot_ctr, resource_ctr) = states.pop().unwrap();
        if minute == time {
            if resource_ctr[3] > max {
                max = resource_ctr[3];
            }
            continue;
        }

        let mut possible_max = resource_ctr[3] + (robot_ctr[3] * (time - minute));
        for i in 0..time - minute {
            possible_max += i;
        }
        if possible_max <= max {
            continue; // Prune
        }

        let mut branch_count = 0;

        // Build ore bot next
        if robot_ctr[0] < max_ore_cost {
            let ore_available_in =
                ((ore_cost.0 - resource_ctr[0] + robot_ctr[0] - 1) / robot_ctr[0]).max(0) + 1;
            if minute + ore_available_in <= time {
                branch_count += 1;
                let mut res_clone = resource_ctr.clone();
                res_clone[0] += (robot_ctr[0] * ore_available_in) - ore_cost.0;
                res_clone[1] += (robot_ctr[1] * ore_available_in) - ore_cost.1;
                res_clone[2] += (robot_ctr[2] * ore_available_in) - ore_cost.2;
                res_clone[3] += robot_ctr[3] * ore_available_in;
                let mut robot_ctr_clone = robot_ctr.clone();
                robot_ctr_clone[0] += 1;
                states.push((minute + ore_available_in, robot_ctr_clone, res_clone));
            }
        }

        // Build clay bot next
        if robot_ctr[1] < max_clay_cost {
            let ore_available_in =
                ((clay_cost.0 - resource_ctr[0] + robot_ctr[0] - 1) / robot_ctr[0]).max(0) + 1;
            if minute + ore_available_in <= time {
                branch_count += 1;
                let mut res_clone = resource_ctr.clone();
                res_clone[0] += (robot_ctr[0] * ore_available_in) - clay_cost.0;
                res_clone[1] += (robot_ctr[1] * ore_available_in) - clay_cost.1;
                res_clone[2] += (robot_ctr[2] * ore_available_in) - clay_cost.2;
                res_clone[3] += robot_ctr[3] * ore_available_in;
                let mut robot_ctr_clone = robot_ctr.clone();
                robot_ctr_clone[1] += 1;
                states.push((minute + ore_available_in, robot_ctr_clone, res_clone));
            }
        }

        // Build obsidian bot next
        if robot_ctr[0] > 0 && robot_ctr[1] > 0 {
            let ore_available_in =
                ((obsidian_cost.0 - resource_ctr[0] + robot_ctr[0] - 1) / robot_ctr[0]).max(0) + 1;
            let clay_available_in =
                ((obsidian_cost.1 - resource_ctr[1] + robot_ctr[1] - 1) / robot_ctr[1]).max(0) + 1;
            let robot_available_in = ore_available_in.max(clay_available_in);
            if minute + robot_available_in <= time {
                branch_count += 1;
                let mut res_clone = resource_ctr.clone();
                res_clone[0] += (robot_ctr[0] * robot_available_in) - obsidian_cost.0;
                res_clone[1] += (robot_ctr[1] * robot_available_in) - obsidian_cost.1;
                res_clone[2] += (robot_ctr[2] * robot_available_in) - obsidian_cost.2;
                res_clone[3] += robot_ctr[3] * robot_available_in;
                let mut robot_ctr_clone = robot_ctr.clone();
                robot_ctr_clone[2] += 1;
                states.push((minute + robot_available_in, robot_ctr_clone, res_clone));
            }
        }

        // Build geode bot next
        if robot_ctr[0] > 0 && robot_ctr[2] > 0 {
            let ore_available_in =
                ((geode_cost.0 - resource_ctr[0] + robot_ctr[0] - 1) / robot_ctr[0]).max(0) + 1;
            let obsidian_available_in =
                ((geode_cost.2 - resource_ctr[2] + robot_ctr[2] - 1) / robot_ctr[2]).max(0) + 1;
            let robot_available_in = ore_available_in.max(obsidian_available_in);
            if minute + robot_available_in <= time {
                branch_count += 1;
                let mut res_clone = resource_ctr.clone();
                res_clone[0] += (robot_ctr[0] * robot_available_in) - geode_cost.0;
                res_clone[1] += (robot_ctr[1] * robot_available_in) - geode_cost.1;
                res_clone[2] += (robot_ctr[2] * robot_available_in) - geode_cost.2;
                res_clone[3] += robot_ctr[3] * robot_available_in;
                let mut robot_ctr_clone = robot_ctr.clone();
                robot_ctr_clone[3] += 1;
                states.push((minute + robot_available_in, robot_ctr_clone, res_clone));
            }
        }

        if branch_count == 0 {
            // No time to build other robots, simulate to minute 30
            let rem = time - minute - 1;
            let mut res_clone = resource_ctr.clone();
            res_clone[0] += resource_ctr[0] * rem;
            res_clone[1] += resource_ctr[1] * rem;
            res_clone[2] += resource_ctr[2] * rem;
            res_clone[3] += resource_ctr[3] * rem;
            states.push((time, robot_ctr.clone(), res_clone));
        }
    }
    return max;
}
