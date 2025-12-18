use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

pub(crate) fn day11() {
    let floors_a = vec![
        (vec!["thulium"], vec!["thulium", "plutonium", "strontium"]),
        (vec!["plutonium", "strontium"], vec![]),
        (vec!["promethium", "ruthenium"], vec!["promethium", "ruthenium"]),
        (vec![], vec![]),
    ];

    println!("{}", solve(floors_a));

    let floors_b = vec![
        (
            vec!["thulium", "elerium", "dilithium"],
            vec!["thulium", "plutonium", "strontium", "elerium", "dilithium"],
        ),
        (vec!["plutonium", "strontium"], vec![]),
        (vec!["promethium", "ruthenium"], vec!["promethium", "ruthenium"]),
        (vec![], vec![]),
    ];
    println!("{}", solve(floors_b));
}

fn solve(input_floors: Vec<(Vec<&str>, Vec<&str>)>) -> usize {
    let mut stack: BinaryHeap<State> = BinaryHeap::new();
    stack.push(State {
        floors: input_floors.clone(),
        elevator: 0,
        steps: 0,
        a_star: eyeball_a_star(0, &input_floors),
    });

    let mut visited = HashSet::new();
    while let Some(state) = stack.pop() {
        if is_invalid(&state.floors) {
            continue;
        }
        if !visited.insert(hash(&state.floors, state.elevator)) {
            continue;
        }

        if is_done(&state.floors) {
            return state.steps;
        }

        let mut next_elevators = vec![];
        if state.elevator < 3 {
            next_elevators.push(state.elevator + 1)
        }
        if state.elevator > 0 {
            next_elevators.push(state.elevator - 1)
        }

        for next_elevator in next_elevators {
            let chips = &state.floors[state.elevator].0;
            let generators = &state.floors[state.elevator].1;
            for i in 0..chips.len() {
                // Go up with just chip
                let element = chips[i];
                let mut new_floors = state.floors.clone();
                new_floors[state.elevator].0 = new_floors[state.elevator]
                    .0
                    .iter()
                    .filter(|s| **s != element)
                    .map(|s| *s)
                    .collect();
                new_floors[next_elevator].0.push(element);
                if !is_invalid(&new_floors) {
                    let dist = eyeball_a_star(state.steps + 1, &new_floors);
                    stack.push(State {
                        floors: new_floors,
                        elevator: next_elevator,
                        steps: state.steps + 1,
                        a_star: dist,
                    });
                }

                // Go up with chip & generator if possible
                if generators.contains(&element) {
                    let mut new_floors = state.floors.clone();
                    new_floors[state.elevator].0 = new_floors[state.elevator]
                        .0
                        .iter()
                        .filter(|s| **s != element)
                        .map(|s| *s)
                        .collect();
                    new_floors[next_elevator].0.push(element);
                    new_floors[state.elevator].1 = new_floors[state.elevator]
                        .1
                        .iter()
                        .filter(|s| **s != element)
                        .map(|s| *s)
                        .collect();
                    new_floors[next_elevator].1.push(element);
                    if !is_invalid(&new_floors) {
                        let dist = eyeball_a_star(state.steps + 1, &new_floors);
                        stack.push(State {
                            floors: new_floors,
                            elevator: next_elevator,
                            steps: state.steps + 1,
                            a_star: dist,
                        });
                    }
                }

                // Go up with 2 chips
                for j in 0..chips.len() {
                    if i == j {
                        continue;
                    }
                    let element_2 = chips[j];
                    let mut new_floors = state.floors.clone();
                    new_floors[state.elevator].0 = new_floors[state.elevator]
                        .0
                        .iter()
                        .filter(|s| **s != element && **s != element_2)
                        .map(|s| *s)
                        .collect();
                    new_floors[next_elevator].0.push(element);
                    new_floors[next_elevator].0.push(element_2);
                    if !is_invalid(&new_floors) {
                        let dist = eyeball_a_star(state.steps + 1, &new_floors);
                        stack.push(State {
                            floors: new_floors,
                            elevator: next_elevator,
                            steps: state.steps + 1,
                            a_star: dist,
                        });
                    }
                }
            }

            for i in 0..generators.len() {
                // Go up with 1 generator
                let element = generators[i];
                let mut new_floors = state.floors.clone();
                new_floors[state.elevator].1 = new_floors[state.elevator]
                    .1
                    .iter()
                    .filter(|s| **s != element)
                    .map(|s| *s)
                    .collect();
                new_floors[next_elevator].1.push(element);
                if !is_invalid(&new_floors) {
                    let dist = eyeball_a_star(state.steps + 1, &new_floors);
                    stack.push(State {
                        floors: new_floors,
                        elevator: next_elevator,
                        steps: state.steps + 1,
                        a_star: dist,
                    });
                }

                // Go up with 2 generators
                for j in 0..generators.len() {
                    if i == j {
                        continue;
                    }
                    let gen_2 = generators[j];
                    let mut new_floors = state.floors.clone();
                    new_floors[state.elevator].1 = new_floors[state.elevator]
                        .1
                        .iter()
                        .filter(|s| **s != element && **s != gen_2)
                        .map(|s| *s)
                        .collect();
                    new_floors[next_elevator].1.push(element);
                    new_floors[next_elevator].1.push(gen_2);
                    if !is_invalid(&new_floors) {
                        let dist = eyeball_a_star(state.steps + 1, &new_floors);
                        stack.push(State {
                            floors: new_floors,
                            elevator: next_elevator,
                            steps: state.steps + 1,
                            a_star: dist,
                        });
                    }
                }
            }
        }
    }

    panic!("never reached end state")
}

