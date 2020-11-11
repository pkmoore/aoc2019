extern crate intcode_computer;

use intcode_computer::IntcodeComputer;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file = File::open("./data/day9.txt")?;
    let mut reader = BufReader::new(file);
    let mut data = String::new();
    reader.read_line(&mut data)?;
    // Blindly delete trailing new line before parsing
    data.pop();

    let one_hundred_meg = 100000000;

    let mut comp = IntcodeComputer::new();
    comp.mem = data.split(",").map(|x| x.parse::<i64>().unwrap()).collect();
    comp.mem.extend(vec![0; one_hundred_meg]);

    // All we have to do is run the program with 2 as the input
    // and hope we're fast enough!
    comp.stdin.push(2);

    println!("Result: {}", comp.run().unwrap());

    Ok(())
}
