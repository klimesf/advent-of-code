use std::collections::{HashMap, HashSet};
use std::fs;

pub(crate) fn day13() {
    let input = fs::read_to_string("input/2018/day13/input.txt").unwrap();
    let mut map = HashMap::new();
    let mut carts = vec!();
    input.split("\n").enumerate()
        .for_each(|(y, row)| {
            row.chars().enumerate()
                .for_each(|(x, c)| {
                    match c {
                        '-' | '|' | '/' | '\\' | '+' => { map.insert((x, y), c); }
                        'v' => {
                            map.insert((x, y), '|');
                            carts.push((x, y, c, 0));
                        }
                        '^' => {
                            map.insert((x, y), '|');
                            carts.push((x, y, c, 0));
                        }
                        '>' => {
                            map.insert((x, y), '-');
                            carts.push((x, y, c, 0));
                        }
                        '<' => {
                            map.insert((x, y), '-');
                            carts.push((x, y, c, 0));
                        }
                        _ => {}
                    };
                });
        });

    part_a(&mut map, carts.clone());
    part_b(&mut map, carts.clone());
}

fn part_a(map: &mut HashMap<(usize, usize), char>, mut carts: Vec<(usize, usize, char, usize)>) {
    let first_collision;
    'outer: loop {
        let mut new_carts = vec!();
        let mut positions = HashSet::new();
        for cart in &carts { positions.insert((cart.0, cart.1)); };

        for cart in &carts {
            let new_cart = eval_cart(&cart, &map);
            positions.remove(&(cart.0, cart.1));
            if !positions.insert((new_cart.0, new_cart.1)) {
                first_collision = (new_cart.0, new_cart.1);
                break 'outer;
            }
            new_carts.push(new_cart);
        }

        new_carts.sort_by(|(x1, y1, _, _), (x2, y2, _, _)| {
            let cmp_y = y1.cmp(y2);
            if cmp_y.is_eq() {
                x1.cmp(x2)
            } else {
                cmp_y
            }
        });
        carts = new_carts;
    }
    println!("First collision happens at {},{}", first_collision.0, first_collision.1);
}

fn part_b(map: &mut HashMap<(usize, usize), char>, mut carts: Vec<(usize, usize, char, usize)>) {
    let last_cart;
    loop {
        let mut new_carts = vec!();
        let mut collisions = HashSet::new();
        let mut positions = HashSet::new();
        for cart in &carts { positions.insert((cart.0, cart.1)); };

        if carts.len() == 1 {
            last_cart = carts[0];
            break;
        }

        for cart in &carts {
            if collisions.contains(&(cart.0, cart.1)) { continue; }

            let new_cart = eval_cart(&cart, &map);
            positions.remove(&(cart.0, cart.1));
            if !positions.insert((new_cart.0, new_cart.1)) {
                collisions.insert((new_cart.0, new_cart.1));
                positions.remove(&(new_cart.0, new_cart.1));
            } else {
                new_carts.push(new_cart);
            }
        }
        let mut new_new_carts = vec!();
        for cart in new_carts {
            if collisions.contains(&(cart.0, cart.1)) { continue; }
            new_new_carts.push(cart);
        }

        new_new_carts.sort_by(|(x1, y1, _, _), (x2, y2, _, _)| {
            let cmp_y = y1.cmp(y2);
            if cmp_y.is_eq() {
                x1.cmp(x2)
            } else {
                cmp_y
            }
        });
        carts = new_new_carts;
    }
    println!("Last remaining cart ends up at {},{}", last_cart.0, last_cart.1);
}

fn eval_cart(cart: &(usize, usize, char, usize), map: &HashMap<(usize, usize), char>) -> (usize, usize, char, usize) {
    let new_pos = match cart.2 {
        '>' => (cart.0 + 1, cart.1),
        '<' => (cart.0 - 1, cart.1),
        '^' => (cart.0, cart.1 - 1),
        'v' => (cart.0, cart.1 + 1),
        _ => panic!("Unknown cart direction {}", cart.2),
    };
    let next = match map.get(&new_pos) {
        Some(pos) => *pos,
        None => panic!("{:?} {:?}", cart, new_pos),
    };

    match next {
        '-' | '|' => (new_pos.0, new_pos.1, cart.2, cart.3),
        '\\' => match cart.2 {
            'v' => (new_pos.0, new_pos.1, '>', cart.3),
            '<' => (new_pos.0, new_pos.1, '^', cart.3),
            '>' => (new_pos.0, new_pos.1, 'v', cart.3),
            '^' => (new_pos.0, new_pos.1, '<', cart.3),
            _ => panic!("Cannot approach \\ from direction {}", cart.2),
        },
        '/' => match cart.2 {
            'v' => (new_pos.0, new_pos.1, '<', cart.3),
            '<' => (new_pos.0, new_pos.1, 'v', cart.3),
            '>' => (new_pos.0, new_pos.1, '^', cart.3),
            '^' => (new_pos.0, new_pos.1, '>', cart.3),
            _ => panic!("Cannot approach / from direction {}", cart.2),
        },
        '+' => match cart.3 {
            0 => { // Turns left
                match cart.2 {
                    'v' => (new_pos.0, new_pos.1, '>', 1),
                    '<' => (new_pos.0, new_pos.1, 'v', 1),
                    '>' => (new_pos.0, new_pos.1, '^', 1),
                    '^' => (new_pos.0, new_pos.1, '<', 1),
                    _ => panic!("Unknown direction {}", cart.2),
                }
            }
            1 => (new_pos.0, new_pos.1, cart.2, 2), // Goes straight
            2 => { // Turns right
                match cart.2 {
                    'v' => (new_pos.0, new_pos.1, '<', 0),
                    '<' => (new_pos.0, new_pos.1, '^', 0),
                    '>' => (new_pos.0, new_pos.1, 'v', 0),
                    '^' => (new_pos.0, new_pos.1, '>', 0),
                    _ => panic!("Unknown direction {}", cart.2),
                }
            }
            _ => panic!("Unknown state: {}", cart.3),
        },
        _ => panic!("Unknown piece of road {}", next),
    }
}
