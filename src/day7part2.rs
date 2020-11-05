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

    let phases_iter = (5..10).permutations(num_amplifiers);
    let mut results: Vec<i64> = Vec::new();

    for phases in phases_iter {
        // Reset the computers
        for c in &mut comps {
            c.stdin.clear();
            c.stdout.clear();
            c.cur_index = 0;
            c.mem = initial_mem.clone();
        }
        // Give every computer their phase config
        for (i, p) in phases.iter().enumerate() {
            comps[i].stdin.push(*p as i64);
        }

        let mut last_run_output = 0;
        let mut current_comp = 0;
        loop {
            let c = &mut comps[current_comp];
            // pass each computer the previous run's output
            // This is 0 for initial iteration, computer 0
            c.stdin.push(last_run_output);
            // This function stops on output or terminate instruction
            match c.run_to_output() {
                Ok(_) => {
                    // If we don't have output, we got a terminate instruction
                    if c.stdout.len() == 0 {
                        break;
                    }
                    // We if we have output, we haven't terminated so we countinue
                    last_run_output = c.stdout.remove(0);
                    current_comp += 1;
                    if current_comp >= num_amplifiers {
                        current_comp = 0;
                    }
                    continue;
                }
                Err(e) => println!("ERR: {}", e),
            }
            c.stdin.clear();
            c.stdout.clear();
        }
        results.push(last_run_output);
    }
    println!("Largest: {}", results.iter().max().unwrap());

    Ok(())
}
