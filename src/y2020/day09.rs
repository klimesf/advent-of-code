use std::fs;

pub(crate) fn day09() {
    let ans = solve(
        fs::read_to_string("input/2020/day09/input.txt").unwrap(),
        25,
    );
    println!("{}", ans.0);
    println!("{}", ans.1);
}

fn solve(input: String, size: usize) -> (usize, usize) {
    let numbers = input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut vulnerability = 0;
    'outer: for i in size..numbers.len() {
        let num = numbers[i];

        for m in i - size..i {
            for n in m + 1..i {
                if num == numbers[m] + numbers[n] {
                    continue 'outer;
                }
            }
        }

        vulnerability = num;
        break;
    }

    let mut weakness = 0;
    'outer2: for i in 0..numbers.len() {
        let mut sum = numbers[i];
        let mut line = vec![];
        line.push(numbers[i]);
        for j in i + 1..numbers.len() {
            line.push(numbers[j]);
            sum += numbers[j];
            if sum == vulnerability {
                weakness = line.iter().min().unwrap() + line.iter().max().unwrap();
                break 'outer2;
            }
            if sum > vulnerability {
                continue 'outer2;
            }
        }
    }

    (vulnerability, weakness)
}

#[cfg(test)]
mod day09_tests {
    use std::fs;

    use crate::y2020::day09::solve;

    #[test]
    fn test_works() {
        assert_eq!(
            (127, 62),
            solve(fs::read_to_string("input/2020/day09/test.txt").unwrap(), 5)
        );
    }

    #[test]
    fn input_works() {
        assert_eq!(
            (375054920, 54142584),
            solve(
                fs::read_to_string("input/2020/day09/input.txt").unwrap(),
                25
            )
        );
    }
}
