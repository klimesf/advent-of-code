use std::collections::HashMap;

pub(crate) fn day11() {
    part_a();
    part_b();
}

fn part_a() {
    let mut monkeys = load_input();
    let mut buffer: HashMap<usize, Vec<i64>> = HashMap::new();
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            if buffer.contains_key(&i) {
                let items = buffer.get(&i).unwrap();
                for item in items {
                    monkey.items.push(*item);
                }
            }
            buffer.insert(i, vec![]);

            while !monkey.items.is_empty() {
                monkey.counter += 1;
                let mut item = monkey.items.pop().unwrap();
                item = (monkey.operation)(item) / 3;
                if item % monkey.test == 0 {
                    buffer.entry(monkey.if_true).or_insert(vec![]).push(item);
                } else {
                    buffer.entry(monkey.if_false).or_insert(vec![]).push(item);
                }
            }
        }
    }
    get_result(&monkeys)
}

fn part_b() {
    let mut monkeys = load_input();
    let mut buffer: HashMap<usize, Vec<i64>> = HashMap::new();
    let mut lcm: i64 = 1;
    for monkey in &monkeys {
        lcm *= monkey.test;
    }

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            if buffer.contains_key(&i) {
                let items = buffer.get(&i).unwrap();
                for item in items {
                    monkey.items.push(*item);
                }
            }
            buffer.insert(i, vec![]);

            while !monkey.items.is_empty() {
                monkey.counter += 1;
                let mut item = monkey.items.pop().unwrap();
                item = (monkey.operation)(item) % lcm;
                if item % monkey.test == 0 {
                    buffer.entry(monkey.if_true).or_insert(vec![]).push(item);
                } else {
                    buffer.entry(monkey.if_false).or_insert(vec![]).push(item);
                }
            }
        }
    }
    get_result(&monkeys)
}

fn get_result(monkeys: &Vec<Monkey>) {
    let mut max1 = 0;
    let mut max2 = 0;
    for monkey in monkeys {
        if monkey.counter > max1 {
            max2 = max1;
            max1 = monkey.counter;
        } else if monkey.counter > max2 {
            max2 = monkey.counter;
        }
    }
    println!("{} * {} = {}", max1, max2, max1 * max2)
}

#[allow(dead_code)]
fn load_test() -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = vec![];
    monkeys.push(Monkey {
        items: vec![79, 98].into_iter().rev().collect(),
        operation: Box::new(|i| i * 19),
        test: 23,
        if_true: 2,
        if_false: 3,
        counter: 0,
    });
    monkeys.push(Monkey {
        items: vec![54, 65, 75, 74].into_iter().rev().collect(),
        operation: Box::new(|old| old + 6),
        test: 19,
        if_true: 2,
        if_false: 0,
        counter: 0,
    });
    monkeys.push(Monkey {
        items: vec![79, 60, 97].into_iter().rev().collect(),
        operation: Box::new(|old| old * old),
        test: 13,
        if_true: 1,
        if_false: 3,
        counter: 0,
    });
    monkeys.push(Monkey {
        items: vec![74].into_iter().rev().collect(),
        operation: Box::new(|old| old + 3),
        test: 17,
        if_true: 0,
        if_false: 1,
        counter: 0,
    });
    monkeys
}

fn load_input() -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = vec![];
    monkeys.push(Monkey {
        items: vec![93, 98].into_iter().rev().collect(),
        operation: Box::new(|old| old * 17),
        test: 19,
        if_true: 5,
        if_false: 3,
        counter: 0,
    });
    monkeys.push(Monkey {
        items: vec![95, 72, 98, 82, 86].into_iter().rev().collect(),
        operation: Box::new(|old| old + 5),
        test: 13,
        if_true: 7,
        if_false: 6,
        counter: 0,
    });
    monkeys.push(Monkey {
        items: vec![85, 62, 82, 86, 70, 65, 83, 76].into_iter().rev().collect(),
        operation: Box::new(|old| old + 8),
        test: 5,
        if_true: 3,
        if_false: 0,
        counter: 0,
    });
    monkeys.push(Monkey {
        items: vec![86, 70, 71, 56].into_iter().rev().collect(),
        operation: Box::new(|old| old + 1),
        test: 7,
        if_true: 4,
        if_false: 5,
        counter: 0,
    });
    monkeys.push(Monkey {
        items: vec![77, 71, 86, 52, 81, 67].into_iter().rev().collect(),
        operation: Box::new(|old| old + 4),
        test: 17,
        if_true: 1,
        if_false: 6,
        counter: 0,
    });
    monkeys.push(Monkey {
        items: vec![89, 87, 60, 78, 54, 77, 98].into_iter().rev().collect(),
        operation: Box::new(|old| old * 7),
        test: 2,
        if_true: 1,
        if_false: 4,
        counter: 0,
    });
    monkeys.push(Monkey {
        items: vec![69, 65, 63].into_iter().rev().collect(),
        operation: Box::new(|old| old + 6),
        test: 3,
        if_true: 7,
        if_false: 2,
        counter: 0,
    });
    monkeys.push(Monkey {
        items: vec![89].into_iter().rev().collect(),
        operation: Box::new(|old| old * old),
        test: 11,
        if_true: 0,
        if_false: 2,
        counter: 0,
    });
    monkeys
}

struct Monkey {
    items: Vec<i64>,
    operation: Box<dyn Fn(i64) -> i64>,
    test: i64,
    if_true: usize,
    if_false: usize,
    counter: usize,
}
