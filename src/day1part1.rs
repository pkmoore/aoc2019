use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file = File::open("./data/day1.txt")?;
    let mut reader = BufReader::new(file);
    let mut total: u64 = 0;
    let mut weight: u64;
    let mut data = String::new();
    while reader.read_line(&mut data)? > 0 {
        // Blindly delete trailing new line before parsing
        data.pop();
        weight = data.parse::<u64>().unwrap();
        total += (weight / 3) - 2;
        data.clear();
    }
    println!("Day 1 Part 1 Answer: {}", total);
    Ok(())
}
