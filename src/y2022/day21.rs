use std::collections::HashMap;
use std::fs;
use crate::utils::toolbox::{gcd_64};

static ZERO: Fraction = (0, 1);

pub(crate) fn day21() {
    let input = fs::read_to_string("input/2022/day21/input.txt").unwrap();
    let monkeys: HashMap<&str, Job> = input.lines().into_iter().map(|line| {
        let (name, job) = line.split_once(": ").unwrap();
        if job.contains("+") {
            let (left, right) = job.split_once(" + ").unwrap();
            (name, Job::OPERATION(left.to_string(), right.to_string(), Operation::PLUS))
        } else if job.contains("-") {
            let (left, right) = job.split_once(" - ").unwrap();
            (name, Job::OPERATION(left.to_string(), right.to_string(), Operation::MINUS))
        } else if job.contains("*") {
            let (left, right) = job.split_once(" * ").unwrap();
            (name, Job::OPERATION(left.to_string(), right.to_string(), Operation::MULTIPLY))
        } else if job.contains("/") {
            let (left, right) = job.split_once(" / ").unwrap();
            (name, Job::OPERATION(left.to_string(), right.to_string(), Operation::DIVIDE))
        } else {
            (name, Job::NUMBER(job.parse().unwrap()))
        }
    }).collect();

    let mut ans: HashMap<&str, i64> = HashMap::new();
    while !ans.contains_key("root") {
        monkeys.iter().for_each(|(name, job)| {
            match job {
                Job::NUMBER(num) => { ans.insert(name, *num); },
                Job::OPERATION(left, right, operation) => {
                    let left = left.as_str();
                    let right = right.as_str();
                    if ans.contains_key(left) && ans.contains_key(right) {
                        match operation {
                            Operation::PLUS => { ans.insert(name, ans.get(left).unwrap() + ans.get(right).unwrap()) },
                            Operation::MINUS => { ans.insert(name, ans.get(left).unwrap() - ans.get(right).unwrap()) },
                            Operation::MULTIPLY => { ans.insert(name, ans.get(left).unwrap() * ans.get(right).unwrap()) },
                            Operation::DIVIDE => { ans.insert(name, ans.get(left).unwrap() / ans.get(right).unwrap()) },
                        };
                    }
                },
            };
        });
    }
    println!("{}", ans.get("root").unwrap());

    let equation = solve("root", &monkeys);
    println!("{}", equation.1.0 / equation.0.0);
}

fn solve(monkey: &str, monkeys: &HashMap<&str, Job>) -> (Fraction, Fraction) { // -> (x, constant, multiply other side)
    match monkeys.get(monkey).unwrap() {
        Job::NUMBER(x) => {
            if monkey == "humn" {
                ((1, 1), ZERO)
            } else {
                (ZERO, (*x, 1))
            }
        }
        Job::OPERATION(left, right, operation) => {
            let ans_left = solve(left, monkeys);
            let ans_right = solve(right, monkeys);

            if monkey == "root" {
                return if ans_left.0 != ZERO {
                    let ans = subtract_fraction(ans_right.1, ans_left.1);
                    (ans_left.0, ans)
                } else if ans_right.0 != ZERO {
                    let ans  = subtract_fraction(ans_left.1, ans_right.1);
                    (ans_right.0, ans)
                } else {
                    panic!("no x to be found")
                }
            }

            return match operation {
                Operation::PLUS => { add_polynome(ans_left, ans_right) }
                Operation::MINUS => { subtract_polynome(ans_left, ans_right) }
                Operation::MULTIPLY => { multiply_polynome(ans_left, ans_right) }
                Operation::DIVIDE => { divide_polynome(ans_left, ans_right) }
            }
        }
    }
}

#[derive(Clone)]
enum Operation {
    PLUS,
    MINUS,
    MULTIPLY,
    DIVIDE,
}

#[derive(Clone)]
enum Job {
    NUMBER(i64),
    OPERATION(String, String, Operation), // Left, Right, Operation
}

type Fraction = (i64, i64);
type Polynome = (Fraction, Fraction);

fn add_polynome(left: Polynome, right: Polynome) -> Polynome {
    (add_fraction(left.0, right.0), add_fraction(left.1, right.1))
}

