use crate::hashmap;
use std::collections::HashMap;
use std::fs;

pub(crate) fn day22() {
    let input = fs::read_to_string("input/2022/day22/input.txt").unwrap();
    let (map_input, instructions_input) = input.split_once("\n\n").unwrap();
    let map: Vec<Vec<Tile>> = map_input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    ' ' => Tile::NOTHING,
                    '.' => Tile::TILE,
                    '#' => Tile::WALL,
                    _ => panic!("Unknown tile {}", c),
                })
                .collect()
        })
        .collect();
    let instructions = parse_instructions(instructions_input);

    part_a(&map, &instructions);

    // let (start_coords, size, moves) = load_test();
    let (start_coords, size, moves) = load_input();
    part_b(&start_coords, size, &moves, &map, &instructions);
}

#[allow(dead_code)]
fn load_test() -> (
    Vec<(usize, usize)>,
    usize,
    HashMap<usize, HashMap<Direction, (usize, Direction)>>,
) {
    // Test
    // __1_
    // 234_
    // __56
    let start_coords = vec![(0, 8), (4, 0), (4, 4), (4, 8), (8, 8), (8, 12)];
    let size = 4;
    let moves = hashmap![
        1 => hashmap![
            Direction::UP => (2, Direction::DOWN),
            Direction::RIGHT => (6, Direction::LEFT),
            Direction::DOWN => (4, Direction::DOWN),
            Direction::LEFT => (3, Direction::DOWN)
        ],
        2 => hashmap![
            Direction::UP => (1, Direction::DOWN),
            Direction::RIGHT => (3, Direction::RIGHT),
            Direction::DOWN => (5, Direction::UP),
            Direction::LEFT => (6, Direction::UP)
        ],
        3 => hashmap![
            Direction::UP => (1, Direction::RIGHT),
            Direction::RIGHT => (4, Direction::RIGHT),
            Direction::DOWN => (5, Direction::RIGHT),
            Direction::LEFT => (2, Direction::LEFT)
        ],
        4 => hashmap![
            Direction::UP => (1, Direction::UP),
            Direction::RIGHT => (6, Direction::DOWN),
            Direction::DOWN => (5, Direction::DOWN),
            Direction::LEFT => (3, Direction::LEFT)
        ],
        5 => hashmap![
            Direction::UP => (4, Direction::UP),
            Direction::RIGHT => (6, Direction::RIGHT),
            Direction::DOWN => (2, Direction::UP),
            Direction::LEFT => (3, Direction::UP)
        ],
        6 => hashmap![
            Direction::UP => (4, Direction::LEFT),
            Direction::RIGHT => (1, Direction::LEFT),
            Direction::DOWN => (2, Direction::RIGHT),
            Direction::LEFT => (5, Direction::LEFT)
        ]
    ];
    (start_coords, size, moves)
}

fn load_input() -> (
    Vec<(usize, usize)>,
    usize,
    HashMap<usize, HashMap<Direction, (usize, Direction)>>,
) {
    // _12
    // _3_
    // 45_
    // 6__
    let start_coords = vec![(0, 50), (0, 100), (50, 50), (100, 0), (100, 50), (150, 0)];
    let size = 50;
    let moves = hashmap![
        1 => hashmap![
            Direction::UP => (6, Direction::RIGHT),
            Direction::RIGHT => (2, Direction::RIGHT),
            Direction::DOWN => (3, Direction::DOWN),
            Direction::LEFT => (4, Direction::RIGHT)
        ],
        2 => hashmap![
            Direction::UP => (6, Direction::UP),
            Direction::RIGHT => (5, Direction::LEFT),
            Direction::DOWN => (3, Direction::LEFT),
            Direction::LEFT => (1, Direction::LEFT)
        ],
        3 => hashmap![
            Direction::UP => (1, Direction::UP),
            Direction::RIGHT => (2, Direction::UP),
            Direction::DOWN => (5, Direction::DOWN),
            Direction::LEFT => (4, Direction::DOWN)
        ],
        4 => hashmap![
            Direction::UP => (3, Direction::RIGHT),
            Direction::RIGHT => (5, Direction::RIGHT),
            Direction::DOWN => (6, Direction::DOWN),
            Direction::LEFT => (1, Direction::RIGHT)
        ],
        5 => hashmap![
            Direction::UP => (3, Direction::UP),
            Direction::RIGHT => (2, Direction::LEFT),
            Direction::DOWN => (6, Direction::LEFT),
            Direction::LEFT => (4, Direction::LEFT)
        ],
        6 => hashmap![
            Direction::UP => (4, Direction::UP),
            Direction::RIGHT => (5, Direction::UP),
            Direction::DOWN => (2, Direction::DOWN),
            Direction::LEFT => (1, Direction::DOWN)
        ]
    ];
    (start_coords, size, moves)
}

