use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::error::Error;

use nom::{
  IResult,
  sequence::delimited,
  // see the "streaming/complete" paragraph lower for an explanation of these submodules
  character::complete::i64,
  bytes::complete::tag,
};

use itertools::Itertools;

fn planet_parser(input: &str) -> IResult<&str, Planet> {
    let (input, _) = tag("<x=")(input)?;
    let (input, x_coord) = i64(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, y_coord) = i64(input)?;
    let (input, _) = tag(", z=")(input)?;
    let (input, z_coord) = i64(input)?;
    let (input, _) = tag(">")(input)?;
    Ok((input, Planet {x_coord: x_coord,
                       y_coord: y_coord,
                       z_coord: z_coord,
                       x_vel: 0,
                       y_vel: 0,
                       z_vel: 0}))
}

#[derive(Debug)]
struct Planet {
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
}

fn update_velocities(p1: &mut Planet, p2: &mut Planet) {
    if p1.x_coord < p2.x_coord {
        p1.x_vel += 1;
        p2.x_coord -= 1;
    } else if p1.x_coord > p2.x_coord {
        p1.x_vel -= 1;
        p2.x_coord += 1;
    } else {
        // x_coords are equal, velocity unchanged
    }

    if p1.y_coord < p2.y_coord {
        p1.y_vel += 1;
        p2.y_coord -= 1;
    } else if p1.y_coord > p2.y_coord {
        p1.y_vel -= 1;
        p2.y_coord += 1;
    } else {
        // y_coords are equal, velocity unchanged
    }

    if p1.z_coord < p2.z_coord {
        p1.z_vel += 1;
        p2.z_coord -= 1;
    } else if p1.z_coord > p2.z_coord {
        p1.z_vel -= 1;
        p2.z_coord += 1;
    } else {
        // z_coords are equal, velocity unchanged
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

    for p in planets.iter_mut().permutations(2) {
        update_velocities(p[0], p[1])
    }

    Ok(())
}
