use std::collections::{HashMap, HashSet};
use std::fs;

pub(crate) fn day11(print: fn(usize)) {
    print(part_a(fs::read_to_string("input/2025/day11/input.txt").unwrap()));
    print(part_b(fs::read_to_string("input/2025/day11/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    input.lines().for_each(|line| {
        let (l, r) = line.split_once(": ").unwrap();
        let edges: Vec<&str> = r.split_whitespace().collect();
        map.insert(l, edges);
    });
    
    let mut count = 0;
    let mut stack = vec!();
    let start = "you";
    stack.push((start, HashSet::new()));
    
    while let Some((pos, visited)) = stack.pop() {
        if pos == "out" {
            count += 1;
            continue;
        }
        let mut new_visited = visited.clone();
        if !new_visited.insert(pos) {
            continue;
        }

        for to in map[pos].iter() {
            stack.push((*to, new_visited.clone()));
        }
    }
    
    count
}

fn part_b(input: String) -> usize {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    input.lines().for_each(|line| {
        let (l, r) = line.split_once(": ").unwrap();
        let edges: Vec<String> = r.split_whitespace().map(|s| s.to_string()).collect();
        map.insert(l.to_string(), edges);
    });

    // There is no path from dac to fft, making it much simpler for us
    let mut cache = HashMap::new();
    let svr_to_fft = search("svr".to_string(), "fft".to_string(), &map, &mut cache);
    
    let mut cache = HashMap::new();
    let fft_to_dac = search("fft".to_string(), "dac".to_string(), &map, &mut cache);

    let mut cache = HashMap::new();
    let dac_to_out = search("dac".to_string(), "out".to_string(), &map, &mut cache);

    svr_to_fft * fft_to_dac * dac_to_out
}

fn search(pos: String, target: String, map: &HashMap<String, Vec<String>>, cache: &mut HashMap<String, usize>) -> usize {
    if pos == target {
        return 1;
    }
    if cache.contains_key(pos.as_str()) {
        return *cache.get(pos.as_str()).unwrap();
    }
    if !map.contains_key(pos.as_str()) {
        return 0;
    }

    let mut count = 0;
    for to in map[&pos.to_string()].iter() {
        count += search(to.to_string(), target.to_string(), map, cache);
    }
    cache.insert(pos, count);
    count
}

#[cfg(test)]
mod day11_tests {
    use std::fs;

    use crate::y2025::day11::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(5, part_a(fs::read_to_string("input/2025/day11/test.txt").unwrap()));
        assert_eq!(2, part_b(fs::read_to_string("input/2025/day11/test_b.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(571, part_a(fs::read_to_string("input/2025/day11/input.txt").unwrap()));
        assert_eq!(511378159390560, part_b(fs::read_to_string("input/2025/day11/input.txt").unwrap()));
    }
}
