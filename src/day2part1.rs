use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file = File::open("./data/day2.txt")?;
    let mut reader = BufReader::new(file);
    let mut data = String::new();
    reader.read_line(&mut data)?;
    // Blindly delete trailing new line before parsing
    data.pop();
    let mut mem = data
        .split(",")
        .map(|m| m.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let mut cur_index: usize = 0;
    // "Fix" the program
    mem[1] = 12;
    mem[2] = 2;
    loop {
        let op = mem[cur_index];
        match op {
            1 => handle_add(&mut mem, cur_index),
            2 => handle_multiply(&mut mem, cur_index),
            99 => break,
            _ => handle_error(&mem, cur_index),
        }
        cur_index += 4;
    }
    handle_terminate(&mem, cur_index);
    Ok(())
}

fn handle_add(mem: &mut Vec<u64>, cur_index: usize) {
    let val1 = mem[mem[cur_index + 1] as usize];
    let val2 = mem[mem[cur_index + 2] as usize];
    let dest = mem[cur_index + 3] as usize;
    mem[dest] = val1 + val2;
}

fn handle_multiply(mem: &mut Vec<u64>, cur_index: usize) {
    let val1 = mem[mem[cur_index + 1] as usize];
    let val2 = mem[mem[cur_index + 2] as usize];
    let dest = mem[cur_index + 3] as usize;
    mem[dest] = val1 * val2;
}

fn handle_terminate(mem: &Vec<u64>, _cur_index: usize) {
    println!("Day 2 Part 1 position 0: {}", mem[0]);
}

fn handle_error(mem: &Vec<u64>, cur_index: usize) {
    println!(
        "Error: Got bad opcode {} at position {}",
        mem[cur_index], cur_index
    );
    println!("{:?}", mem);
}