fn part_a(map: &Vec<Vec<Tile>>, instructions: &Vec<Instruction>) {
    let mut direction = Direction::RIGHT;
    let mut pos = find_start_right(0, &map);
    for inst in instructions {
        match inst {
            Instruction::MOVE(by) => {
                let d: (i32, i32) = match direction {
                    Direction::RIGHT => (0, 1),
                    Direction::DOWN => (1, 0),
                    Direction::LEFT => (0, -1),
                    Direction::UP => (-1, 0),
                };
                for _ in 0..*by {
                    let new_x = pos.0 as i32 + d.0;
                    let new_y = pos.1 as i32 + d.1;
                    let mut new_pos = (new_x as usize, new_y as usize);
                    if new_x < 0
                        || new_pos.0 >= map.len()
                        || new_y < 0
                        || new_pos.1 >= map[new_pos.0].len()
                        || map[new_pos.0][new_pos.1] == Tile::NOTHING
                    {
                        new_pos = match direction {
                            Direction::RIGHT => find_start_right(new_pos.0, &map),
                            Direction::DOWN => find_start_down(new_pos.1, &map),
                            Direction::LEFT => find_start_left(new_pos.0, &map),
                            Direction::UP => find_start_up(new_pos.1, &map),
                        };
                    }

                    if map[new_pos.0][new_pos.1] == Tile::WALL {
                        break;
                    } else {
                        pos = new_pos;
                    }
                }
            }
            Instruction::RIGHT => {
                direction = match direction {
                    Direction::RIGHT => Direction::DOWN,
                    Direction::DOWN => Direction::LEFT,
                    Direction::LEFT => Direction::UP,
                    Direction::UP => Direction::RIGHT,
                }
            }
            Instruction::LEFT => {
                direction = match direction {
                    Direction::RIGHT => Direction::UP,
                    Direction::DOWN => Direction::RIGHT,
                    Direction::LEFT => Direction::DOWN,
                    Direction::UP => Direction::LEFT,
                }
            }
        }
    }

    let ans = 1000 * (pos.0 + 1) + 4 * (pos.1 + 1) + direction.score();
    println!("{}", ans);
}

fn part_b(
    start_coords: &Vec<(usize, usize)>,
    size: usize,
    moves: &HashMap<usize, HashMap<Direction, (usize, Direction)>>,
    map: &Vec<Vec<Tile>>,
    instructions: &Vec<Instruction>,
) {
    let mut quadrant = 1;
    let mut quadrant_bounds = (
        start_coords[quadrant - 1].0,
        start_coords[quadrant - 1].0 + size - 1,
        start_coords[quadrant - 1].1,
        start_coords[quadrant - 1].1 + size - 1,
    );
    let mut direction = Direction::RIGHT;
    let mut pos = find_start_right(0, &map);
    for inst in instructions {
        match inst {
            Instruction::MOVE(by) => {
                for _ in 0..*by {
                    let d: (i32, i32) = match direction {
                        Direction::RIGHT => (0, 1),
                        Direction::DOWN => (1, 0),
                        Direction::LEFT => (0, -1),
                        Direction::UP => (-1, 0),
                    };
                    let new_x = pos.0 as i32 + d.0;
                    let new_y = pos.1 as i32 + d.1;
                    let mut new_pos = (new_x as usize, new_y as usize);
                    let mut new_quadrant = quadrant;
                    let mut new_dir = direction;
                    let mut to_bounds = quadrant_bounds;
                    if new_pos.0 < quadrant_bounds.0
                        || new_pos.0 > quadrant_bounds.1
                        || new_pos.1 < quadrant_bounds.2
                        || new_pos.1 > quadrant_bounds.3
                    {
                        let (to_quadrant, to_direction) =
                            moves.get(&quadrant).unwrap().get(&direction).unwrap();
                        let relative_pos = (
                            pos.0 - start_coords[quadrant - 1].0,
                            pos.1 - start_coords[quadrant - 1].1,
                        );
                        to_bounds = (
                            start_coords[to_quadrant - 1].0,
                            start_coords[to_quadrant - 1].0 + size - 1,
                            start_coords[to_quadrant - 1].1,
                            start_coords[to_quadrant - 1].1 + size - 1,
                        );
                        new_pos = match (direction, to_direction) {
                            (Direction::UP, Direction::UP) => {
                                (to_bounds.1, to_bounds.2 + relative_pos.1)
                            }
                            (Direction::UP, Direction::DOWN) => {
                                (to_bounds.0, to_bounds.3 - relative_pos.1)
                            }
                            (Direction::UP, Direction::RIGHT) => {
                                (to_bounds.0 + relative_pos.1, to_bounds.2)
                            }
                            (Direction::UP, Direction::LEFT) => {
                                (to_bounds.1 - relative_pos.1, to_bounds.3)
                            }
                            (Direction::DOWN, Direction::UP) => {
                                (to_bounds.1, to_bounds.3 - relative_pos.1)
                            }
                            (Direction::DOWN, Direction::DOWN) => {
                                (to_bounds.0, to_bounds.2 + relative_pos.1)
                            }
                            (Direction::DOWN, Direction::RIGHT) => {
                                (to_bounds.1 - relative_pos.1, to_bounds.2)
                            }
                            (Direction::DOWN, Direction::LEFT) => {
                                (to_bounds.0 + relative_pos.1, to_bounds.3)
                            }
                            (Direction::LEFT, Direction::UP) => {
                                (to_bounds.1, to_bounds.3 - relative_pos.0)
                            }
                            (Direction::LEFT, Direction::DOWN) => {
                                (to_bounds.0, to_bounds.2 + relative_pos.0)
                            }
                            (Direction::LEFT, Direction::RIGHT) => {
                                (to_bounds.1 - relative_pos.0, to_bounds.2)
                            }
                            (Direction::LEFT, Direction::LEFT) => {
                                (to_bounds.0 + relative_pos.0, to_bounds.3)
                            }
                            (Direction::RIGHT, Direction::UP) => {
                                (to_bounds.1, to_bounds.2 + relative_pos.0)
                            }
                            (Direction::RIGHT, Direction::DOWN) => {
                                (to_bounds.0, to_bounds.3 - relative_pos.0)
                            }
                            (Direction::RIGHT, Direction::RIGHT) => {
                                (to_bounds.0 + relative_pos.0, to_bounds.2)
                            }
                            (Direction::RIGHT, Direction::LEFT) => {
                                (to_bounds.1 - relative_pos.0, to_bounds.3)
                            }
                        };
                        new_quadrant = *to_quadrant;
                        new_dir = *to_direction;
                    }

                    if map[new_pos.0 as usize][new_pos.1 as usize] == Tile::WALL {
                        break;
                    } else {
                        pos = (new_pos.0 as usize, new_pos.1 as usize);
                        quadrant = new_quadrant;
                        direction = new_dir;
                        quadrant_bounds = to_bounds;
                    }
                }
            }
            Instruction::RIGHT => {
                direction = match direction {
                    Direction::RIGHT => Direction::DOWN,
                    Direction::DOWN => Direction::LEFT,
                    Direction::LEFT => Direction::UP,
                    Direction::UP => Direction::RIGHT,
                }
            }
            Instruction::LEFT => {
                direction = match direction {
                    Direction::RIGHT => Direction::UP,
                    Direction::DOWN => Direction::RIGHT,
                    Direction::LEFT => Direction::DOWN,
                    Direction::UP => Direction::LEFT,
                }
            }
        }
    }

    let ans = 1000 * (pos.0 + 1) + 4 * (pos.1 + 1) + direction.score();
    println!("{}", ans);
}

