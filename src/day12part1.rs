use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use nom::{
    bytes::complete::tag,
    // see the "streaming/complete" paragraph lower for an explanation of these submodules
    character::complete::i64,
    IResult,
};

use std::sync::atomic::{AtomicUsize, Ordering};

static PLANET_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

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

impl Planet {
    fn from_line(line: &str) -> Planet {
        planet_parser(line).unwrap().1
    }

    fn update_velocity(&mut self, p2: &Planet) {
        println!("Updating planet {} with planet {}", self.id, p2.id);
        if self.x_coord < p2.x_coord {
            self.x_coord += 1;
        } else if self.x_coord > p2.x_coord {
            self.x_coord -= 1;
        } else {
            // coords are the same, no change necessary
        }
        if self.y_coord < p2.y_coord {
            self.y_coord += 1;
        } else if self.y_coord > p2.y_coord {
            self.y_coord -= 1;
        } else {
            // coords are the same, no change necessary
        }
        if self.z_coord < p2.z_coord {
            self.z_coord += 1;
        } else if self.z_coord > p2.z_coord {
            self.z_coord -= 1;
        } else {
            // coords are the same, no change necessary
        }
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
    for i in 0..planets.len() - 1 {
        for j in i..planets.len() {
            if i == j {
                continue;
            }
            // Update the first planet with the second...
            let mut p: Planet =  planets[i];
            p.update_velocity(&planets[j]);
            planets[i] = p;
            // ...and then the second planet with the first
            let mut p: Planet = planets[j];
            p.update_velocity(&planets[i]);
            planets[j] = p;
        }
    }

    Ok(())
}
