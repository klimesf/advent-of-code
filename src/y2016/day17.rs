use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub(crate) fn day17() {
    println!("{}", part_a("njfxhljp"));
    println!("{}", part_b("njfxhljp"));
}

fn part_a(password: &str) -> String {
    let mut stack: BinaryHeap<Pos> = BinaryHeap::new();
    stack.push(Pos {
        r: 0,
        c: 0,
        steps: String::new(),
    });

    while let Some(pos) = stack.pop() {
        if pos.reached_fin() {
            return pos.steps;
        }

        let hash: Vec<char> = format!("{:x}", md5::compute(format!("{}{}", password, pos.steps)))
            .chars()
            .collect();
        let up_open = hash[0] == 'b' || hash[0] == 'c' || hash[0] == 'd' || hash[0] == 'e' || hash[0] == 'f';
        let down_open = hash[1] == 'b' || hash[1] == 'c' || hash[1] == 'd' || hash[1] == 'e' || hash[1] == 'f';
        let left_open = hash[2] == 'b' || hash[2] == 'c' || hash[2] == 'd' || hash[2] == 'e' || hash[2] == 'f';
        let right_open = hash[3] == 'b' || hash[3] == 'c' || hash[3] == 'd' || hash[3] == 'e' || hash[3] == 'f';

        if pos.r > 0 && up_open {
            stack.push(Pos {
                r: pos.r - 1,
                c: pos.c,
                steps: format!("{}U", pos.steps),
            })
        }
        if pos.r < 3 && down_open {
            stack.push(Pos {
                r: pos.r + 1,
                c: pos.c,
                steps: format!("{}D", pos.steps),
            })
        }
        if pos.c > 0 && left_open {
            stack.push(Pos {
                r: pos.r,
                c: pos.c - 1,
                steps: format!("{}L", pos.steps),
            })
        }
        if pos.c < 3 && right_open {
            stack.push(Pos {
                r: pos.r,
                c: pos.c + 1,
                steps: format!("{}R", pos.steps),
            })
        }
    }

    panic!("never reached 3, 3");
}

fn part_b(password: &str) -> usize {
    let mut stack: BinaryHeap<Pos> = BinaryHeap::new();
    stack.push(Pos {
        r: 0,
        c: 0,
        steps: String::new(),
    });

    let mut max = 0;
    while let Some(pos) = stack.pop() {
        if pos.reached_fin() {
            if max < pos.steps.len() {
                max = pos.steps.len()
            }
            continue;
        }

        let hash: Vec<char> = format!("{:x}", md5::compute(format!("{}{}", password, pos.steps)))
            .chars()
            .collect();
        let up_open = hash[0] == 'b' || hash[0] == 'c' || hash[0] == 'd' || hash[0] == 'e' || hash[0] == 'f';
        let down_open = hash[1] == 'b' || hash[1] == 'c' || hash[1] == 'd' || hash[1] == 'e' || hash[1] == 'f';
        let left_open = hash[2] == 'b' || hash[2] == 'c' || hash[2] == 'd' || hash[2] == 'e' || hash[2] == 'f';
        let right_open = hash[3] == 'b' || hash[3] == 'c' || hash[3] == 'd' || hash[3] == 'e' || hash[3] == 'f';

        if pos.r > 0 && up_open {
            stack.push(Pos {
                r: pos.r - 1,
                c: pos.c,
                steps: format!("{}U", pos.steps),
            })
        }
        if pos.r < 3 && down_open {
            stack.push(Pos {
                r: pos.r + 1,
                c: pos.c,
                steps: format!("{}D", pos.steps),
            })
        }
        if pos.c > 0 && left_open {
            stack.push(Pos {
                r: pos.r,
                c: pos.c - 1,
                steps: format!("{}L", pos.steps),
            })
        }
        if pos.c < 3 && right_open {
            stack.push(Pos {
                r: pos.r,
                c: pos.c + 1,
                steps: format!("{}R", pos.steps),
            })
        }
    }

    max
}

#[derive(Clone, Eq, PartialEq)]
struct Pos {
    r: usize,
    c: usize,
    steps: String,
}

impl Pos {
    fn manhattan_distance(&self) -> usize {
        (3 - self.r) + (3 - self.c)
    }

    fn reached_fin(&self) -> bool {
        self.r == 3 && self.c == 3
    }
}

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> Ordering {
        other.manhattan_distance().cmp(&self.manhattan_distance())
    }
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod day17_tests {
    use crate::y2016::day17::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!("DDRRRD", part_a("ihgpwlah"));
        assert_eq!("DDUDRLRRUDRD", part_a("kglvqrro"));
        assert_eq!(370, part_b("ihgpwlah"));
        assert_eq!(492, part_b("kglvqrro"));
    }

    #[test]
    fn input_works() {
        assert_eq!("DURLDRRDRD", part_a("njfxhljp"));
        assert_eq!(650, part_b("njfxhljp"));
    }
}
