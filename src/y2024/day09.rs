use std::fs;

pub fn day09(print: fn(i64)) {
    print(part_a(fs::read_to_string("input/2024/day09/input.txt").unwrap()));
    print(part_b(fs::read_to_string("input/2024/day09/input.txt").unwrap()));
}

fn part_a(input: String) -> i64 {
    let mut disk: Vec<i64> = vec![];
    let mut file_mode = true;
    let mut id = 0;
    for c in input.chars() {
        let size = c.to_digit(10).unwrap();
        if file_mode {
            for _ in 0..size {
                disk.push(id);
            }
            id += 1;
        } else {
            for _ in 0..size {
                disk.push(-1);
            }
        }
        file_mode = !file_mode;
    }

    let mut last = disk.len() - 1;
    let mut ans = 0;
    'outer: for i in 0..disk.len() {
        if disk[i] != -1 {
            ans += i as i64 * disk[i];
            continue;
        }

        loop {
            if disk[last] != -1 {
                break;
            }
            if last <= i {
                break 'outer;
            }
            last -= 1;
        }

        if i >= last {
            break 'outer;
        }

        disk[i] = disk[last];
        disk[last] = -1;
        last -= 1;

        ans += i as i64 * disk[i];
    }

    ans
}

fn part_b(input: String) -> i64 {
    // Build the files and the free spaces as intervals, files also carry id
    let mut files: Vec<(i64, usize, usize)> = vec![];
    let mut free_spaces: Vec<(usize, usize)> = vec![];
    let mut file_mode = true;
    let mut i = 0;
    let mut id = 0;
    for c in input.chars() {
        let len = c.to_digit(10).unwrap() as usize;
        if file_mode {
            files.push((id, i, i + len - 1));
            id += 1;
        } else {
            free_spaces.push((i, i + len - 1));
        }
        i += len;
        file_mode = !file_mode;
    }

    let mut ans = 0;
    for i in (0..files.len()).rev() {
        let (file_id, file_start, file_end) = files[i];
        for j in 0..free_spaces.len() {
            let (free_start, free_end) = free_spaces[j];
            if free_start >= file_start {
                break;
            } // Overshot

            if free_start > free_end {
                continue;
            } // Free space exhausted
            if free_end - free_start < file_end - file_start {
                continue;
            } // Does not fit, continue

            let file_start_new = free_start;
            let file_end_new = free_start + (file_end - file_start);
            let free_start_new = file_end_new + 1;
            let free_end_new = free_end;

            files[i] = (file_id, file_start_new, file_end_new);
            free_spaces[j] = (free_start_new, free_end_new);

            // Fits, file is placed
            break;
        }

        // Add to ans, regardless the file moved or not
        for j in files[i].1..=files[i].2 {
            ans += j as i64 * file_id;
        }
    }
    ans
}

#[cfg(test)]
mod day09_tests {
    use std::fs;

    use crate::y2024::day09::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(1928, part_a(fs::read_to_string("input/2024/day09/test.txt").unwrap()));
        assert_eq!(2858, part_b(fs::read_to_string("input/2024/day09/test.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(6337367222422, part_a(fs::read_to_string("input/2024/day09/input.txt").unwrap()));
        assert_eq!(6361380647183, part_b(fs::read_to_string("input/2024/day09/input.txt").unwrap()));
    }
}
