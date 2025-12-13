use crate::utils::toolbox::parse_i32;
use regex::Regex;
use serde_json::Value;
use std::fs;

pub(crate) fn day12() {
    println!(
        "{}",
        part_a(fs::read_to_string("input/2015/day12/input.txt").unwrap())
    );
    println!(
        "{}",
        part_b(fs::read_to_string("input/2015/day12/input.txt").unwrap())
    );
}

fn part_a(input: String) -> i32 {
    let re = Regex::new("(:|\\[|,)(-?[0-9]+)").unwrap();
    re.captures_iter(input.as_str())
        .map(|x| parse_i32(x.get(2)))
        .sum()
}

fn part_b(input: String) -> i64 {
    if let Ok(v) = serde_json::from_str(input.as_str()) {
        let mut ans = 0;
        recursive_sum(&v, &mut ans);
        ans
    } else {
        panic!("couldnt parse json")
    }
}

fn recursive_sum(v: &Value, total: &mut i64) {
    match v {
        Value::Null => {}
        Value::Bool(_) => {}
        Value::Number(n) => *total += n.as_i64().unwrap(),
        Value::String(_) => {}
        Value::Array(a) => a.iter().for_each(|vv| {
            recursive_sum(vv, total);
        }),
        Value::Object(o) => {
            if o.values().any(|vv| *vv == Value::String("red".to_string())) {
                return;
            }
            o.values().for_each(|vv| recursive_sum(vv, total))
        }
    }
}

#[cfg(test)]
mod day12_tests {
    use std::fs;

    use crate::y2015::day12::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(6, part_a("[1,2,3]".to_string()));
        assert_eq!(6, part_a("{\"a\":2,\"b\":4}".to_string()));
        assert_eq!(3, part_a("{\"a\":{\"b\":4},\"c\":-1}".to_string()));
        assert_eq!(3, part_a("[[[3]]]".to_string()));
        assert_eq!(0, part_a("{\"a\":[-1,1]}".to_string()));
        assert_eq!(0, part_a("[-1,{\"a\":1}]".to_string()));
        assert_eq!(0, part_a("[]".to_string()));
        assert_eq!(0, part_a("{}".to_string()));

        assert_eq!(6, part_b("[1,2,3]".to_string()));
        assert_eq!(4, part_b("[1,{\"c\":\"red\",\"b\":2},3]".to_string()));
        assert_eq!(
            0,
            part_b("{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}".to_string())
        );
        assert_eq!(6, part_b("[1,\"red\",5]".to_string()));
    }

    #[test]
    fn input_works() {
        assert_eq!(
            119433,
            part_a(fs::read_to_string("input/2015/day12/input.txt").unwrap())
        );
        assert_eq!(
            68466,
            part_b(fs::read_to_string("input/2015/day12/input.txt").unwrap())
        );
    }
}