fn subtract_polynome(left: Polynome, right: Polynome) -> Polynome {
    (subtract_fraction(left.0, right.0), subtract_fraction(left.1, right.1))
}

fn multiply_polynome(left: Polynome, right: Polynome) -> Polynome {
    if left.0 != ZERO && right.0 != ZERO {
        panic!("x^2");
    } else if left.0 != ZERO {
        (multiply_fraction(left.0, right.1), multiply_fraction(left.1, right.1))
    } else if right.0 != ZERO {
        (multiply_fraction(right.0, left.1), multiply_fraction(right.1, left.1))
    } else {
        (ZERO, multiply_fraction(left.1, right.1))
    }
}

fn divide_polynome(left: Polynome, right: Polynome) -> Polynome {
    if right.0 != ZERO {
        panic!("division by x")
    } else if right.1 == ZERO {
        panic!("division by zero")
    } else {
        (divide_fraction(left.0, right.1), divide_fraction(left.1, right.1))
    }
}

fn add_fraction(left: Fraction, right: Fraction) -> Fraction {
    collapse((left.0 * right.1 + left.1 * right.0, left.1 * right.1))
}

fn subtract_fraction(left: Fraction, right: Fraction) -> Fraction {
    collapse((left.0 * right.1 - left.1 * right.0, left.1 * right.1))
}

fn multiply_fraction(left: Fraction, right: Fraction) -> Fraction {
    collapse((left.0 * right.0, left.1 * right.1))
}

fn divide_fraction(left: Fraction, right: Fraction) -> Fraction {
    collapse(multiply_fraction(left, (right.1, right.0)))
}

fn collapse(num: Fraction) -> Fraction {
    if num.1 == 1 { return num }
    let gcd = gcd_64(num.0, num.1);
    let mut ans = (num.0 / gcd, num.1 / gcd);
    if ans.0 < 0 && ans.1 < 0 {
        ans.0 = ans.0.abs();
        ans.1 = ans.1.abs();
    }
    return ans
}

#[cfg(test)]
mod day17_tests {
    use crate::y2022::day21::{add_polynome, collapse, divide_fraction, divide_polynome, multiply_fraction, multiply_polynome, subtract_polynome, ZERO};

    #[test]
    fn collapse_works() {
        assert_eq!((1, 2), collapse((2, 4)));
        assert_eq!((2, 1), collapse((4, 2)));
        assert_eq!((13, 1), collapse((13, 1)));
        assert_eq!((13, 2), collapse((13, 2)));
        assert_eq!((0, 1), collapse((0, 2)));
    }

    #[test]
    fn add_polynome_works() {
        assert_eq!(((2, 1), ZERO), add_polynome(((1, 1), ZERO), ((1, 1), ZERO)));
        assert_eq!(((5, 6), ZERO), add_polynome(((1, 2), ZERO), ((1, 3), ZERO)));
    }

    #[test]
    fn subtract_polynome_works() {
        assert_eq!((ZERO, ZERO), subtract_polynome(((1, 1), ZERO), ((1, 1), ZERO)));
    }

    #[test]
    fn multiply_polynome_works() {
        assert_eq!(((2, 1), (2, 1)), multiply_polynome(((1, 1), (1, 1)), (ZERO, (2, 1))));
        assert_eq!(((2, 1), (2, 1)), multiply_polynome((ZERO, (2, 1)), ((1, 1), (1, 1))));
    }

    #[test]
    fn divide_polynome_works() {
        assert_eq!(((1, 2), (1, 2)), divide_polynome(((1, 1), (1, 1)), (ZERO, (2, 1))));
        assert_eq!((ZERO, (1, 1)), divide_polynome((ZERO, (2, 1)), (ZERO, (2, 1))));
    }

    #[test]
    fn multiply_fraction_works() {
        assert_eq!((1, 2), multiply_fraction((1, 1), (1, 2)));
        assert_eq!((2, 1), multiply_fraction((1, 1), (2, 1)));
        assert_eq!((1, 1), multiply_fraction((-1, 1), (-1, 1)));
    }

    #[test]
    fn divide_fraction_works() {
        assert_eq!((2, 1), divide_fraction((1, 1), (1, 2)));
        assert_eq!((1, 2), divide_fraction((1, 1), (2, 1)));
        assert_eq!((1, 1), divide_fraction((-1, 1), (-1, 1)));
    }
}
