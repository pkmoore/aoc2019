use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

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
    let wire1intersections = populate_intersection_set(wire1inst).unwrap();
    let wire2intersections = populate_intersection_set(wire2inst).unwrap();

    // Sets are pretty good yo
    let shared_intersections = wire1intersections.intersection(&wire2intersections);
    // Manhattan distance uses absolute value
    println!(
        "Day 3 Part 1 Closest Shared Intersection: {}",
        shared_intersections
            .map(|x| x.0.abs() + x.1.abs())
            .min()
            .unwrap()
    );
    Ok(())
}

fn populate_intersection_set(wire: Vec<&str>) -> Result<HashSet<(i64, i64)>, &str> {
    let mut set = HashSet::new();
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    for i in wire {
        let direction_distance = i.split_at(1);
        let int_distance = direction_distance.1.parse::<i64>().unwrap();
        match direction_distance.0 {
            "U" => {
                for _i in 0..int_distance {
                    y += 1;
                    set.insert((x, y));
                }
            }
            "D" => {
                for _i in 0..int_distance {
                    y -= 1;
                    set.insert((x, y));
                }
            }
            "L" => {
                for _i in 0..int_distance {
                    x -= 1;
                    set.insert((x, y));
                }
            }
            "R" => {
                for _i in 0..int_distance {
                    x += 1;
                    set.insert((x, y));
                }
            }
            _ => return Err("Unrecognized Direction"),
        }
    }
    Ok(set)
}
