use std::fs;

use regex::Regex;

pub(crate) fn day22() {
    let input = fs::read_to_string("input/2019/day22/input.txt").unwrap();
    let instructions = prepare_instructions(&input);

    // Inspired by https://codeforces.com/blog/entry/72593
    part_a(instructions.clone());
    part_b(instructions.clone());
}

fn part_a(instructions: Vec<(i128, i128)>) {
    let len = 10007;

    let shuffle_fn: (i128, i128) = instructions
        .iter()
        .copied()
        // g(f(x)) = acx + bc + d mod m
        .reduce(|(a, b), (c, d)| ((a * c) % len, (((b * c) % len) + d) % len))
        .unwrap();

    println!("Position of card 2019 is {}", (2019 * shuffle_fn.0 + shuffle_fn.1) % len);
}

fn part_b(instructions: Vec<(i128, i128)>) {
    let m = 119315717514047;
    let k = 101741582076661;

    let shuffle_fn: (i128, i128) = instructions
        .iter()
        .copied()
        // g(f(x)) = acx + bc + d mod m
        .reduce(|(a, b), (c, d)| ((a * c) % m, (((b * c) % m) + d) % m))
        .unwrap();

    // Now we need to apply shuffle function k = 101741582076661 times within modulo m = 119315717514047
    // f^k (x) = ((a^k * x) + (b * (1 - a^k)) / 1 - a)) % m
    let a_to_the_k = pow_mod(shuffle_fn.0, k, m);
    let fn_to_the_k: (i128, i128) = (
        a_to_the_k,
        ((((shuffle_fn.1 % m) * ((1 - a_to_the_k) % m)) % m) * pow_mod(1 - shuffle_fn.0, m - 2, m)) % m,
    );

    // And finally, we need to inverse the function
    // f^-1 (x) = (x - b) / a
    let fn_inv = (((2020 - fn_to_the_k.1) % m) * pow_mod(fn_to_the_k.0, m - 2, m)) % m;

    println!("Card 2020 ends up in position {}", fn_inv);
}

fn prepare_instructions(input: &String) -> Vec<(i128, i128)> {
    let re = Regex::new(r"^(deal into new stack|deal with increment|cut)(\s[\-0-9]+)?$").unwrap();
    input
        .trim()
        .split("\n")
        // Parse each instruction
        .map(|s| {
            let captures = re.captures(s).unwrap();
            let technique = captures.get(1).unwrap().as_str();
            let n = match captures.get(2) {
                Some(v) => v.as_str().trim().parse::<i128>().unwrap(),
                None => 0, // deal into new stack has no parameter, but we don't care, it's not going to be used
            };
            (technique, n)
        })
        // Map each technique into linear congruential function
        .map(|(technique, n)| match technique {
            "deal into new stack" => (-1, -1),
            "cut" => (1, -n),
            "deal with increment" => (n, 0),
            c => panic!("Unknown command: {}", c),
        })
        .collect()
}

// x ^ n mod m
fn pow_mod(x: i128, n: i128, m: i128) -> i128 {
    if n == 0 {
        return 1;
    }
    let t = pow_mod(x, n / 2, m);

    if n % 2 == 0 {
        (t * t) % m
    } else {
        (((t * t) % m) * x) % m
    }
}
