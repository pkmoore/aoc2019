use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use nom::{bytes::complete::tag, character::complete::i64, IResult};

use std::sync::atomic::{AtomicUsize, Ordering};
static PLANET_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

use std::collections::HashSet;
use std::hash::{Hash, Hasher};

fn planet_parser(input: &str) -> IResult<&str, Planet> {
    let (input, _) = tag("<x=")(input)?;
    let (input, x_coord) = i64(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, y_coord) = i64(input)?;
    let (input, _) = tag(", z=")(input)?;
    let (input, z_coord) = i64(input)?;
    let (input, _) = tag(">")(input)?;
    let result = Ok((
        input,
        Planet {
            id: PLANET_ID_COUNTER.fetch_add(1, Ordering::SeqCst),
            x_coord: x_coord,
            y_coord: y_coord,
            z_coord: z_coord,
            x_vel: 0,
            y_vel: 0,
            z_vel: 0,
        },
    ));
    result
}

#[derive(Debug, Clone, Copy)]
struct Planet {
    id: usize,
    x_coord: i64,
    y_coord: i64,
    z_coord: i64,
    x_vel: i64,
    y_vel: i64,
    z_vel: i64,
}

impl Hash for Planet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x_coord.hash(state);
        self.y_coord.hash(state);
        self.z_coord.hash(state);
        self.x_vel.hash(state);
        self.y_vel.hash(state);
        self.z_vel.hash(state);
    }
}

impl PartialEq for Planet {
    fn eq(&self, other: &Self) -> bool {
        self.x_coord == other.x_coord &&
        self.y_coord == other.y_coord &&
        self.z_coord == other.z_coord &&
        self.x_vel == other.x_vel &&
        self.y_vel == other.y_vel &&
        self.z_vel == other.z_vel
    }
}

impl Eq for Planet {}

impl Planet {
    fn from_line(line: &str) -> Planet {
        planet_parser(line).unwrap().1
    }

    fn update_velocity(&mut self, p2: &Planet) {
        if self.x_coord < p2.x_coord {
            self.x_vel += 1;
        } else if self.x_coord > p2.x_coord {
            self.x_vel -= 1;
        } else {
            // coords are the same, no change necessary
        }
        if self.y_coord < p2.y_coord {
            self.y_vel += 1;
        } else if self.y_coord > p2.y_coord {
            self.y_vel -= 1;
        } else {
            // coords are the same, no change necessary
        }
        if self.z_coord < p2.z_coord {
            self.z_vel += 1;
        } else if self.z_coord > p2.z_coord {
            self.z_vel -= 1;
        } else {
            // coords are the same, no change necessary
        }
    }

    fn update_position(&mut self) {
        self.x_coord += self.x_vel;
        self.y_coord += self.y_vel;
        self.z_coord += self.z_vel;
    }

    fn potential_energy(&self) -> i64 {
        self.x_coord.abs() + self.y_coord.abs() + self.z_coord.abs()
    }

    fn kinetic_energy(&self) -> i64 {
        self.x_vel.abs() + self.y_vel.abs() + self.z_vel.abs()
    }

    fn total_energy(&self) -> i64 {
        self.potential_energy() * self.kinetic_energy()
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("./data/day12.txt")?;
    let mut reader = BufReader::new(file);
    let mut data = String::new();

    let mut planets: Vec<Planet> = std::vec::Vec::new();

    while reader.read_line(&mut data)? > 0 {
        data.pop();
        planets.push(Planet::from_line(data.as_str()));
        data.clear();
    }

    // For each planet, loop through and update its velocity with the remaining planets
    let mut states: HashSet<Vec<Planet>> = HashSet::new();
    loop {
        for i in 0..planets.len() - 1 {
            for j in i..planets.len() {
                if i == j {
                    continue;
                }

                // Update the first planet with the second...
                let mut p1: Planet = planets[i];
                let mut p2: Planet = planets[j];
                p1.update_velocity(&planets[j]);
                p2.update_velocity(&planets[i]);
                planets[i] = p1;
                planets[j] = p2;
            }
        }
        for i in planets.iter_mut() {
            i.update_position();
        }
        if !states.insert(planets.clone()) {
            break;
        }
    }
    println!("Steps: {}", states.len());

    Ok(())
}