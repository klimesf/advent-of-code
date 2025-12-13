use std::collections::HashMap;
use std::fs;

use regex::Regex;

pub(crate) fn day04() {
    let input = fs::read_to_string("input/2018/day04/input.txt").unwrap();
    let mut raw_log: Vec<&str> = input.trim().split("\n").collect();
    raw_log.sort();

    let sleep_log = build_sleep_log(&mut raw_log);

    let most_sleeping = sleep_log
        .iter()
        .max_by(|g, h| sum_minutes_slept(g.1).cmp(&sum_minutes_slept(h.1)))
        .unwrap();
    println!(
        "The most sleeping guard is #{} with {} minutes slept",
        most_sleeping.0,
        sum_minutes_slept(most_sleeping.1)
    );

    let best_minute = find_best_minute(most_sleeping);
    println!(
        "The best minute is {} with {} days asleep",
        best_minute.0, best_minute.1
    );

    println!(
        "{} x {} = {}",
        most_sleeping.0,
        best_minute.0,
        most_sleeping.0 * best_minute.0
    );

    let resident_sleeper = sleep_log
        .iter()
        .map(|log| (log.0, find_best_minute(log)))
        .max_by_key(|(_, best_minute)| best_minute.1)
        .unwrap();
    println!(
        "Guard #{} spent the minute {} asleep more than any other guard or minute",
        resident_sleeper.0, resident_sleeper.1 .0
    );

    println!(
        "{} x {} = {}",
        resident_sleeper.0,
        resident_sleeper.1 .0,
        resident_sleeper.0 * resident_sleeper.1 .0
    );
}

fn build_sleep_log(raw_log: &mut Vec<&str>) -> HashMap<usize, Vec<[bool; 60]>> {
    let re_1 = Regex::new(r"^\[\d+-\d+-\d+\s\d+:(\d+)]\s(.+)$").unwrap();
    let re_2 = Regex::new(r"^Guard #(\d+) begins shift$").unwrap();
    let mut sleep_log: HashMap<usize, Vec<[bool; 60]>> = HashMap::new();

    let mut current_guard = 0;
    let mut current_log = [false; 60];
    let mut sleeping_start = 0;
    for log in raw_log {
        let captures = re_1.captures(log).unwrap();
        let minute: usize = captures.get(1).unwrap().as_str().parse().unwrap();
        let action = captures.get(2).unwrap().as_str();

        match action {
            "falls asleep" => {
                sleeping_start = minute;
            }
            "wakes up" => {
                for m in sleeping_start..minute {
                    current_log[m] = true;
                }
            }
            g => {
                if current_guard != 0 {
                    let log = sleep_log.entry(current_guard).or_insert(vec![]);
                    log.push(current_log);
                    current_log = [false; 60];
                }
                current_guard = re_2
                    .captures(g)
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str()
                    .parse()
                    .unwrap();
            }
        }
    }
    sleep_log
}

fn sum_minutes_slept(logs: &Vec<[bool; 60]>) -> usize {
    logs.iter()
        .map(|l| l.iter().map(|m| if *m { 1 } else { 0 }).sum::<usize>())
        .sum::<usize>()
}

fn find_best_minute(log: (&usize, &Vec<[bool; 60]>)) -> (usize, i32) {
    let mut best_minutes = [0; 60];
    log.1.iter().for_each(|log| {
        (0..60).for_each(|m| {
            if log[m] {
                best_minutes[m] += 1
            }
        })
    });
    let best_minute = (0..60)
        .map(|m| (m, best_minutes[m]))
        .max_by_key(|(_, total)| *total)
        .unwrap();
    best_minute
}
