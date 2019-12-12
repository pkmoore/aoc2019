use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file = File::open("./data/day1.txt")?;
    let mut reader = BufReader::new(file);
    let mut total_fuel_for_modules: i64 = 0;
    let mut total_fuel_for_fuel: i64 = 0;
    let mut data = String::new();
    while reader.read_line(&mut data)? > 0 {
        // Blindly delete trailing new line before parsing
        data.pop();
        let weight = data.parse::<i64>().unwrap();
        let fuel_for_module = (weight / 3) - 2;
        total_fuel_for_modules += fuel_for_module;
        total_fuel_for_fuel += fuel_for_fuel(fuel_for_module);
        data.clear();
    }

    println!(
        "Day 1 Part 2 total_fuel_for_modules: {}",
        total_fuel_for_modules
    );
    println!("Day 1 Part 2 total_fuel_for_fuel: {}", total_fuel_for_fuel);
    println!(
        "Day 1 Part 2 grand_total: {}",
        total_fuel_for_modules + total_fuel_for_fuel
    );
    Ok(())
}

fn fuel_for_fuel(fuel: i64) -> i64 {
    let mut total = 0;
    let mut working_fuel: i64 = (fuel / 3) - 2;
    while working_fuel > 0 {
        total += working_fuel;
        working_fuel = (working_fuel / 3) - 2;
    }
    return total;
}
