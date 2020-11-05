extern crate intcode_computer;

use intcode_computer::IntcodeComputer;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use itertools::Itertools;

fn main() -> std::io::Result<()> {
    let file = File::open("./data/day7.txt")?;
    let mut reader = BufReader::new(file);
    let mut data = String::new();
    reader.read_line(&mut data)?;
    // Blindly delete trailing new line before parsing
    data.pop();

    let initial_mem = data
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let num_amplifiers = 5;

    let mut comps: Vec<IntcodeComputer> = Vec::new();
    for _ in 0..num_amplifiers {
        let comp = IntcodeComputer::new();
        comps.push(comp);
    }

    let phases_iter = (0..num_amplifiers).permutations(num_amplifiers);
    let mut results: Vec<i64> = Vec::new();
    for mut phases in phases_iter {
        let mut last_run_output = 0;
        for c in &mut comps {
            c.cur_index = 0;
            c.stdin.clear();
            c.stdout.clear();
            c.mem = initial_mem.clone();
            // Load up the computers stdin with the phase
            // followed by the output from the last run (or 0 for the first one)
            c.stdin.push(phases.remove(0) as i64);
            c.stdin.push(last_run_output);
            match c.run() {
                Ok(v) => {
                    println!("OK: {}", v);
                    last_run_output = c.stdout.remove(0);
                }
                Err(e) => println!("ERR: {}", e),
            }
        }
        results.push(last_run_output);
    }
    println!("Largest: {}", results.iter().max().unwrap());

    Ok(())
}
