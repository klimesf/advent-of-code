use crate::utils::toolbox::lcm_64;

type Coord = (i32, i32, i32);

pub(crate) fn day12() {
    // Test A
    // let moons = vec![
    //     Moon { pos: (-1, 0, 2), vel: (0, 0, 0) },
    //     Moon { pos: (2, -10, -7), vel: (0, 0, 0) },
    //     Moon { pos: (4, -8, 8), vel: (0, 0, 0) },
    //     Moon { pos: (3, 5, -1), vel: (0, 0, 0) },
    // ];

    // Test B
    // let moons = vec![
    //     Moon { pos: (-8, -10, 0), vel: (0, 0, 0) },
    //     Moon { pos: (5, 5, 10), vel: (0, 0, 0) },
    //     Moon { pos: (2, -7, 3), vel: (0, 0, 0) },
    //     Moon { pos: (9, -8, -3), vel: (0, 0, 0) },
    // ];

    // Input
    let moons = vec![
        Moon {
            pos: (-4, -9, -3),
            vel: (0, 0, 0),
        },
        Moon {
            pos: (-13, -11, 0),
            vel: (0, 0, 0),
        },
        Moon {
            pos: (-17, -7, 15),
            vel: (0, 0, 0),
        },
        Moon {
            pos: (-16, 4, 2),
            vel: (0, 0, 0),
        },
    ];

    part_a(moons.clone());
    part_b(moons.clone());
}

fn part_a(mut moons: Vec<Moon>) {
    for _ in 0..1000 {
        let old_moons = moons.clone();
        for moon_a in &mut moons {
            for moon_b in &old_moons {
                if moon_a.pos == moon_b.pos {
                    continue;
                }
                moon_a.update_vel(moon_b);
            }
        }

        for moon in &mut moons {
            moon.apply_vel();
        }
    }
    println!(
        "{}",
        moons.iter().map(|m| m.calculate_energy()).sum::<i32>()
    )
}

fn part_b(mut moons: Vec<Moon>) {
    let mut i = 0 as i64;
    let original = moons.clone();

    let mut x_full_rotation = 0;
    let mut y_full_rotation = 0;
    let mut z_full_rotation = 0;

    loop {
        let old_moons = moons.clone();
        for moon_a in &mut moons {
            for moon_b in &old_moons {
                if moon_a.pos == moon_b.pos {
                    continue;
                }
                moon_a.update_vel(moon_b);
            }
        }
        for moon in &mut moons {
            moon.apply_vel();
        }
        i += 1;

        if x_full_rotation == 0
            && original
                .iter()
                .zip(&moons)
                .filter(|&(m1, m2)| m1.pos.0 == m2.pos.0 && m1.vel.0 == m2.vel.0)
                .count()
                == 4
        {
            x_full_rotation = i;
        }
        if y_full_rotation == 0
            && original
                .iter()
                .zip(&moons)
                .filter(|&(m1, m2)| m1.pos.1 == m2.pos.1 && m1.vel.1 == m2.vel.1)
                .count()
                == 4
        {
            y_full_rotation = i;
        }
        if z_full_rotation == 0
            && original
                .iter()
                .zip(&moons)
                .filter(|&(m1, m2)| m1.pos.2 == m2.pos.2 && m1.vel.2 == m2.vel.2)
                .count()
                == 4
        {
            z_full_rotation = i;
        }

        if x_full_rotation > 0 && y_full_rotation > 0 && z_full_rotation > 0 {
            break;
        }
    }
    let cycle = lcm_64(lcm_64(x_full_rotation, y_full_rotation), z_full_rotation);
    println!("Full cycle repeated after {} steps", cycle);
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Moon {
    pos: Coord,
    vel: Coord,
}

impl Moon {
    fn calculate_energy(&self) -> i32 {
        (self.pos.0.abs() + self.pos.1.abs() + self.pos.2.abs()) // Potential
            * (self.vel.0.abs() + self.vel.1.abs() + self.vel.2.abs()) // Kinetic
    }

    fn apply_vel(&mut self) {
        self.pos.0 += self.vel.0;
        self.pos.1 += self.vel.1;
        self.pos.2 += self.vel.2;
    }

    fn update_vel(&mut self, other: &Moon) {
        let x1 = self.pos.0;
        let x2 = other.pos.0;
        if x1 > x2 {
            self.vel.0 -= 1;
        } else if x2 > x1 {
            self.vel.0 += 1;
        }

        let y1 = self.pos.1;
        let y2 = other.pos.1;
        if y1 > y2 {
            self.vel.1 -= 1;
        } else if y2 > y1 {
            self.vel.1 += 1;
        }

        let z1 = self.pos.2;
        let z2 = other.pos.2;
        if z1 > z2 {
            self.vel.2 -= 1;
        } else if z2 > z1 {
            self.vel.2 += 1;
        }
    }
}
