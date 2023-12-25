pub(crate) fn day04() {
    part_a();
    part_b();
}

fn part_a() {
    let mut ctr = 0;
    for pass_num in 134792..=675810 {
        let pass: Vec<u32> = pass_num.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();

        let mut never_decreases = true;
        let mut doubles = false;

        let mut last_num = 0;
        for num in &pass {
            if last_num == *num {
                doubles = true;
            }
            if last_num != 0 && last_num > *num {
                never_decreases = last_num == 0;
            }
            last_num = *num;
        }

        if never_decreases && doubles { ctr += 1; }
    }
    println!("{}", ctr);
}

fn part_b() {
    let mut ctr = 0;
    for pass_num in 134792..=675810 {
        let pass: Vec<u32> = pass_num.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();

        let mut never_decreases = true;
        let mut doubles = false;

        let mut last_num = 0;
        for num in &pass {
            if last_num != 0 && last_num > *num {
                never_decreases = last_num == 0;
            }
            last_num = *num;
        }

        if pass[0] == pass[1] && pass[1] != pass[2] {
            doubles = true;
        }
        if pass[1] == pass[2] && pass[0] != pass[1] && pass[2] != pass[3] {
            doubles = true;
        }
        if pass[2] == pass[3] && pass[1] != pass[2] && pass[3] != pass[4] {
            doubles = true;
        }
        if pass[3] == pass[4] && pass[2] != pass[3] && pass[4] != pass[5] {
            doubles = true;
        }
        if pass[4] == pass[5] && pass[3] != pass[4] {
            doubles = true;
        }

        if never_decreases && doubles { ctr += 1; }
    }
    println!("{}", ctr);
}