fn parse_instructions(instructions_input: &str) -> Vec<Instruction> {
    let mut instructions = vec![];
    let mut buf = String::new();
    for c in instructions_input.chars() {
        if c == 'L' || c == 'R' {
            if !buf.is_empty() {
                instructions.push(Instruction::MOVE(buf.parse().unwrap()));
                buf.clear();
            }
            instructions.push(if c == 'L' {
                Instruction::LEFT
            } else {
                Instruction::RIGHT
            });
        } else {
            buf.push(c);
        }
    }
    if !buf.is_empty() {
        instructions.push(Instruction::MOVE(buf.trim().parse().unwrap()));
        buf.clear();
    }
    instructions
}

fn find_start_right(x: usize, map: &Vec<Vec<Tile>>) -> (usize, usize) {
    let y = map[x]
        .iter()
        .enumerate()
        .filter(|(_, t)| **t != Tile::NOTHING)
        .map(|(i, _)| i)
        .min()
        .unwrap();
    (x, y)
}

fn find_start_left(x: usize, map: &Vec<Vec<Tile>>) -> (usize, usize) {
    let y = map[x]
        .iter()
        .enumerate()
        .filter(|(_, t)| **t != Tile::NOTHING)
        .map(|(i, _)| i)
        .max()
        .unwrap();
    (x, y)
}

fn find_start_up(y: usize, map: &Vec<Vec<Tile>>) -> (usize, usize) {
    for x in (0..map.len()).rev() {
        if y >= map[x].len() {
            continue;
        }
        if map[x][y] != Tile::NOTHING {
            return (x, y);
        }
    }
    panic!("no start for col {}", y)
}

fn find_start_down(y: usize, map: &Vec<Vec<Tile>>) -> (usize, usize) {
    for x in 0..map.len() {
        if y >= map[x].len() {
            continue;
        }
        if map[x][y] != Tile::NOTHING {
            return (x, y);
        }
    }
    panic!("no start for col {}", y)
}

#[derive(Debug)]
enum Instruction {
    MOVE(usize),
    RIGHT,
    LEFT,
}

#[derive(PartialEq)]
enum Tile {
    NOTHING,
    TILE,
    WALL,
}

#[derive(Eq, PartialEq, Debug, Hash, Clone, Copy)]
enum Direction {
    RIGHT,
    DOWN,
    LEFT,
    UP,
}

impl Direction {
    fn score(&self) -> usize {
        match self {
            Direction::RIGHT => 0,
            Direction::DOWN => 1,
            Direction::LEFT => 2,
            Direction::UP => 3,
        }
    }
}
