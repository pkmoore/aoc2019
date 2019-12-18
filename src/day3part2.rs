use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

struct Wire {
    hs: HashSet<(i64, i64)>,
    v: Vec<(i64, i64)>,
}

fn main() -> std::io::Result<()> {
    let file = File::open("./data/day3.txt")?;
    let mut reader = BufReader::new(file);
    let mut wire1str = String::new();
    let mut wire2str = String::new();
    reader.read_line(&mut wire1str)?;
    reader.read_line(&mut wire2str)?;

    // Blindly chop off trailing new lines
    wire1str.pop();
    wire2str.pop();

    let wire1inst = wire1str.split(",").collect::<Vec<&str>>();
    let wire2inst = wire2str.split(",").collect::<Vec<&str>>();

    // Build coords in which wires exist
    let wire1 = populate_intersection_set(wire1inst).unwrap();
    let wire2 = populate_intersection_set(wire2inst).unwrap();

    // Sets are pretty good yo
    let shared_intersections = wire1.hs.intersection(&wire2.hs);

    println!(
        "Day 3 Part 2: Wire Distance: {}",
        shared_intersections
            .map(|x| {
                wire1
                    .v
                    .iter()
                    .position(|y| x.0 == y.0 && x.1 == y.1)
                    .unwrap()
                    + wire2
                        .v
                        .iter()
                        .position(|y| x.0 == y.0 && x.1 == y.1)
                        .unwrap()
            })
            .min()
            .unwrap()
            + 2
    );
    // We add two here  ^^ because the position counts from zero but wire length
    // starts counting from 1

    Ok(())
}

fn populate_intersection_set(wirestr: Vec<&str>) -> Result<Wire, &str> {
    let mut wire = Wire {
        hs: HashSet::new(),
        v: Vec::new(),
    };
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    for i in wirestr {
        let direction_distance = i.split_at(1);
        let int_distance = direction_distance.1.parse::<i64>().unwrap();
        match direction_distance.0 {
            "U" => {
                for _i in 0..int_distance {
                    y += 1;
                    wire.hs.insert((x, y));
                    wire.v.push((x, y));
                }
            }
            "D" => {
                for _i in 0..int_distance {
                    y -= 1;
                    wire.hs.insert((x, y));
                    wire.v.push((x, y));
                }
            }
            "L" => {
                for _i in 0..int_distance {
                    x -= 1;
                    wire.hs.insert((x, y));
                    wire.v.push((x, y));
                }
            }
            "R" => {
                for _i in 0..int_distance {
                    x += 1;
                    wire.hs.insert((x, y));
                    wire.v.push((x, y));
                }
            }
            _ => return Err("Unrecognized Direction"),
        }
    }
    Ok(wire)
}
