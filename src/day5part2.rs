extern crate intcode_computer;

use intcode_computer::IntcodeComputer;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file = File::open("./data/day5.txt")?;
    let mut reader = BufReader::new(file);
    let mut data = String::new();
    reader.read_line(&mut data)?;
    // Blindly delete trailing new line before parsing
    data.pop();

    let initial_mem = data
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut comp = IntcodeComputer::new();
    comp.mem = initial_mem.clone();

    match comp.run() {
        Ok(v) => println!("OK: {}", v),
        Err(e) => println!("ERR: {}", e),
    }

    Ok(())
}
