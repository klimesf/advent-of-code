pub(crate) fn day11() {
    let serial_number = 7511;
    // let serial_number = 42;
    // let serial_number = 18;

    let mut grid = [[0; 300]; 300];
    for y in 0..300 {
        for x in 0..300 {
            grid[y][x] = power_level((x + 1) as i32, (y + 1) as i32, serial_number);
        }
    }

    let mut summed_area_table = [[0; 300]; 300];
    for y in 0..300 {
        for x in 0..300 {
            summed_area_table[y][x] = grid[y][x];
            if y > 0 {
                summed_area_table[y][x] += summed_area_table[y - 1][x];
            }
            if x > 0 {
                summed_area_table[y][x] += summed_area_table[y][x - 1];
            }
            if x > 0 && y > 0 {
                summed_area_table[y][x] -= summed_area_table[y - 1][x - 1];
            }
        }
    }

    let mut max = i32::MIN;
    let mut max_coord = (0, 0);
    for y in 3..300 {
        for x in 3..300 {
            let sum = summed_area_table[y][x] + summed_area_table[y - 3][x - 3]
                - summed_area_table[y - 3][x]
                - summed_area_table[y][x - 3];
            if sum > max {
                max = sum;
                max_coord = (x - 1, y - 1);
            };
        }
    }
    println!("{},{} is the coord of 3x3 square with largest total power {}", max_coord.0, max_coord.1, max);

    let mut max = i32::MIN;
    let mut max_coord = (0, 0, 0);

    for size in 1..=300 {
        for y in size..300 {
            for x in size..300 {
                let sum = summed_area_table[y][x] + summed_area_table[y - size][x - size]
                    - summed_area_table[y - size][x]
                    - summed_area_table[y][x - size];
                if sum > max {
                    max = sum;
                    max_coord = (x - size + 2, y - size + 2, size);
                };
            }
        }
    }

    println!(
        "{},{} is the coord of {}x{} square with largest total power {}",
        max_coord.0, max_coord.1, max_coord.2, max_coord.2, max
    );
}

fn power_level(x: i32, y: i32, serial_number: i32) -> i32 {
    let rack_id = x + 10;
    let mut power_level = rack_id * y;
    power_level += serial_number;
    power_level *= rack_id;
    power_level = (power_level / 100) % 10;
    power_level - 5
}

#[cfg(test)]
mod day11_tests {
    use crate::y2018::day11::power_level;

    #[test]
    fn power_level_works() {
        assert_eq!(4, power_level(3, 5, 8));
        assert_eq!(-5, power_level(122, 79, 57));
        assert_eq!(0, power_level(217, 196, 39));
        assert_eq!(4, power_level(101, 153, 71));
    }
}
