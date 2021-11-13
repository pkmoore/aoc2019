extern crate textplots;
extern crate intcode_computer;

use intcode_computer::IntcodeComputer;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use std::collections::HashMap;
use std::collections::HashSet;

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

enum Turn {
    LEFT,
    RIGHT
}

struct Robot {
    x_coord: i64,
    y_coord: i64,
    facing: Direction,
    painted_tiles: HashMap<(i64, i64), usize>
}

impl Robot {
    fn new() -> Robot {
        Robot {
            x_coord: 0,
            y_coord: 0,
            facing: Direction::UP,
            painted_tiles: HashMap::new()
        }
    }

    fn get_color_below(&self) -> usize {
        match self.painted_tiles.get(&(self.x_coord, self.y_coord)) {
            Some(t) => *t,
            None => 0
        }
    }

    fn paint_tile(&mut self, color: usize) {
        self.painted_tiles.insert((self.x_coord, self.y_coord), color);
    }

    fn turn(&mut self, direction: usize) {
        let turn = match direction {
            0 => Turn::LEFT,
            1 => Turn::RIGHT,
            _ => panic!("Bad direction: {}", direction)
        };

        match self.facing {
            Direction::UP => {
                match turn {
                    Turn::LEFT => {
                        self.facing = Direction::LEFT;
                        self.x_coord -= 1;
                    },
                    Turn::RIGHT => {
                        self.facing = Direction::RIGHT;
                        self.x_coord += 1;
                    }
                }
            },
            Direction::DOWN => {
                match turn {
                    Turn::LEFT => {
                        self.facing = Direction::RIGHT;
                        self.x_coord += 1;
                    },
                    Turn::RIGHT => {
                        self.facing = Direction::LEFT;
                        self.x_coord -= 1;
                    }
                }
            },
            Direction::LEFT => {
                match turn {
                    Turn::LEFT => {
                        self.facing = Direction::DOWN;
                        self.y_coord -= 1;
                    },
                    Turn::RIGHT => {
                        self.facing = Direction::UP;
                        self.y_coord += 1;
                    }
                }
            },
            Direction::RIGHT => {
                match turn {
                    Turn::LEFT => {
                        self.facing = Direction::UP;
                        self.y_coord += 1;
                    },
                    Turn::RIGHT => {
                        self.facing = Direction::DOWN;
                        self.y_coord -= 1;
                    }
                }
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("./data/day11.txt")?;
    let mut reader = BufReader::new(file);
    let mut data = String::new();
    reader.read_line(&mut data)?;
    // Blindly delete trailing new line before parsing
    data.pop();

    let one_hundred_meg = 100000000;

    let mut comp = IntcodeComputer::new();
    comp.mem = data.split(",").map(|x| x.parse::<i64>().unwrap()).collect();
    comp.mem.extend(vec![0; one_hundred_meg]);

    let mut rob: Robot = Robot::new();

    let mut unique_tiles_painted: HashSet<(i64, i64)> = HashSet::new();

    loop {
        let color_below = rob.get_color_below();
        comp.stdin.push(color_below as i64);
        comp.run_to_output();
        // We halted without output
        if comp.stdout.len() == 0 {
            break;
        }
        let color = comp.stdout.remove(0);
        rob.paint_tile(color as usize);
        unique_tiles_painted.insert((rob.x_coord, rob.y_coord));
        comp.run_to_output();
        if comp.stdout.len() == 0 {
            break;
        }
        let direction = comp.stdout.remove(0);
        rob.turn(direction as usize);
    }
    println!("Tiles painted: {}", unique_tiles_painted.len());

    Ok(())
}