fn eyeball_a_star(steps: usize, floors: &Vec<(Vec<&str>, Vec<&str>)>) -> usize {
    let mut ans = 0;
    for i in 0..=2 {
        ans += (3 - i) * 3 * (floors[i].0.iter().count() + floors[i].1.iter().count())
    }
    steps + ans
}

#[derive(Clone, Eq, PartialEq)]
struct State<'a> {
    floors: Vec<(Vec<&'a str>, Vec<&'a str>)>,
    elevator: usize,
    steps: usize,
    a_star: usize,
}

impl Ord for State<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.a_star.cmp(&self.a_star)
    }
}

impl PartialOrd for State<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn is_done(floors: &Vec<(Vec<&str>, Vec<&str>)>) -> bool {
    floors[0].0.is_empty()
        && floors[0].1.is_empty()
        && floors[1].0.is_empty()
        && floors[1].1.is_empty()
        && floors[2].0.is_empty()
        && floors[2].1.is_empty()
}

fn is_invalid(floors: &Vec<(Vec<&str>, Vec<&str>)>) -> bool {
    for i in 0..floors.len() {
        if floors[i]
            .0
            .iter()
            .any(|s| !floors[i].1.contains(s) && floors[i].1.len() > 0)
        {
            // println!("invalid");
            return true;
        }
    }
    false
}

fn hash(floors: &Vec<(Vec<&str>, Vec<&str>)>, elevator: usize) -> String {
    let mut floors_hash = String::new();
    for i in 0..floors.len() {
        let mut chips: Vec<&str> = floors[i].0.clone();
        chips.sort();
        floors_hash.push_str(chips.join(",").as_str());
        floors_hash.push_str("_");
        floors_hash.push_str(floors[i].1.join(",").as_str());
        floors_hash.push_str(" | ");
    }
    format!("({}): {}", elevator, floors_hash)
}

#[cfg(test)]
mod day11_tests {

    use crate::y2016::day11::solve;

    #[test]
    fn test_works() {
        let floors = vec![
            (vec!["hydrogen", "lithium"], vec![]),
            (vec![], vec!["hydrogen"]),
            (vec![], vec!["lithium"]),
            (vec![], vec![]),
        ];

        assert_eq!(11, solve(floors));
    }

    #[test]
    fn input_works() {
        let floors_a = vec![
            (vec!["thulium"], vec!["thulium", "plutonium", "strontium"]),
            (vec!["plutonium", "strontium"], vec![]),
            (vec!["promethium", "ruthenium"], vec!["promethium", "ruthenium"]),
            (vec![], vec![]),
        ];

        assert_eq!(31, solve(floors_a));

        let floors_b = vec![
            (
                vec!["thulium", "elerium", "dilithium"],
                vec!["thulium", "plutonium", "strontium", "elerium", "dilithium"],
            ),
            (vec!["plutonium", "strontium"], vec![]),
            (vec!["promethium", "ruthenium"], vec!["promethium", "ruthenium"]),
            (vec![], vec![]),
        ];
        assert_eq!(55, solve(floors_b));
    }
}
